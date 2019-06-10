//! Implements graph algorithms presented in the course

use graph::Graph;

trait Paths<'a> {
    fn has_path_to(&'a self, v: usize, w: usize) -> bool;
}

impl<'a, G> Paths<'a> for G
  where G: Graph<'a> {

    fn has_path_to(&'a self, v: usize, w: usize) -> bool {
        let mut dfp = DepthFirstPaths::new(self);
        dfp.mark_vertices_recursively(v);
        dfp.is_visited(w)

    }
}

struct DepthFirstPaths<'a, G> {
    graph: &'a G,
    visited: Vec<bool>,
    visited_from: Vec<Option<usize>>
}

impl<'a, G> DepthFirstPaths<'a, G>
    where G: Graph<'a> {

    fn new(graph: &'a G) -> DepthFirstPaths<'a, G> {
        DepthFirstPaths {
            graph,
            visited: vec![false; graph.number_of_vertices()],
            visited_from: vec![None; graph.number_of_vertices()]
        }
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
        assert!(graph.has_path_to(0, 3));
        assert!(graph.has_path_to(6, 2));
        assert!(graph.has_path_to(1, 3));
        assert!(graph.has_path_to(5, 1));

        assert!(!graph.has_path_to(0, 7));
        assert!(!graph.has_path_to(8, 10));
        assert!(!graph.has_path_to(2, 9));
        assert!(!graph.has_path_to(6, 11));

        assert!(graph.has_path_to(10, 11));
        assert!(graph.has_path_to(10, 10));
    }
}