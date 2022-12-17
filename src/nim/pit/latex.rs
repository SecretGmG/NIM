use super::{Pit, cell::{Cell, CellWalls}};


impl Pit{
    ///returns a string that can be put into latex
    ///Walls are not represented!
    pub fn get_latex_repr(&self) -> String{
    
    let mut str = String::new();
    str.push_str("\\begin{tabular}{|");

    for _ in 0..self.x {str.push_str("m{0.2cm}");}

    str.push_str("|}\\hline\n");
    for j in 0..self.y{
        for i in 0..self.x{
            if i != 0 {str.push('&');}
            let cell_walls: CellWalls = self.board[i as usize][j as usize];
            let (cell,_,_) = cell_walls;
            match cell{
                Cell::Off => str.push(' '),
                Cell::On => str.push('O'),
            }
        }
        str.push_str("\\\\\n");
    }
    str.push_str("\\hline\\end{tabular}");
    return str;





    }

    
}