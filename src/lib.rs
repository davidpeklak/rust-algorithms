pub mod quick_find {
    pub trait QuickFind {
        type Name;

        fn new(size: usize) -> Self;
        fn is_connected(&self, first: Self::Name, second: Self::Name) -> bool;
        fn connect(&mut self, first: Self::Name, second: Self::Name);
    }

    impl QuickFind for Vec<u32> {
        type Name = usize;

        fn new(size: usize) -> Vec<u32> {
            (0..size as u32).collect()
        }

        fn is_connected(&self, first: usize, second: usize) -> bool {
            self[first] == self[second]
        }

        fn connect(&mut self, first: usize, second: usize) {
            let compound = self[first];
            let other_compund = self[second];

            for i in 0..self.len() {
                if self[i] == other_compund {
                    self[i] = compound;
                }
            }
        }
    }
}
