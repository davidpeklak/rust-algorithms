//! Implements graph algorithms presented in the course

use graph::Graph;

trait Paths<'a> {
    fn has_path(&'a self, v: usize, w: usize) -> bool;

    fn path(&'a self, v: usize, w: usize) -> Option<Vec<usize>>;
}

impl<'a, G> Paths<'a> for G
  where G: Graph<'a> {

    fn has_path(&'a self, v: usize, w: usize) -> bool {
        let dfp = DepthFirstPaths::new(self, v);
        dfp.is_visited(w)
    }

    fn path(&'a self, v: usize, w: usize) -> Option<Vec<usize>> {
        let dfp = DepthFirstPaths::new(self, v);
        dfp.path_to(w)
    }
}

struct DepthFirstPaths<'a, G> {
    graph: &'a G,
    start: usize,
    visited: Vec<bool>,
    visited_from: Vec<Option<usize>>
}

impl<'a, G> DepthFirstPaths<'a, G>
    where G: Graph<'a> {

    fn new(graph: &'a G, start: usize) -> DepthFirstPaths<'a, G> {
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

#[cfg(test)]
mod tests {
    use graph::{Graph, GraphImplType};
    use super::Paths;

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
    fn test_has_path_to() {
        let graph = examplary_graph();
        assert!(graph.has_path(0, 3));
        assert!(graph.has_path(6, 2));
        assert!(graph.has_path(1, 3));
        assert!(graph.has_path(5, 1));

        assert!(!graph.has_path(0, 7));
        assert!(!graph.has_path(8, 10));
        assert!(!graph.has_path(2, 9));
        assert!(!graph.has_path(6, 11));

        assert!(graph.has_path(10, 11));
        assert!(graph.has_path(10, 10));
    }

    #[test]
    fn test_path() {
        let graph = examplary_graph();
        assert_eq!(Some(vec!(0)), graph.path(0, 0));
        assert_eq!(Some(vec!(0, 6, 4, 3)), graph.path(0, 3));
        assert_eq!(Some(vec!(6, 0, 2)), graph.path(6, 2));
        assert_eq!(Some(vec!(1, 0, 6, 4, 3)), graph.path(1, 3));
        assert_eq!(Some(vec!(5, 0, 1)), graph.path(5, 1));

        assert_eq!(None, graph.path(0, 7));
        assert_eq!(None, graph.path(8, 10));
        assert_eq!(None, graph.path(2, 9));
        assert_eq!(None, graph.path(6, 11));

        assert_eq!(Some(vec!(10, 9, 12, 11)), graph.path(10, 11));
        assert_eq!(Some(vec!(10)), graph.path(10, 10));
    }
}