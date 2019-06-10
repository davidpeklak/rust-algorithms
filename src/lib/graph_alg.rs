//! Implements graph algorithms presented in the course

use graph::Graph;

trait Paths {
    fn has_path_to(&self, v: usize, w: usize) -> bool;
}

impl<'a, G> Paths for G
  where G: Graph<'a> {

    fn has_path_to(&self, _v: usize, _w: usize) -> bool {
        // let mut visited: Vec<bool> = vec![false; self.number_of_vertices()];
        // let mut visited_from: Vec<Option<usize>> = vec![None; self.number_of_vertices()];

        false
    }
}