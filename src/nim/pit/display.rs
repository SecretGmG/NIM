use super::{Pit, Cell, Wall};
use std::{fmt};


impl Pit{
    fn get_cell_at(&self, x :i32, y :i32) -> Cell{
        if x<self.x as i32 && x>=0 && y<self.y as i32 && y>=0 {
            let (cell,_,_) = self.board[x as usize][y as usize];
            return cell;
        }
        return Cell::None;
    }
    fn get_v_wall_at(&self, x :i32, y :i32) -> Wall{
        if y == -1 || y == self.y as i32 {return Wall::None;}
        else if x == -1 {return Wall::Wall;}
        else if x == self.x as i32 -1 {return Wall::Wall;}
        else {
            let (_,v_wall,_) = self.board[x as usize][y as usize];
            return v_wall;
        }
    }
    fn get_h_wall_at(&self, x :i32, y :i32) -> Wall{
        if x == -1 || x == self.x as i32 {return Wall::None;}
        else if y == -1 {return Wall::Wall;}
        else if y == self.y as i32 -1 {return Wall::Wall;}
        else {
            let (_,_,h_wall) = self.board[x as usize][y as usize];
            return h_wall;
        }
    }
}



impl fmt::Display for Pit{



    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{

        let mut display_string = String::from("");

        
        for y in -1..self.y as i32{
            if y != -1{

            for x in -1..self.x as i32{
                
                

                let cell = self.get_cell_at(x, y);
                let v_wall = self.get_v_wall_at(x, y);

                if x != -1 {
                match cell{
                    Cell::None => display_string.push('▓'),
                    Cell::On => display_string.push( 'O'),
                    Cell::Off => display_string.push(' '),
                }}


                match v_wall{
                    Wall::Wall => display_string.push('│'),
                    Wall::None => display_string.push(' '),
                }
            }}

            display_string.push('\n');
            
            for x in -1..self.x as i32{
                
                let v_wall = self.get_v_wall_at(x, y);
                let h_wall = self.get_h_wall_at(x, y);
                
                if x != -1 {
                match h_wall{
                    Wall::Wall => display_string.push('─'),
                    Wall::None => display_string.push(' '),
                }}

                
                let next_v_wall= self.get_v_wall_at(x, y+1);
                let next_h_wall= self.get_h_wall_at(x+1, y);

                match (v_wall, h_wall, next_v_wall, next_h_wall){
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