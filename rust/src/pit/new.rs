use rand::{rng, Rng};

use super::{Pit,Cell,Wall};

impl Pit {
    #[allow(dead_code)]
    pub fn empty() -> Pit {
        return Pit {
            board: Vec::new(),
            x: 0,
            y: 0,
        };
    }
    #[allow(dead_code)]
    pub fn empty_rect(x: usize, y: usize) -> Pit {
        return Pit {
            board: vec![vec![(Cell::On, Wall::None, Wall::None); y ]; x ],
            x: x,
            y: y,
        };
    }
    #[allow(dead_code, non_snake_case)] //the L is supposed to be uppercase
    pub fn empty_L_shape(x: usize, y: usize) -> Pit {
        let mut board = vec![];
        board.push(vec![(Cell::On, Wall::None, Wall::None); y ]);

        let mut line = vec![(Cell::Off, Wall::None, Wall::None); y ];
        line[0] = (Cell::On, Wall::None, Wall::None);

        for _ in 1..x {
            board.push(line.clone());
        }
        let pit = Pit { board, x: x, y: y };

        return pit;
    }
    #[allow(dead_code)]
    ///cell_type_distrs(off, on) wall_type_distr(none, some)
    pub fn random_rect(
        cell_type_distribution: (u32, u32),
        wall_type_distribution: (u32, u32),
        x: usize,
        y: usize,
    ) -> Pit {
        let mut board = vec![vec![(Cell::Off, Wall::None, Wall::None); y ]; x ];

        let mut rng = rng();
        for i in 0..x {
            for j in 0..y {
                let (off, on) = cell_type_distribution;
                let rand_int = rng.random_range(..(off + on));
                let cell = if rand_int < off {
                    Cell::Off
                } else {
                    Cell::On
                };

                let (none, some) = wall_type_distribution;
                let rand_int = rng.random_range(..(none + some));
                let h_wall = if rand_int < none {
                    Wall::None
                } else {
                    Wall::Wall
                };

                let (none, some) = wall_type_distribution;
                let v_wall;
                let rand_int = rng.random_range(..(none + some));
                if rand_int < none {
                    v_wall = Wall::None
                } else {
                    v_wall = Wall::Wall
                }

                board[i][j] = (cell, h_wall, v_wall);
            }
        }
        return Pit {
            x: x,
            y: y,
            board: board,
        };
    }
}
