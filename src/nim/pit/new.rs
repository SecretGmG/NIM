use super::*;

impl Pit {
    pub fn empty() -> Pit {
        return Pit {
            board: Vec::new(),
            x: 0,
            y: 0,
        };
    }

    pub fn empty_rect(x: u8, y: u8) -> Pit {
        return Pit {
            board: vec![vec![(Cell::On, Wall::None, Wall::None); y as usize]; x as usize],
            x: x,
            y: y,
        };
    }
    pub fn empty_L_shape(x: u8, y: u8) -> Pit {
        let mut board = vec![];
        board.push(vec![(Cell::On, Wall::None, Wall::None); y as usize]);

        let mut line = vec![(Cell::Off, Wall::None, Wall::None); y as usize];
        line[0] = (Cell::On, Wall::None, Wall::None);

        for _ in 1..x {
            board.push(line.clone());
        }
        let pit = Pit { board, x: x, y: y };

        return pit;
    }
    pub fn random_rect(
        cell_type_distribution: (u32, u32, u32),
        wall_type_distribution: (u32, u32),
        x: u8,
        y: u8,
    ) -> Pit {
        let mut board = vec![vec![(Cell::None, Wall::None, Wall::None); y as usize]; x as usize];

        let mut rng = thread_rng();
        for i in 0..x {
            for j in 0..y {
                let (none, empty, checked) = cell_type_distribution;
                let cell;
                let rand_int = rng.gen_range(0, none + empty + checked);
                if rand_int < none {
                    cell = Cell::None
                } else if rand_int < empty {
                    cell = Cell::On
                } else {
                    cell = Cell::Off
                }

                let (none, some) = wall_type_distribution;
                let h_wall;
                let rand_int = rng.gen_range(0, none + some);
                if rand_int < none {
                    h_wall = Wall::None
                } else {
                    h_wall = Wall::Wall
                }

                let (none, some) = wall_type_distribution;
                let v_wall;
                let rand_int = rng.gen_range(0, none + some);
                if rand_int < none {
                    v_wall = Wall::None
                } else {
                    v_wall = Wall::Wall
                }

                board[i as usize][j as usize] = (cell, h_wall, v_wall);
            }
        }
        return Pit {
            x: x,
            y: y,
            board: board,
        };
    }
}
