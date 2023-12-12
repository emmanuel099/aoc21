#[derive(Debug, PartialEq)]
struct Packet {
    header: Header,
    payload: Payload,
}

impl Packet {
    pub fn eval(&self) -> usize {
        use Operator::*;
        match &self.payload {
            Payload::Literal(value) => *value,
            Payload::Operator(Sum(operands)) => operands.iter().map(|p| p.eval()).sum(),
            Payload::Operator(Product(operands)) => operands.iter().map(|p| p.eval()).product(),
            Payload::Operator(Minimum(operands)) => {
                operands.iter().map(|p| p.eval()).min().unwrap()
            }
            Payload::Operator(Maximum(operands)) => {
                operands.iter().map(|p| p.eval()).max().unwrap()
            }
            Payload::Operator(GreaterThan { left, right }) => (left.eval() > right.eval()) as usize,
            Payload::Operator(LessThan { left, right }) => (left.eval() < right.eval()) as usize,
            Payload::Operator(EqualTo { left, right }) => (left.eval() == right.eval()) as usize,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan {
        left: Box<Packet>,
        right: Box<Packet>,
    },
    LessThan {
        left: Box<Packet>,
        right: Box<Packet>,
    },
    EqualTo {
        left: Box<Packet>,
        right: Box<Packet>,
    },
}

#[derive(Debug, PartialEq)]
enum Payload {
    Literal(usize),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
struct Header {
    version: usize,
    type_id: usize,
}

type BitSlice<'a> = &'a [bool];

fn read_bits(bits: BitSlice, n: usize) -> Option<(BitSlice, usize)> {
    if bits.len() < n {
        return None;
    }

    let value = bits[..n]
        .iter()
        .enumerate()
        .fold(0, |value, (i, &bit)| value | (bit as usize) << (n - i - 1));
    Some((&bits[n..], value))
}

fn parse_header(bits: BitSlice) -> Option<(BitSlice, Header)> {
    let (bits, version) = read_bits(bits, 3)?;
    let (bits, type_id) = read_bits(bits, 3)?;
    Some((bits, Header { version, type_id }))
}

fn parse_literal(bits: BitSlice) -> Option<(BitSlice, usize)> {
    let mut next_bits = bits;
    let mut value = 0;
    loop {
        let (bits, prefix) = read_bits(next_bits, 1)?;
        let (bits, group) = read_bits(bits, 4)?;

        value = value << 4 | group;
        next_bits = bits;

        if prefix == 0 {
            break Some((next_bits, value));
        }
    }
}

fn parse_operands(bits: BitSlice) -> Option<(BitSlice, Vec<Packet>)> {
    let (bits, length_type_id) = read_bits(bits, 1)?;
    match length_type_id {
        0 => {
            let (bits, byte_length_of_packets) = read_bits(bits, 15)?;
            let packets = read_packets_until_end(&bits[0..byte_length_of_packets]);
            Some((&bits[byte_length_of_packets..], packets))
        }
        1 => {
            let (bits, number_of_packets) = read_bits(bits, 11)?;
            read_packets_exactly(bits, number_of_packets)
        }
        _ => unreachable!(),
    }
}

fn read_packets_until_end(bits: BitSlice) -> Vec<Packet> {
    let mut next_bits = bits;
    let mut packets = Vec::new();
    while let Some((bits, packet)) = parse_packet(next_bits) {
        packets.push(packet);
        next_bits = bits;
    }
    packets
}

fn read_packets_exactly(bits: BitSlice, n: usize) -> Option<(BitSlice, Vec<Packet>)> {
    let mut next_bits = bits;
    let mut packets = Vec::with_capacity(n);
    for _ in 0..n {
        let (bits, packet) = parse_packet(next_bits)?;
        packets.push(packet);
        next_bits = bits;
    }
    Some((next_bits, packets))
}

fn parse_packet(bits: BitSlice) -> Option<(BitSlice, Packet)> {
    let (bits, header) = parse_header(bits)?;
    let (bits, payload) = match header {
        Header { type_id: 4, .. } => {
            let (bits, value) = parse_literal(bits)?;
            (bits, Payload::Literal(value))
        }
        Header {
            type_id: op @ (0 | 1 | 2 | 3),
            ..
        } => {
            let (bits, operands) = parse_operands(bits)?;
            let operator = match op {
                0 => Operator::Sum(operands),
                1 => Operator::Product(operands),
                2 => Operator::Minimum(operands),
                3 => Operator::Maximum(operands),
                _ => unreachable!(),
            };
            (bits, Payload::Operator(operator))
        }
        Header {
            type_id: op @ (5 | 6 | 7),
            ..
        } => {
            let (bits, mut operands) = parse_operands(bits)?;
            if operands.len() != 2 {
                panic!("Invalid operator, expected 2 operands");
            }
            let right = Box::new(operands.pop()?);
            let left = Box::new(operands.pop()?);
            let operator = match op {
                5 => Operator::GreaterThan { left, right },
                6 => Operator::LessThan { left, right },
                7 => Operator::EqualTo { left, right },
                _ => unreachable!(),
            };
            (bits, Payload::Operator(operator))
        }
        _ => panic!("Malformed packet"),
    };
    Some((bits, Packet { header, payload }))
}

fn hex_string_to_bits(s: &str) -> Vec<bool> {
    s.chars()
        .flat_map(|c| match c {
            '0' => vec![false, false, false, false],
            '1' => vec![false, false, false, true],
            '2' => vec![false, false, true, false],
            '3' => vec![false, false, true, true],
            '4' => vec![false, true, false, false],
            '5' => vec![false, true, false, true],
            '6' => vec![false, true, true, false],
            '7' => vec![false, true, true, true],
            '8' => vec![true, false, false, false],
            '9' => vec![true, false, false, true],
            'A' => vec![true, false, true, false],
            'B' => vec![true, false, true, true],
            'C' => vec![true, true, false, false],
            'D' => vec![true, true, false, true],
            'E' => vec![true, true, true, false],
            'F' => vec![true, true, true, true],
            _ => panic!("Unexpected hex char"),
        })
        .collect()
}

fn decode_transmission(transmission: &str) -> Option<Packet> {
    let bits = hex_string_to_bits(transmission);
    parse_packet(&bits).map(|(_, packet)| packet)
}

fn sum_of_packet_version(packet: &Packet) -> usize {
    let sub_packet_version_sum = match &packet.payload {
        Payload::Literal(..) => 0,
        Payload::Operator(
            Operator::Sum(ops)
            | Operator::Product(ops)
            | Operator::Minimum(ops)
            | Operator::Maximum(ops),
        ) => ops.iter().map(sum_of_packet_version).sum(),
        Payload::Operator(
            Operator::GreaterThan { left, right }
            | Operator::LessThan { left, right }
            | Operator::EqualTo { left, right },
        ) => sum_of_packet_version(left) + sum_of_packet_version(right),
    };
    packet.header.version + sub_packet_version_sum
}

const INSTANCE:&str = "220D700071F39F9C6BC92D4A6713C737B3E98783004AC0169B4B99F93CFC31AC4D8A4BB89E9D654D216B80131DC0050B20043E27C1F83240086C468A311CC0188DB0BA12B00719221D3F7AF776DC5DE635094A7D2370082795A52911791ECB7EDA9CFD634BDED14030047C01498EE203931BF7256189A593005E116802D34673999A3A805126EB2B5BEEBB823CB561E9F2165492CE00E6918C011926CA005465B0BB2D85D700B675DA72DD7E9DBE377D62B27698F0D4BAD100735276B4B93C0FF002FF359F3BCFF0DC802ACC002CE3546B92FCB7590C380210523E180233FD21D0040001098ED076108002110960D45F988EB14D9D9802F232A32E802F2FDBEBA7D3B3B7FB06320132B0037700043224C5D8F2000844558C704A6FEAA800D2CFE27B921CA872003A90C6214D62DA8AA9009CF600B8803B10E144741006A1C47F85D29DCF7C9C40132680213037284B3D488640A1008A314BC3D86D9AB6492637D331003E79300012F9BDE8560F1009B32B09EC7FC0151006A0EC6082A0008744287511CC0269810987789132AC600BD802C00087C1D88D05C001088BF1BE284D298005FB1366B353798689D8A84D5194C017D005647181A931895D588E7736C6A5008200F0B802909F97B35897CFCBD9AC4A26DD880259A0037E49861F4E4349A6005CFAD180333E95281338A930EA400824981CC8A2804523AA6F5B3691CF5425B05B3D9AF8DD400F9EDA1100789800D2CBD30E32F4C3ACF52F9FF64326009D802733197392438BF22C52D5AD2D8524034E800C8B202F604008602A6CC00940256C008A9601FF8400D100240062F50038400970034003CE600C70C00F600760C00B98C563FB37CE4BD1BFA769839802F400F8C9CA79429B96E0A93FAE4A5F32201428401A8F508A1B0002131723B43400043618C2089E40143CBA748B3CE01C893C8904F4E1B2D300527AB63DA0091253929E42A53929E420";

fn main() {
    let packet = decode_transmission(INSTANCE).unwrap();
    println!("Part 1: {}", sum_of_packet_version(&packet));
    println!("Part 2: {}", packet.eval());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_decode_literal_packet() {
        assert_eq!(
            decode_transmission("D2FE28"),
            Some(Packet {
                header: Header {
                    version: 6,
                    type_id: 4,
                },
                payload: Payload::Literal(2021)
            })
        )
    }

    #[test]
    fn test_decode_operator_packet() {
        assert_eq!(
            decode_transmission("38006F45291200"),
            Some(Packet {
                header: Header {
                    version: 1,
                    type_id: 6,
                },
                payload: Payload::Operator(Operator::LessThan {
                    left: Box::new(Packet {
                        header: Header {
                            version: 6,
                            type_id: 4,
                        },
                        payload: Payload::Literal(10)
                    }),
                    right: Box::new(Packet {
                        header: Header {
                            version: 2,
                            type_id: 4,
                        },
                        payload: Payload::Literal(20)
                    })
                })
            })
        )
    }

    #[rstest]
    #[case("8A004A801A8002F478", 16)]
    #[case("620080001611562C8802118E34", 12)]
    #[case("C0015000016115A2E0802F182340", 23)]
    #[case("A0016C880162017C3686B18A3D4780", 31)]
    fn test_sum_of_packet_version(#[case] transmission: &str, #[case] expected_sum: usize) {
        let packet = decode_transmission(&transmission).unwrap();
        dbg!(&packet);
        assert_eq!(sum_of_packet_version(&packet), expected_sum);
    }

    #[rstest]
    #[case("C200B40A82", 3)]
    #[case("04005AC33890", 54)]
    #[case("880086C3E88112", 7)]
    #[case("CE00C43D881120", 9)]
    #[case("D8005AC2A8F0", 1)]
    #[case("F600BC2D8F", 0)]
    #[case("9C005AC2F8F0", 0)]
    #[case("9C0141080250320F1802104A08", 1)]
    fn test_eval(#[case] transmission: &str, #[case] expected_result: usize) {
        let packet = decode_transmission(&transmission).unwrap();
        dbg!(&packet);
        assert_eq!(packet.eval(), expected_result);
    }
}
