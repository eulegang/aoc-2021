use nom::{
    IResult,
    multi::many1,
    character::complete::{newline, char, alpha1},
    sequence::{terminated, pair},
};

use std::collections::VecDeque;

fn main() {
    let graph: Graph = utils::input();

    println!("path count: {}", graph.paths().count());
    println!("multi path count: {}", graph.multi_paths().count());
}

pub struct Path {
    comps: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Graph {
    edge: Vec<String>,
    verts: Vec<Vec<usize>>,
}

impl Graph {
    fn path_repr(&self, path: &Path) -> String {
        let mut s = String::new();

        if path.comps.len() == 0 {
            return s;
        }

        s.push_str(&self[path.comps[0]]);

        for comp in &path.comps[1..] {
            s.push_str(" -> ");
            s.push_str(&self[*comp]);
        }

        s
    }

    fn is_reenterent(&self, node: usize) -> bool {
        self.edge[node].chars().nth(0).map_or(false, |ch| ch.is_uppercase())
    }

    fn paths(&self) -> PathGen {
        let graph = self;
        let mut states = VecDeque::with_capacity(128);
        states.push_back(vec![0]);

        PathGen { graph, states }
    }

    fn multi_paths(&self) -> MultiSmallPathGen {
        let graph = self;
        let mut states = VecDeque::with_capacity(128);
        states.push_back(vec![0]);

        MultiSmallPathGen { graph, states }
    }
}

pub struct PathGen<'a> {
    graph: &'a Graph,
    states: VecDeque<Vec<usize>>,
}

impl<'a> Iterator for PathGen<'a> {
    type Item = Path;

    fn next(&mut self) -> Option<Path> {
        while let Some(state) = self.states.pop_front() {
            if state[state.len() - 1] == 1 {
                return Some(Path { comps: state });
            }

            let cur = state[state.len() - 1];
            let nexts = &self.graph.verts[cur];

            for next in nexts {
                if self.graph.is_reenterent(*next) || !state.contains(next)  {
                    let mut x = state.clone();
                    x.push(*next);
                    self.states.push_back(x);
                }
            }
        }

        None
    }
}

pub struct MultiSmallPathGen<'a> {
    graph: &'a Graph,
    states: VecDeque<Vec<usize>>,
}

impl<'a> Iterator for MultiSmallPathGen<'a> {
    type Item = Path;

    fn next(&mut self) -> Option<Path> {
        fn reenter(graph: &Graph, state: &[usize]) -> bool {
            let mut counts = vec![0; graph.edge.len()];

            for s in state {
                counts[*s] += 1;
            }

            for i in 0..counts.len() {
                if counts[i] > 1 && !graph.is_reenterent(i) {
                    return false;
                }
            }

            true
        }


        while let Some(state) = self.states.pop_front() {
            if state[state.len() - 1] == 1 {
                return Some(Path { comps: state });
            }

            let cur = state[state.len() - 1];
            let nexts = &self.graph.verts[cur];

            for next in nexts {
                if *next == 0 {
                    continue;
                }

                if self.graph.is_reenterent(*next) || !state.contains(next) || reenter(self.graph, &state) {
                    let mut x = state.clone();
                    x.push(*next);
                    self.states.push_back(x);
                }
            }
        }

        None
    }
}

impl std::ops::Index<usize> for Graph {
    type Output = str;

    fn index(&self, index: usize) -> &str {
        &self.edge[index]
    }
}

impl utils::Parsable for Graph {
    fn parse(input: &str) -> IResult<&str, Graph> {
        let (input, parsed) = many1(pair(terminated(alpha1, char('-')), terminated(alpha1, newline)))(input)?;


        let mut edge = vec!["start".to_string(), "end".to_string()];
        let mut verts = vec![vec![], vec![]];

        for (src, dst) in &parsed {
            for marker in [src, dst] {
                if *marker == "start" || *marker == "end" {
                    continue;
                }

                let elem = String::from(*marker);

                if !edge.contains(&elem) {
                    edge.push(marker.to_string());
                    verts.push(vec![]);
                }
            }
        }

        for (src, dst) in &parsed {
            let spos = edge.iter().position(|s| s == src).expect("invalid node");
            let dpos = edge.iter().position(|s| s == dst).expect("invalid node");

            verts[spos].push(dpos);
            verts[dpos].push(spos);
        }

        Ok((input, Graph { edge, verts }))
    }
}

#[test]
fn input() {
    let graph: Graph = utils::test_input();

    assert_eq!(graph, Graph {
        edge: vec![
            "start".to_string(),
            "end".to_string(),
            "A".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ],
        verts: vec![
            vec![2, 3],
            vec![2, 3],
            vec![0, 4, 3, 1],
            vec![0, 2, 5, 1],
            vec![2],
            vec![3]
        ],
    });
}

#[test]
fn input_mid() {
    let graph: Graph = utils::parse_file("test.input.mid");
    assert_eq!(graph.edge, vec![
        "start".to_string(),
        "end".to_string(),
        "dc".to_string(), // 2
        "HN".to_string(), // 3
        "kj".to_string(), // 4
        "LN".to_string(), // 5
        "sa".to_string(), // 6
    ]);

    assert_eq!(graph.verts, vec![
               vec![3, 4, 2],
               vec![2, 3],
               vec![1, 0, 3, 5, 4],
               vec![0, 2, 1, 4],
               vec![0, 6, 3, 2],
               vec![2],
               vec![4],
    ]);

}


#[test]
fn part1() {
    let graph: Graph = utils::test_input();

    assert_eq!(graph.paths().count(), 10);
}

#[test]
fn part2() {
    let graph: Graph = utils::test_input();

    for x in graph.multi_paths().take(50) {
        println!("{}", graph.path_repr(&x));
    }

    //panic!("foobar");

    assert_eq!(graph.multi_paths().count(), 36);
}

#[test]
fn part1_mid() {
    let graph: Graph = utils::parse_file("test.input.mid");

    assert_eq!(graph.paths().map(|p| println!("{}", graph.path_repr(&p))).count(), 19);
}

#[test]
fn part2_mid() {
    let graph: Graph = utils::parse_file("test.input.mid");

    assert_eq!(graph.multi_paths().map(|p| println!("{}", graph.path_repr(&p))).count(), 103);
}


#[test]
fn part1_lrg() {
    let graph: Graph = utils::parse_file("test.input.lrg");

    assert_eq!(graph.paths().count(), 226);
}

#[test]
fn part2_lrg() {
    let graph: Graph = utils::parse_file("test.input.lrg");

    assert_eq!(graph.multi_paths().count(), 3509);
}
