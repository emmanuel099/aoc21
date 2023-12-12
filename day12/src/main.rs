use std::io::{self, BufRead};

fn main() {
    let graph = io::stdin().lock().lines().map(|line| line.unwrap()).fold(
        Graph::default(),
        |mut graph, line| {
            let (from, to) = line.split_once('-').unwrap();
            graph.insert_edge_undirected(from, to);
            graph
        },
    );

    println!(
        "Part 1: {}",
        graph.all_paths("start", "end", false).unwrap().len()
    );

    println!(
        "Part 2: {}",
        graph.all_paths("start", "end", true).unwrap().len()
    );
}

#[derive(Debug)]
struct Node {
    label: String,
    visit_once: bool,
}

type Path<'a> = Vec<&'a str>;

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
}

#[derive(Debug, Default)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn insert_edge_directed(&mut self, from: &str, to: &str) {
        let from = self.find_or_insert_node(from);
        let to = self.find_or_insert_node(to);
        self.edges.push(Edge { from, to });
    }

    pub fn insert_edge_undirected(&mut self, from: &str, to: &str) {
        self.insert_edge_directed(from, to);
        self.insert_edge_directed(to, from);
    }

    fn find_or_insert_node(&mut self, label: &str) -> usize {
        if let Some(index) = self.find_node(label) {
            index
        } else {
            self.insert_node(label)
        }
    }

    fn insert_node<S: Into<String>>(&mut self, label: S) -> usize {
        let label = label.into();
        let visit_once = label.chars().all(|c| c.is_lowercase());
        let index = self.nodes.len();
        self.nodes.push(Node { label, visit_once });
        index
    }

    fn find_node(&self, label: &str) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_, node)| node.label == label)
            .map(|(i, _)| i)
    }

    fn all_successors(&self) -> Vec<Vec<usize>> {
        let mut succ = vec![Vec::new(); self.nodes.len()];
        for edge in &self.edges {
            succ[edge.from].push(edge.to);
        }
        succ
    }

    pub fn all_paths<'graph>(
        &'graph self,
        start: &str,
        end: &str,
        allow_one_small_cave_twice: bool,
    ) -> Result<Vec<Path<'graph>>, &'static str> {
        let start = self.find_node(start).ok_or("Start node not found")?;
        let end = self.find_node(end).ok_or("End node not found")?;

        let all_succ = self.all_successors();

        fn all_paths_rec<'graph>(
            nodes: &'graph [Node],
            all_succ: &[Vec<usize>],
            current: usize,
            start: usize,
            end: usize,
            allow_one_small_cave_twice: bool,
            node_count: &mut [usize],
            path: &mut Vec<usize>,
            paths: &mut Vec<Path<'graph>>,
        ) {
            if current == end {
                path.push(current);
                let labeled_path = path.iter().map(|&i| nodes[i].label.as_ref()).collect();
                path.pop();
                paths.push(labeled_path);
                return;
            }

            if nodes[current].visit_once && node_count[current] > 0 {
                if !allow_one_small_cave_twice || current == start {
                    return;
                }

                let any_small_cave_visited_twice = node_count
                    .iter()
                    .enumerate()
                    .any(|(i, c)| nodes[i].visit_once && *c > 1);
                if any_small_cave_visited_twice {
                    return;
                }
            }

            path.push(current);
            node_count[current] += 1;

            for &succ in &all_succ[current] {
                all_paths_rec(
                    nodes,
                    all_succ,
                    succ,
                    start,
                    end,
                    allow_one_small_cave_twice,
                    node_count,
                    path,
                    paths,
                );
            }

            path.pop();
            node_count[current] -= 1;
        }

        let mut paths = Vec::with_capacity(100);
        let mut node_count = vec![0; self.nodes.len()];
        let mut path = Vec::with_capacity(100);

        all_paths_rec(
            &self.nodes,
            &all_succ,
            start,
            start,
            end,
            allow_one_small_cave_twice,
            &mut node_count,
            &mut path,
            &mut paths,
        );

        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_graph_all_path_search() {
        let mut graph = Graph::default();
        graph.insert_edge_undirected("start", "A");
        graph.insert_edge_undirected("start", "b");
        graph.insert_edge_undirected("A", "c");
        graph.insert_edge_undirected("A", "b");
        graph.insert_edge_undirected("b", "d");
        graph.insert_edge_undirected("A", "end");
        graph.insert_edge_undirected("b", "end");

        let paths: HashSet<_> = graph
            .all_paths("start", "end", false)
            .unwrap()
            .into_iter()
            .map(|path| path.join(","))
            .collect();

        let expected_paths: HashSet<_> = vec![
            "start,A,b,A,c,A,end",
            "start,A,b,A,end",
            "start,A,b,end",
            "start,A,c,A,b,A,end",
            "start,A,c,A,b,end",
            "start,A,c,A,end",
            "start,A,end",
            "start,b,A,c,A,end",
            "start,b,A,end",
            "start,b,end",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(paths, expected_paths);
    }
}
