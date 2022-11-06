
use crate::nim::{generalized::closed_generalized::ClosedGeneralizedNimGame};
use super::{*, cell::CellWalls};

impl Pit {
    pub fn new(board: Vec<Vec<CellWalls>>) -> Pit {
        let x = board.len() as u8;
        let y = board[0].len() as u8;

        return Pit {
            board: board,
            x: x,
            y: y,
        };
    }
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
        line[0] = (cell::Cell::On, cell::Wall::None, cell::Wall::None);

        for _ in 1..x {
            board.push(line.clone());
        }
        let pit = Pit { board, x: x, y: y };

        return pit;
    }
    pub fn random_rect(
        cell_type_distribution: (u32, u32),
        wall_type_distribution: (u32, u32),
        x: u8,
        y: u8,
    ) -> Pit {
        let mut board = vec![vec![(Cell::Off, Wall::None, Wall::None); y as usize]; x as usize];

        let mut rng = thread_rng();
        for i in 0..x {
            for j in 0..y {
                let (off, on) = cell_type_distribution;
                let rand_int = rng.gen_range(0, off + on);
                let cell = if rand_int < off { cell::Cell::On } else { cell::Cell::Off };

                let (none, some) = wall_type_distribution;
                let rand_int = rng.gen_range(0, none + some);
                let h_wall = if rand_int < none {
                    cell::Wall::None
                } else {
                    cell::Wall::Wall
                };

                let (none, some) = wall_type_distribution;
                let v_wall;
                let rand_int = rng.gen_range(0, none + some);
                if rand_int < none {
                    v_wall = cell::Wall::None
                } else {
                    v_wall = cell::Wall::Wall
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


