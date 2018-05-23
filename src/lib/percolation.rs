//! implements the percolation example of the course
//! # Example
//! ```
//! use algorithms::percolation::{Percolation, PercolationState};
//! let mut perc: PercolationState = Percolation::new(3);
//!
//! assert!(!perc.percolates());
//!
//! perc.open(0, 2);
//! perc.open(1, 0);
//! perc.open(1, 1);
//! perc.open(1, 2);
//! perc.open(2, 0);
//!
//! assert!(perc.percolates());
//! println!("{}", perc);
//! ```

use super::weighted_quick_union::{WeightedQuickUnion, WQU};
use std::fmt;

pub trait Percolation {
    /// create n-by-n grid, with all sites blocked
    fn new(n: usize) -> Self;

    /// open site (row, col) if it is not open already
    fn open(&mut self, row: usize, col: usize);

    fn is_open(&self, row: usize, col: usize) -> bool;

    fn is_full(&self, row: usize, col: usize) -> bool;

    fn number_of_open_sites(&self) -> usize;

    fn percolates(&self) -> bool;
}

/// identifies sites in the grid
enum Site {
    Coord { row: usize, col: usize },
    VirtualTop,
    VirtualBottom,
}

#[derive(Clone, PartialEq)]
enum SiteState {
    Open,
    Full,
}

pub struct PercolationState {
    size: usize,
    wqu: WQU,
    site_states: Vec<SiteState>,
    open_count: usize
}

impl PercolationState {
    /// converts a Site to an index in a vector
    fn site_to_index(&self, grid_site: Site) -> usize {
        match grid_site {
            Site::VirtualTop => 0,
            Site::Coord { row, col } => row * self.size + col + 1,
            Site::VirtualBottom => self.size * self.size + 1
        }
    }

    /// calculates the necessary size of the vectors for the given grid-size
    fn vec_size(size: usize) -> usize {
        size * size + 2
    }

    /// connects a site (if it is open) to its open neighbors
    fn connect_to_open_neighbors(&mut self, row: usize, col: usize) {
        let index = self.site_to_index(Site::Coord{row, col});

        // connect to upper neighbor
        {
            let upper_neighbor_index = if row == 0 {
                self.site_to_index(Site::VirtualTop)
            } else {
                self.site_to_index(Site::Coord { row: row - 1, col })
            };
            self.connect_if_open(index, upper_neighbor_index);
        }

        // connect to lower neighbor
        {
            let lower_neighbor_index = if row == self.size - 1 {
                self.site_to_index(Site::VirtualBottom)
            } else {
                self.site_to_index(Site::Coord { row: row + 1, col })
            };
            self.connect_if_open(index, lower_neighbor_index);
        }

        // connect to left neighbor
        if col != 0 {
            let left_neighbor_index = self.site_to_index(Site::Coord { row, col: col - 1 });
            self.connect_if_open(index, left_neighbor_index);
        }

        // connect to right neighbor
        if col != self.size - 1 {
            let right_neighbor_index = self.site_to_index(Site::Coord { row, col: col + 1 });
            self.connect_if_open(index, right_neighbor_index);
        }
    }

    /// connects two sites if they are both open
    fn connect_if_open(&mut self, first: usize, second: usize) {
        if self.site_states[first] == SiteState::Open && self.site_states[second] == SiteState::Open {
            self.wqu.connect(first, second);
        }
    }
}

impl Percolation for PercolationState {
    fn new(size: usize) -> Self {
        let vec_size = PercolationState::vec_size(size);

        let mut new_perc_state = PercolationState {
            size,
            wqu: WeightedQuickUnion::new(vec_size),
            site_states: vec![SiteState::Full; vec_size],
            open_count: 0
        };

        let virtual_top = new_perc_state.site_to_index(Site::VirtualTop);
        new_perc_state.site_states[virtual_top] = SiteState::Open;

        let virtual_bottom = new_perc_state.site_to_index(Site::VirtualBottom);
        new_perc_state.site_states[virtual_bottom] = SiteState::Open;

        new_perc_state
    }

    fn open(&mut self, row: usize, col: usize) {
        let i = self.site_to_index(Site::Coord { row, col });
        if self.site_states[i] == SiteState::Full {
            self.site_states[i] = SiteState::Open;
            self.open_count = self.open_count + 1;

            self.connect_to_open_neighbors(row, col);
        }
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        self.site_states[self.site_to_index(Site::Coord { row, col })] == SiteState::Open
    }

    fn is_full(&self, row: usize, col: usize) -> bool {
        self.site_states[self.site_to_index(Site::Coord { row, col })] == SiteState::Full
    }

    fn number_of_open_sites(&self) -> usize {
        self.open_count
    }

    fn percolates(&self) -> bool {
        let virtual_top = self.site_to_index(Site::VirtualTop);
        let virtual_bottom = self.site_to_index(Site::VirtualBottom);
        self.wqu.is_connected(virtual_top, virtual_bottom)
    }
}

impl fmt::Display for PercolationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for row in 0..self.size {
            for col in 0..self.size {
                let index = self.site_to_index(Site::Coord{row, col});
                let site_state = &self.site_states[index];
                let symbol = if *site_state == SiteState::Open {
                    let virtual_top_index = self.site_to_index(Site::VirtualTop);
                    if self.wqu.is_connected(index, virtual_top_index) {
                        'O'
                    } else {
                        '#'
                    }
                } else {
                    '.'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Percolation, PercolationState};

    #[test]
    fn new_percolation_is_full() {
        let size = 5;
        let perc: PercolationState = Percolation::new(size);

        for row in 0..size {
            for col in 0..size {
                assert!(perc.is_full(row, col));
                assert!(!perc.is_open(row, col));
            }
        }
    }

    #[test]
    fn open_once() {
        let size = 5;
        let mut perc: PercolationState = Percolation::new(size);

        assert!(perc.is_full(0, 0));
        assert_eq!(perc.number_of_open_sites(), 0);

        perc.open(0, 0);

        assert!(perc.is_open(0, 0));
        assert_eq!(perc.number_of_open_sites(), 1);
    }

    #[test]
    fn open_twice() {
        let size = 5;
        let mut perc: PercolationState = Percolation::new(size);

        assert!(perc.is_full(0, 0));
        assert_eq!(perc.number_of_open_sites(), 0);

        perc.open(0, 0);
        perc.open(0, 0);

        assert!(perc.is_open(0, 0));
        assert_eq!(perc.number_of_open_sites(), 1);
    }

    #[test]
    fn percolate_1() {
        let size = 1;
        let mut perc: PercolationState = Percolation::new(size);

        assert!(!perc.percolates());

        perc.open(0, 0);

        assert!(perc.percolates());
    }

    #[test]
    fn percolate_3_s() {
        let size = 3;
        let mut perc: PercolationState = Percolation::new(size);

        assert!(!perc.percolates());

        // #..
        // ###
        // ..#
        perc.open(0, 0);
        perc.open(1, 0);
        perc.open(1, 1);
        perc.open(1, 2);
        perc.open(2, 2);

        assert!(perc.percolates());
    }

    #[test]
    fn percolate_3_z() {
        let size = 3;
        let mut perc: PercolationState = Percolation::new(size);

        assert!(!perc.percolates());

        // ..#
        // ###
        // #..
        perc.open(0, 2);
        perc.open(1, 0);
        perc.open(1, 1);
        perc.open(1, 2);
        perc.open(2, 0);

        assert!(perc.percolates());
    }
}
