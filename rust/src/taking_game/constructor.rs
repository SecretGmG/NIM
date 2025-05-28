use super::TakingGame;
use rand::{rng,Rng};
use std::vec;

pub struct Constructor {
    g: TakingGame,
}
impl Constructor {

    pub fn new(sets_of_nodes: Vec<Vec<usize>>) -> Constructor{
        return Constructor {g: TakingGame::new(sets_of_nodes)};
    }

    pub fn empty() -> Constructor{
        return Constructor::new(vec![vec![]]);
    }
    pub fn unit() -> Constructor {
        return Constructor::new(vec![vec![0]]);
    }

    pub fn kayles(size: usize) -> Constructor{
        if size == 0 {
            return Constructor::empty();
        }
        if size == 1 {
            return Constructor::unit();
        }
        let mut sets_of_nodes = vec![];
        for i in 1..size{
            sets_of_nodes.push(vec![i-1, i]);
        }
        return Constructor::new(sets_of_nodes);
    }
    
    #[allow(dead_code)]
    pub fn rand(
        node_count: usize,
        set_count: usize,
        min_sets_per_node: usize,
        max_sets_per_node: usize,
    ) -> Constructor {
        let mut sets_of_nodes = vec![vec![]; set_count ];
        for node in 0..node_count {
            for _ in 0..(rng().random_range(min_sets_per_node..max_sets_per_node)) {
                sets_of_nodes[rng().random_range(..set_count) ].push(node);
            }
        }
        return Constructor::new(sets_of_nodes);
    }
    
    #[allow(dead_code)]
    pub fn triangle(l: usize) -> Constructor {
        let mut sets_of_nodes = vec![];
        for i in 0..l {
            let mut new_set_of_nodes1 = vec![];
            let mut new_set_of_nodes2 = vec![];
            let mut new_set_of_nodes3 = vec![];
            for j in 0..(l - i) {
                /*
                12# # #
                8 9 # #
                4 5 6 #
                0 1 2 3
                */
                new_set_of_nodes1.push(i + j * l);
                new_set_of_nodes2.push(j + i * l);
                new_set_of_nodes3.push(l - 1 - i + j * (l - 1));
            }
            sets_of_nodes.push(new_set_of_nodes1);
            sets_of_nodes.push(new_set_of_nodes2);
            sets_of_nodes.push(new_set_of_nodes3);
        }
        return Constructor::new(sets_of_nodes);
    }
    #[allow(dead_code)]
    pub fn rect(x: usize, y: usize) -> Constructor {
        Self::hyper_cuboid(vec![x, y])
    }
    #[allow(dead_code)]
    pub fn hyper_cube(dim: usize, l: usize) -> Constructor {
        Self::hyper_cuboid(vec![l; dim ])
    }
    #[allow(dead_code)]
    pub fn hyper_cuboid(lengths: Vec<usize>) -> Constructor {
        let mut g = Self::unit();
        for length in lengths {
            g = g.extrude(length);
        }
        return g;
    }
    #[allow(dead_code)]
    pub fn hyper_tetrahedron(dim: usize) -> Constructor {
        let mut g = Self::unit();
        for _ in 0..dim {
            g = g.add_connection_to_all();
        }
        return g;
    }
    pub fn build(self) -> TakingGame{
        return self.g;
    }
    #[allow(dead_code)]
    pub fn add_connection_to_all(self) -> Constructor {
        return self.fully_connect(&Self::unit().build());
    }
    pub fn fully_connect(mut self, g:&TakingGame) -> Constructor {
        let node_count = self.g.get_node_count();
        let mut new_sets_of_nodes = self.g.get_sets_of_nodes().clone();
        for set in g.get_sets_of_nodes() {
            new_sets_of_nodes.push(set.iter().map(|n| n + node_count).collect());
        }
        for i in 0..node_count {
            for j in node_count..(node_count + g.get_node_count()) {
                new_sets_of_nodes.push(vec![i, j]);
            }
        }
        self.g = TakingGame::new(new_sets_of_nodes);
        return self;
    }
    #[allow(dead_code)]
    pub fn combine(self, g: TakingGame) -> Constructor{
        let mut new_sets_of_nodes = self.g.get_sets_of_nodes().clone();
        let node_count = self.g.get_node_count();
        for set_of_nodes in g.get_sets_of_nodes() {
            new_sets_of_nodes.push(set_of_nodes.iter().map(|n| n + node_count).collect());
        }
        return Self::new(new_sets_of_nodes);
    }
    pub fn extrude(mut self, l: usize) -> Constructor {
        let mut new_sets_of_nodes = self.g.get_sets_of_nodes().clone();
        let node_count = self.g.get_node_count();

        for set in self.g.get_sets_of_nodes() {
            for offset in 0..l {
                let mut new_set_of_nodes = vec![];
                for node in set {
                    new_set_of_nodes.push(node + offset * node_count);
                }
                new_sets_of_nodes.push(new_set_of_nodes);
            }
        }
        for node in 0..node_count {
            let mut new_set = vec![];
            for offset in 0..l {
                new_set.push(node + offset * node_count);
            }
            new_sets_of_nodes.push(new_set);
        }
        self.g = TakingGame::new(new_sets_of_nodes);
        return self;
    }
}
