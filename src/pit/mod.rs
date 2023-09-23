mod display;
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
        let mut list_of_groups = vec![];

        for y in 0..self.y {
            let mut current_group = vec![];
            for x in 0..self.x {
                let (cell, v_wall, _) = self.board[x ][y ];

                Self::match_cell_and_wall(
                    cell,
                    &mut list_of_groups,
                    &mut current_group,
                    v_wall,
                    x ,
                    y ,
                    self.x ,
                );
            }
            Self::add_group_to_list_of_groups(&mut list_of_groups, &mut current_group);
        }
        for x in 0..self.x {
            let mut current_group = vec![];
            for y in 0..self.y {
                let (cell, _, h_wall) = self.board[x ][y ];

                Self::match_cell_and_wall(
                    cell,
                    &mut list_of_groups,
                    &mut current_group,
                    h_wall,
                    x ,
                    y ,
                    self.x ,
                );
            }
            Self::add_group_to_list_of_groups(&mut list_of_groups, &mut current_group);
        }

        return TakingGame::new(list_of_groups);
    }

    ///handles the matching of a cell and following wall, value
    fn match_cell_and_wall(
        cell: Cell,
        list_of_groups: &mut Vec<Vec<usize>>,
        current_group: &mut Vec<usize>,
        wall: Wall,
        x: usize,
        y: usize,
        board_x: usize,
    ) {
        match cell {
            Cell::On => {
                current_group.push(x + (y * board_x));
                match wall {
                    Wall::Wall => Self::add_group_to_list_of_groups(list_of_groups, current_group),
                    Wall::None => (),
                }
            }
            Cell::Off => (),
        }
    }

    fn add_group_to_list_of_groups(list_of_groups: &mut Vec<Vec<usize>>, group: &mut Vec<usize>) {
        //No need to add empty groups
        if group.is_empty() {
        }
        //skips redundant empty groups
        else if group.len() == 1 && list_of_groups.contains(group) {
            group.clear();
        } else {
            list_of_groups.push(group.clone());
            group.clear();
        }
    }

    fn new(board: Vec<Vec<(Cell, Wall, Wall)>>) -> Pit {
        let x = board.len();
        let y = board[0].len();
        Pit{x, y, board}
    }
}

