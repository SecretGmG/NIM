mod display;
pub mod new;
pub mod cell;
//pub mod reconstruct;
use self::cell::{Cell, Wall, CellWalls};

use super::generalized::TakingGame;
#[derive(Debug)]
pub struct Pit {
    ///first x then y
    board: Vec<Vec<CellWalls>>,
    x: u8,
    y: u8,
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
                let (cell, v_wall, _) = self.board[x as usize][y as usize];

                Self::match_cell_and_wall(
                    cell,
                    &mut list_of_groups,
                    &mut current_group,
                    v_wall,
                    x as u16,
                    y as u16,
                    self.x as u16,
                );
            }
            Self::add_group_to_list_of_groups(&mut list_of_groups, &mut current_group);
        }
        for x in 0..self.x {
            let mut current_group = vec![];
            for y in 0..self.y {
                let (cell, _, h_wall) = self.board[x as usize][y as usize];

                Self::match_cell_and_wall(
                    cell,
                    &mut list_of_groups,
                    &mut current_group,
                    h_wall,
                    x as u16,
                    y as u16,
                    self.x as u16,
                );
            }
            Self::add_group_to_list_of_groups(&mut list_of_groups, &mut current_group);
        }

        return TakingGame::new(list_of_groups);
    }

    ///handles the matching of a cell and following wall, value
    fn match_cell_and_wall(
        cell: cell::Cell,
        list_of_groups: &mut Vec<Vec<u16>>,
        current_group: &mut Vec<u16>,
        wall: cell::Wall,
        x: u16,
        y: u16,
        board_x: u16,
    ) {
        match cell {
            cell::Cell::On => {
                current_group.push(x + (y * board_x));
                match wall {
                    cell::Wall::Wall => Self::add_group_to_list_of_groups(list_of_groups, current_group),
                    cell::Wall::None => (),
                }
            }
            cell::Cell::Off => (),
        }
    }

    fn add_group_to_list_of_groups(list_of_groups: &mut Vec<Vec<u16>>, group: &mut Vec<u16>) {
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
}

