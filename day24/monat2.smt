(declare-const z0 Int)
(assert (= 0 z0))
; Iteration 1
(declare-const w1 Int)
(assert (< 0 w1 10))
(declare-const x1 Int)
(declare-const y1 Int)
(declare-const z1 Int)
; let y1 = z0 / 1
(assert (= y1 (div z0 1)))
; let x1 = z0 % 26 + 12
(assert (= x1 (+ (mod z0 26) 12)))
; let z1 = if x1 == w1 { y1 } else { 26 * y1 + w1 + 7 }
(assert (= z1 (ite (= x1 w1) y1 (+ (* 26 y1) w1 7))))
; Iteration 2
(declare-const w2 Int)
(assert (< 0 w2 10))
(declare-const x2 Int)
(declare-const y2 Int)
(declare-const z2 Int)
; let y2 = z1 / 1
(assert (= y2 (div z1 1)))
; let x2 = z1 % 26 + 13
(assert (= x2 (+ (mod z1 26) 13)))
; let z2 = if x2 == w2 { y2 } else { 26 * y2 + w2 + 8 }
(assert (= z2 (ite (= x2 w2) y2 (+ (* 26 y2) w2 8))))
; Iteration 3
(declare-const w3 Int)
(assert (< 0 w3 10))
(declare-const x3 Int)
(declare-const y3 Int)
(declare-const z3 Int)
; let y3 = z2 / 1
(assert (= y3 (div z2 1)))
; let x3 = z2 % 26 + 13
(assert (= x3 (+ (mod z2 26) 13)))
; let z3 = if x3 == w3 { y3 } else { 26 * y3 + w3 + 10 }
(assert (= z3 (ite (= x3 w3) y3 (+ (* 26 y3) w3 10))))
; Iteration 4
(declare-const w4 Int)
(assert (< 0 w4 10))
(declare-const x4 Int)
(declare-const y4 Int)
(declare-const z4 Int)
; let y4 = z3 / 26
(assert (= y4 (div z3 26)))
; let x4 = z3 % 26 + -2
(assert (= x4 (+ (mod z3 26) -2)))
; let z4 = if x4 == w4 { y4 } else { 26 * y4 + w4 + 4 }
(assert (= z4 (ite (= x4 w4) y4 (+ (* 26 y4) w4 4))))
; Iteration 5
(declare-const w5 Int)
(assert (< 0 w5 10))
(declare-const x5 Int)
(declare-const y5 Int)
(declare-const z5 Int)
; let y5 = z4 / 26
(assert (= y5 (div z4 26)))
; let x5 = z4 % 26 + -10
(assert (= x5 (+ (mod z4 26) -10)))
; let z5 = if x5 == w5 { y5 } else { 26 * y5 + w5 + 4 }
(assert (= z5 (ite (= x5 w5) y5 (+ (* 26 y5) w5 4))))
; Iteration 6
(declare-const w6 Int)
(assert (< 0 w6 10))
(declare-const x6 Int)
(declare-const y6 Int)
(declare-const z6 Int)
; let y6 = z5 / 1
(assert (= y6 (div z5 1)))
; let x6 = z5 % 26 + 13
(assert (= x6 (+ (mod z5 26) 13)))
; let z6 = if x6 == w6 { y6 } else { 26 * y6 + w6 + 6 }
(assert (= z6 (ite (= x6 w6) y6 (+ (* 26 y6) w6 6))))
; Iteration 7
(declare-const w7 Int)
(assert (< 0 w7 10))
(declare-const x7 Int)
(declare-const y7 Int)
(declare-const z7 Int)
; let y7 = z6 / 26
(assert (= y7 (div z6 26)))
; let x7 = z6 % 26 + -14
(assert (= x7 (+ (mod z6 26) -14)))
; let z7 = if x7 == w7 { y7 } else { 26 * y7 + w7 + 11 }
(assert (= z7 (ite (= x7 w7) y7 (+ (* 26 y7) w7 11))))
; Iteration 8
(declare-const w8 Int)
(assert (< 0 w8 10))
(declare-const x8 Int)
(declare-const y8 Int)
(declare-const z8 Int)
; let y8 = z7 / 26
(assert (= y8 (div z7 26)))
; let x8 = z7 % 26 + -5
(assert (= x8 (+ (mod z7 26) -5)))
; let z8 = if x8 == w8 { y8 } else { 26 * y8 + w8 + 13 }
(assert (= z8 (ite (= x8 w8) y8 (+ (* 26 y8) w8 13))))
; Iteration 9
(declare-const w9 Int)
(assert (< 0 w9 10))
(declare-const x9 Int)
(declare-const y9 Int)
(declare-const z9 Int)
; let y9 = z8 / 1
(assert (= y9 (div z8 1)))
; let x9 = z8 % 26 + 15
(assert (= x9 (+ (mod z8 26) 15)))
; let z9 = if x9 == w9 { y9 } else { 26 * y9 + w9 + 1 }
(assert (= z9 (ite (= x9 w9) y9 (+ (* 26 y9) w9 1))))
; Iteration 10
(declare-const w10 Int)
(assert (< 0 w10 10))
(declare-const x10 Int)
(declare-const y10 Int)
(declare-const z10 Int)
; let y10 = z9 / 1
(assert (= y10 (div z9 1)))
; let x10 = z9 % 26 + 15
(assert (= x10 (+ (mod z9 26) 15)))
; let z10 = if x10 == w10 { y10 } else { 26 * y10 + w10 + 8 }
(assert (= z10 (ite (= x10 w10) y10 (+ (* 26 y10) w10 8))))
; Iteration 11
(declare-const w11 Int)
(assert (< 0 w11 10))
(declare-const x11 Int)
(declare-const y11 Int)
(declare-const z11 Int)
; let y11 = z10 / 26
(assert (= y11 (div z10 26)))
; let x11 = z10 % 26 + -14
(assert (= x11 (+ (mod z10 26) -14)))
; let z11 = if x11 == w11 { y11 } else { 26 * y11 + w11 + 4 }
(assert (= z11 (ite (= x11 w11) y11 (+ (* 26 y11) w11 4))))
; Iteration 12
(declare-const w12 Int)
(assert (< 0 w12 10))
(declare-const x12 Int)
(declare-const y12 Int)
(declare-const z12 Int)
; let y12 = z11 / 1
(assert (= y12 (div z11 1)))
; let x12 = z11 % 26 + 10
(assert (= x12 (+ (mod z11 26) 10)))
; let z12 = if x12 == w12 { y12 } else { 26 * y12 + w12 + 13 }
(assert (= z12 (ite (= x12 w12) y12 (+ (* 26 y12) w12 13))))
; Iteration 13
(declare-const w13 Int)
(assert (< 0 w13 10))
(declare-const x13 Int)
(declare-const y13 Int)
(declare-const z13 Int)
; let y13 = z12 / 26
(assert (= y13 (div z12 26)))
; let x13 = z12 % 26 + -14
(assert (= x13 (+ (mod z12 26) -14)))
; let z13 = if x13 == w13 { y13 } else { 26 * y13 + w13 + 4 }
(assert (= z13 (ite (= x13 w13) y13 (+ (* 26 y13) w13 4))))
; Iteration 14
(declare-const w14 Int)
(assert (< 0 w14 10))
(declare-const x14 Int)
(declare-const y14 Int)
(declare-const z14 Int)
; let y14 = z13 / 26
(assert (= y14 (div z13 26)))
; let x14 = z13 % 26 + -5
(assert (= x14 (+ (mod z13 26) -5)))
; let z14 = if x14 == w14 { y14 } else { 26 * y14 + w14 + 14 }
(assert (= z14 (ite (= x14 w14) y14 (+ (* 26 y14) w14 14))))
(declare-const model_number Int)
(assert (= model_number (+ (* (+ (* (+ (* (+ (* (+ (* (+ (* (+ (* (+ (* (+ (* (+ (* (+ (* (+ (* (+ (* w1 10) w2) 10) w3) 10) w4) 10) w5) 10) w6) 10) w7) 10) w8) 10) w9) 10) w10) 10) w11) 10) w12) 10) w13) 10) w14)))
(assert (= z14 0))

(assert (> w10 6))
(assert (= w11 (- w10 6)))
(assert (< w12 3))
;(assert (> w13 1))
;(assert (= w14 (- w13 1)))

(push)
(maximize model_number)
(check-sat)
(get-value (z14 model_number w14 w13 z13 y14 x14 w4))
(pop)
(push)
(minimize model_number)
(check-sat)
(get-value (z14 model_number w14 w13 z13 y14 x14 w4))
(pop)
