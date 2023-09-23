use super::TakingGame;
use rand::Rng;
use std::vec;

pub struct Constructor {
    g: TakingGame,
}
impl Constructor {

    pub fn new(groups: Vec<Vec<usize>>) -> Constructor{
        return Constructor {g: TakingGame::new(groups)};
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
        let mut groups = vec![];
        for i in 1..size{
            groups.push(vec![i-1, i]);
        }
        return Constructor::new(groups);
    }
    
    #[allow(dead_code)]
    pub fn rand(
        node_count: usize,
        group_count: usize,
        min_groups_per_node: usize,
        max_groups_per_node: usize,
    ) -> Constructor {
        let mut groups = vec![vec![]; group_count ];
        for node in 0..node_count {
            for _ in 0..(rand::thread_rng().gen_range(min_groups_per_node, max_groups_per_node)) {
                groups[rand::thread_rng().gen_range(0, group_count) ].push(node);
            }
        }
        return Constructor::new(groups);
    }
    
    #[allow(dead_code)]
    pub fn triangle(l: usize) -> Constructor {
        let mut groups = vec![];
        for i in 0..l {
            let mut new_group1 = vec![];
            let mut new_group2 = vec![];
            let mut new_group3 = vec![];
            for j in 0..(l - i) {
                /*
                12# # #
                8 9 # #
                4 5 6 #
                0 1 2 3
                */
                new_group1.push(i + j * l);
                new_group2.push(j + i * l);
                new_group3.push(l - 1 - i + j * (l - 1));
            }
            groups.push(new_group1);
            groups.push(new_group2);
            groups.push(new_group3);
        }
        return Constructor::new(groups);
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
        let mut new_groups = self.g.get_groups();
        for group in g.get_groups() {
            new_groups.push(group.iter().map(|n| n + node_count).collect());
        }
        for i in 0..node_count {
            for j in node_count..(node_count + g.get_node_count()) {
                new_groups.push(vec![i, j]);
            }
        }
        self.g = TakingGame::new(new_groups);
        return self;
    }
    #[allow(dead_code)]
    pub fn combine(self, g: TakingGame) -> Constructor{
        let mut new_groups = self.g.get_groups();
        let node_count = self.g.get_node_count();
        for group in g.get_groups() {
            new_groups.push(group.iter().map(|n| n + node_count).collect());
        }
        return Self::new(new_groups);
    }
    pub fn extrude(mut self, l: usize) -> Constructor {
        let mut new_groups = self.g.get_groups().clone();
        let node_count = self.g.get_node_count();

        for group in self.g.get_groups() {
            for offset in 0..l {
                let mut new_group = vec![];
                for node in &group {
                    new_group.push(node + offset * node_count);
                }
                new_groups.push(new_group);
            }
        }
        for node in 0..node_count {
            let mut new_group = vec![];
            for offset in 0..l {
                new_group.push(node + offset * node_count);
            }
            new_groups.push(new_group);
        }
        self.g = TakingGame::new(new_groups);
        return self;
    }
}
