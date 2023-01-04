use super::GeneralizedNimGame;
use rand::Rng;
use std::vec;

pub struct Constructor {
    g: GeneralizedNimGame,
}
impl Constructor {

    pub fn new(groups: Vec<Vec<u16>>) -> Constructor{
        return Constructor {g: GeneralizedNimGame::new(groups)};
    }

    pub fn unit() -> Constructor {
        return Constructor::new(vec![vec![0]]);
    }
    pub fn rand(
        node_count: u16,
        group_count: u16,
        min_groups_per_node: u16,
        max_groups_per_node: u16,
    ) -> Constructor {
        let mut groups = vec![vec![]; group_count as usize];
        for node in 0..node_count {
            for _ in 0..(rand::thread_rng().gen_range(min_groups_per_node, max_groups_per_node)) {
                groups[rand::thread_rng().gen_range(0, group_count) as usize].push(node);
            }
        }
        return Constructor::new(groups);
    }
    pub fn hyper_cube_like(node_count: f32, dim: f32) -> Constructor {
        let size = node_count.powf(1.0 / dim);
        let node_count = node_count.round() as u16;
        let group_count = (dim * size).round() as u16;

        Self::rand(
            node_count,
            group_count,
            dim.floor() as u16,
            dim.ceil() as u16 + 1,
        )
    }
    
    #[allow(dead_code)]
    pub fn triangle(l: u16) -> Constructor {
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
    pub fn rect(x: u16, y: u16) -> Constructor {
        Self::hyper_cuboid(vec![x, y])
    }
    pub fn hyper_cube(dim: u16, l: u16) -> Constructor {
        Self::hyper_cuboid(vec![l; dim as usize])
    }
    pub fn hyper_cuboid(lengths: Vec<u16>) -> Constructor {
        let mut g = Self::unit();
        for length in lengths {
            g = g.extrude(length);
        }
        return g;
    }
    #[allow(dead_code)]
    pub fn hyper_tetrahedron(dim: u16) -> Constructor {
        let mut g = Self::unit();
        for _ in 0..dim {
            g = g.add_connection_to_all();
        }
        return g;
    }
    pub fn build(self) -> GeneralizedNimGame{
        return self.g;
    }
    pub fn add_connection_to_all(mut self) -> Constructor {
        return self.fully_connect(&Self::unit().build());
    }
    pub fn fully_connect(mut self, g:&GeneralizedNimGame) -> Constructor {
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
        self.g = GeneralizedNimGame::new(new_groups);
        return self;
    }
    pub fn combine(mut self, g: GeneralizedNimGame) -> Constructor{
        let mut new_groups = self.g.get_groups();
        let node_count = self.g.get_node_count();
        for group in g.get_groups() {
            new_groups.push(group.iter().map(|n| n + node_count).collect());
        }
        return Self::new(new_groups);
    }
    pub fn extrude(mut self, l: u16) -> Constructor {
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
        self.g = GeneralizedNimGame::new(new_groups);
        return self;
    }
}
