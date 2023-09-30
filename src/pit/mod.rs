mod display;
pub mod impls;
pub mod new;
pub mod reconstruct;

use super::generalized_taking_game::TakingGame;

#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Off,
    On,
}

#[derive(Copy, Clone, Debug)]
pub enum Wall {
    None,
    Wall,
}
pub type CellWalls = (Cell, Wall, Wall);



#[derive(Debug)]
pub struct Pit {
    ///first x then y
    board: Vec<Vec<CellWalls>>,
    x: usize,
    y: usize,
}

impl Pit {
    pub fn make_move() -> Option<Pit> {
        todo!();
    }

    pub fn get_generalized(&self) -> TakingGame {
        let mut list_of_sets = vec![];

        for y in 0..self.y {
            let mut set = vec![];
            for x in 0..self.x {
                let (cell, v_wall, _) = self.board[x ][y ];

                Self::match_cell_and_wall(
                    cell,
                    &mut list_of_sets,
                    &mut set,
                    v_wall,
                    x ,
                    y ,
                    self.x ,
                );
            }
            Self::append_set(&mut list_of_sets, &mut set);
        }
        for x in 0..self.x {
            let mut set = vec![];
            for y in 0..self.y {
                let (cell, _, h_wall) = self.board[x ][y ];

                Self::match_cell_and_wall(
                    cell,
                    &mut list_of_sets,
                    &mut set,
                    h_wall,
                    x ,
                    y ,
                    self.x ,
                );
            }
            Self::append_set(&mut list_of_sets, &mut set);
        }

        return TakingGame::new(list_of_sets);
    }

    ///handles the matching of a cell and following wall, value
    fn match_cell_and_wall(
        cell: Cell,
        sets_of_nodes: &mut Vec<Vec<usize>>,
        set: &mut Vec<usize>,
        wall: Wall,
        x: usize,
        y: usize,
        board_x: usize,
    ) {
        match cell {
            Cell::On => {
                set.push(x + (y * board_x));
                match wall {
                    Wall::Wall => Self::append_set(sets_of_nodes, set),
                    Wall::None => (),
                }
            }
            Cell::Off => (),
        }
    }

    fn append_set(sets_of_nodes: &mut Vec<Vec<usize>>, set: &mut Vec<usize>) {
        //No need to add empty sets
        if set.is_empty() {
        }
        //skips redundant empty sets
        else if set.len() == 1 && sets_of_nodes.contains(set) {
            set.clear();
        } else {
            sets_of_nodes.push(set.clone());
            set.clear();
        }
    }

    fn new(board: Vec<Vec<(Cell, Wall, Wall)>>) -> Pit {
        let x = board.len();
        let y = board[0].len();
        Pit{x, y, board}
    }
}

