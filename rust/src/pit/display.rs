use super::{Cell, Wall, Pit};
use std::fmt;

impl Pit {
    fn get_cell_at(&self, x: usize, y: usize) -> Cell {
        if x < self.x && y < self.y {
            let (cell, _, _) = self.board[x][y];
            return cell;
        }
        return Cell::Off;
    }
    fn get_v_wall_at(&self, x: usize, y: usize) -> Wall {
        if y >= self.y {
            return Wall::None;
        } else if x == self.x - 1 {
            return Wall::Wall;
        } else {
            let (_, v_wall, _) = self.board[x][y];
            return v_wall;
        }
    }
    fn get_h_wall_at(&self, x: usize, y: usize) -> Wall {
        if x == self.x {
            return Wall::None;
        } else if y == self.y - 1 {
            return Wall::Wall;
        } else {
            let (_, _, h_wall) = self.board[x][y];
            return h_wall;
        }
    }
}

impl fmt::Display for Pit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = String::from("");

        for y in 0..self.y {
            for x in 0..self.x {
                let cell = self.get_cell_at(x, y);
                let v_wall = self.get_v_wall_at(x, y);

                match cell {
                    Cell::Off => display_string.push(' '),
                    Cell::On => display_string.push('O'),
                }

                match v_wall {
                    Wall::None => display_string.push(' '),
                    Wall::Wall => display_string.push('│'),
                }
            }

            display_string.push('\n');

            for x in 0..self.x {
                let v_wall = self.get_v_wall_at(x, y);
                let h_wall = self.get_h_wall_at(x, y);

                match h_wall {
                    Wall::Wall => display_string.push('─'),
                    Wall::None => display_string.push(' '),
                }

                let next_v_wall = self.get_v_wall_at(x, y + 1);
                let next_h_wall = self.get_h_wall_at(x + 1, y);

                match (v_wall, h_wall, next_v_wall, next_h_wall) {
                    (Wall::Wall, Wall::Wall, Wall::Wall, Wall::Wall) => display_string.push('┼'),
                    (Wall::Wall, Wall::Wall, Wall::Wall, Wall::None) => display_string.push('┤'),
                    (Wall::Wall, Wall::Wall, Wall::None, Wall::Wall) => display_string.push('┴'),
                    (Wall::Wall, Wall::Wall, Wall::None, Wall::None) => display_string.push('┘'),
                    (Wall::Wall, Wall::None, Wall::Wall, Wall::Wall) => display_string.push('├'),
                    (Wall::Wall, Wall::None, Wall::Wall, Wall::None) => display_string.push('│'),
                    (Wall::Wall, Wall::None, Wall::None, Wall::Wall) => display_string.push('└'),
                    (Wall::Wall, Wall::None, Wall::None, Wall::None) => display_string.push(' '),
                    (Wall::None, Wall::Wall, Wall::Wall, Wall::Wall) => display_string.push('┬'),
                    (Wall::None, Wall::Wall, Wall::Wall, Wall::None) => display_string.push('┐'),
                    (Wall::None, Wall::Wall, Wall::None, Wall::Wall) => display_string.push('─'),
                    (Wall::None, Wall::Wall, Wall::None, Wall::None) => display_string.push(' '),
                    (Wall::None, Wall::None, Wall::Wall, Wall::Wall) => display_string.push('┌'),
                    (Wall::None, Wall::None, Wall::Wall, Wall::None) => display_string.push(' '),
                    (Wall::None, Wall::None, Wall::None, Wall::Wall) => display_string.push(' '),
                    (Wall::None, Wall::None, Wall::None, Wall::None) => display_string.push(' '),
                }
            }
            display_string.push('\n');
        }

        write!(f, "{}", display_string)
    }
}
