use evaluator::Impartial;

use super::{Pit, Cell, Wall};

impl Impartial<Pit> for Pit {
    fn get_parts(self) -> Vec<Pit> {
        vec![self]
    }

    fn get_max_nimber(&self) -> usize {
        self.board.iter().map(|row| {
            row.iter().map(|(cell, _, _)| match cell {
                Cell::On => 1,
                _ => 0,
            }).sum::<usize>()
        }).sum()
    }

    fn get_unique_moves(&self) -> Vec<Pit> {
        let mut groups = vec![];
        for y in 0..self.y {
            let mut group = vec![];
            for x in 0..self.x {
                let (cell, v_wall, _) = self.board[x][y];
                if cell == Cell::On {
                    group.push((x,y))
                }
                if v_wall == Wall::Wall{
                    groups.push(group);
                    group = vec![];
                }
            }
        }
        for x in 0..self.x {
            let mut group = vec![];
            for y in 0..self.y {
                let (cell, _, h_wall) = self.board[x][y];
                if cell == Cell::On {
                    group.push((x,y))
                }
                if h_wall == Wall::Wall{
                    groups.push(group);
                    group = vec![];
                }

            }
        }
        todo!()

    }

    fn get_possible_nimbers(&self) -> Vec<usize> {
        (0..=self.get_max_nimber()).collect()
    }
}
