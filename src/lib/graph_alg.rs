//! Implements graph algorithms presented in the course

use graph::Graph;
use std::collections::vec_deque::VecDeque;

pub trait Paths {
    fn is_visited(&self, u: usize) -> bool;

    fn path_to(&self, v: usize) -> Option<Vec<usize>>;
}

pub struct DepthFirstPaths<'a, G> {
    graph: &'a G,
    start: usize,
    visited: Vec<bool>,
    visited_from: Vec<Option<usize>>
}

impl<'a, G> DepthFirstPaths<'a, G>
    where G: Graph<'a> {

    pub fn new(graph: &'a G, start: usize) -> DepthFirstPaths<'a, G> {
        let mut dfp = DepthFirstPaths {
            graph,
            start,
            visited: vec![false; graph.number_of_vertices()],
            visited_from: vec![None; graph.number_of_vertices()]
        };

        dfp.mark_vertices_recursively(start);

        dfp
    }

    fn mark_vertices_recursively(&mut self, v: usize) {
        self.visited[v] = true;
        for &w in self.graph.adj(v) {
            if !self.visited[w] {
                self.visited_from[w] = Some(v);
                self.mark_vertices_recursively(w);
            }
        }
    }
}

impl<'a, G> Paths for DepthFirstPaths<'a, G>
    where G: Graph<'a> {

    fn is_visited(&self, v: usize) -> bool {
        self.visited[v]
    }

    fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if v == self.start {
            Some(vec!(v))
        } else {
            self.visited_from[v]
                .and_then(|prev| {
                    self.path_to(prev)
                        .map(|mut prev_vec| {
                            prev_vec.push(v);
                            prev_vec
                        })
                })
        }
    }
}

pub struct BreadthFirstPaths<'a, G> {
    _graph: &'a G,
    start: usize,
    visited_from: Vec<Option<usize>>,
    path_length_from_start: Vec<Option<usize>>
}

impl<'a, G> BreadthFirstPaths<'a, G>
    where  G: Graph<'a> {

    pub fn new(graph: &'a G, start: usize) -> BreadthFirstPaths<'a, G> {
        let mut bfp = BreadthFirstPaths {
            _graph: graph,
            start,
            visited_from: vec![None; graph.number_of_vertices()],
            path_length_from_start: vec![None; graph.number_of_vertices()]
        };

        let mut deque: VecDeque<usize> = VecDeque::new();

        deque.push_back(start);
        bfp.path_length_from_start[start] = Some(0);

        while !deque.is_empty() {
            let v = deque.pop_front().unwrap();
            let path_length_from_start = bfp.path_length_from_start[v].map(|l| l + 1);

            for &w in graph.adj(v) {
                if bfp.path_length_from_start[w].is_none() {
                    deque.push_back(w);
                    bfp.visited_from[w] = Some(v);
                    bfp.path_length_from_start[w] = path_length_from_start;
                }
            }
        }

        bfp
    }
}

impl <'a, G> Paths for BreadthFirstPaths<'a, G>
    where G: Graph<'a> {
    fn is_visited(&self, u: usize) -> bool {
        self.path_length_from_start[u].is_some()
    }

    fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if v == self.start {
            Some(vec!(v))
        } else {
            self.visited_from[v]
                .and_then(|prev| {
                    self.path_to(prev)
                        .map(|mut prev_vec| {
                            prev_vec.push(v);
                            prev_vec
                        })
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use graph::{Graph, GraphImplType};
    use super::{Paths, DepthFirstPaths, BreadthFirstPaths};

    fn examplary_graph() -> GraphImplType {
        let mut g: GraphImplType = Graph::new(13);
        g.add_edge(0, 6);
        g.add_edge(0, 2);
        g.add_edge(0, 1);
        g.add_edge(0, 5);

        g.add_edge(3, 5);
        g.add_edge(3, 4);

        g.add_edge(4, 5);
        g.add_edge(4, 6);

        g.add_edge(7, 8);

        g.add_edge(12, 11);
        g.add_edge(12, 9);
        g.add_edge(9, 11);
        g.add_edge(9, 10);

        g
    }

    #[test]
    fn test_depth_is_visted() {
        let graph = examplary_graph();
        assert!(DepthFirstPaths::new(&graph, 0).is_visited(3));
        assert!(DepthFirstPaths::new(&graph, 6).is_visited(2));
        assert!(DepthFirstPaths::new(&graph,1).is_visited(3));
        assert!(DepthFirstPaths::new(&graph,5).is_visited(1));

        assert!(!DepthFirstPaths::new(&graph,0).is_visited(7));
        assert!(!DepthFirstPaths::new(&graph,8).is_visited(10));
        assert!(!DepthFirstPaths::new(&graph,2).is_visited(9));
        assert!(!DepthFirstPaths::new(&graph,6).is_visited(11));

        assert!(DepthFirstPaths::new(&graph,10).is_visited(11));
        assert!(DepthFirstPaths::new(&graph,10).is_visited(10));
    }

    #[test]
    fn test_depth_path_to() {
        let graph = examplary_graph();
        assert_eq!(Some(vec!(0)), DepthFirstPaths::new(&graph,0).path_to(0));
        assert_eq!(Some(vec!(0, 6, 4, 3)), DepthFirstPaths::new(&graph,0).path_to(3));
        assert_eq!(Some(vec!(6, 0, 2)), DepthFirstPaths::new(&graph,6).path_to(2));
        assert_eq!(Some(vec!(1, 0, 6, 4, 3)), DepthFirstPaths::new(&graph,1).path_to(3));
        assert_eq!(Some(vec!(5, 0, 1)), DepthFirstPaths::new(&graph,5).path_to(1));

        assert_eq!(None, DepthFirstPaths::new(&graph,0).path_to(7));
        assert_eq!(None, DepthFirstPaths::new(&graph,8).path_to(10));
        assert_eq!(None, DepthFirstPaths::new(&graph,2).path_to(9));
        assert_eq!(None, DepthFirstPaths::new(&graph,6).path_to(11));

        assert_eq!(Some(vec!(10, 9, 12, 11)), DepthFirstPaths::new(&graph,10).path_to(11));
        assert_eq!(Some(vec!(10)), DepthFirstPaths::new(&graph,10).path_to(10));
    }

    #[test]
    fn test_breadth_is_visted() {
        let graph = examplary_graph();
        assert!(BreadthFirstPaths::new(&graph, 0).is_visited(3));
        assert!(BreadthFirstPaths::new(&graph, 6).is_visited(2));
        assert!(BreadthFirstPaths::new(&graph,1).is_visited(3));
        assert!(BreadthFirstPaths::new(&graph,5).is_visited(1));

        assert!(!BreadthFirstPaths::new(&graph,0).is_visited(7));
        assert!(!BreadthFirstPaths::new(&graph,8).is_visited(10));
        assert!(!BreadthFirstPaths::new(&graph,2).is_visited(9));
        assert!(!BreadthFirstPaths::new(&graph,6).is_visited(11));

        assert!(BreadthFirstPaths::new(&graph,10).is_visited(11));
        assert!(BreadthFirstPaths::new(&graph,10).is_visited(10));
    }

    #[test]
    fn test_breadth_path_to() {
        let graph = examplary_graph();
        assert_eq!(Some(vec!(0)), BreadthFirstPaths::new(&graph,0).path_to(0));
        assert_eq!(Some(vec!(0, 5, 3)), BreadthFirstPaths::new(&graph,0).path_to(3));
        assert_eq!(Some(vec!(6, 0, 2)), BreadthFirstPaths::new(&graph,6).path_to(2));
        assert_eq!(Some(vec!(1, 0, 5, 3)), BreadthFirstPaths::new(&graph,1).path_to(3));
        assert_eq!(Some(vec!(5, 0, 1)), BreadthFirstPaths::new(&graph,5).path_to(1));

        assert_eq!(None, BreadthFirstPaths::new(&graph,0).path_to(7));
        assert_eq!(None, BreadthFirstPaths::new(&graph,8).path_to(10));
        assert_eq!(None, BreadthFirstPaths::new(&graph,2).path_to(9));
        assert_eq!(None, BreadthFirstPaths::new(&graph,6).path_to(11));

        assert_eq!(Some(vec!(10, 9, 11)), BreadthFirstPaths::new(&graph,10).path_to(11));
        assert_eq!(Some(vec!(10)), BreadthFirstPaths::new(&graph,10).path_to(10));
    }
}