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