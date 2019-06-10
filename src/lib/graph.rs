//! Implements a graph as described in the course.

pub trait Graph<'a> {
    type AdjIter: Iterator<Item=&'a usize>;

    fn new(number_of_vertices: usize) -> Self;
    fn add_edge(&mut self, v: usize, w: usize);
    fn adj(&'a self, v: usize) -> Self::AdjIter;

    fn number_of_vertices(&self) -> usize;
    fn number_of_edges(&self) -> usize;
}

impl<'a> Graph<'a> for Vec<Vec<usize>> {

    type AdjIter = std::slice::Iter<'a, usize>;

    fn new(number_of_vertices: usize) -> Self {
        let mut vec: Vec<Vec<usize>> = vec!();
        for _ in 0..number_of_vertices {
            vec.push(vec!());
        }
        vec
    }

    fn add_edge(&mut self, v: usize, w: usize) {
        self[v].push(w);
        self[w].push(v);
    }

    fn adj(&'a self, v: usize) -> std::slice::Iter<usize> {
        self[v].iter()
    }

    fn number_of_vertices(&self) -> usize {
        self.len()
    }

    fn number_of_edges(&self) -> usize {
        let twice_the_number: usize = self.iter().map(|v| v.len()).sum();
        twice_the_number / 2
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_number_of_vertices() {
        let g: Vec<Vec<usize>> = Graph::new(4);

        assert_eq!(4, g.number_of_vertices());
    }

    #[test]
    fn test_number_of_edges() {
        let mut g: Vec<Vec<usize>> = Graph::new(4);
        assert_eq!(0, g.number_of_edges());

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        assert_eq!(2, g.number_of_edges());
    }

    #[test]
    fn test_adj() {
        let mut g: Vec<Vec<usize>> = Graph::new(4);

        g.add_edge(0, 1);
        g.add_edge(0, 2);

        let adj: Vec<usize> = g.adj(0).map(|x| *x).collect();

        assert_eq!(vec!(1, 2), adj);
    }
}