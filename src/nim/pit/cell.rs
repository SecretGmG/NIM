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
pub const ON: CellWalls = (Cell::On, Wall::None, Wall::None);
pub const OFF: CellWalls = (Cell::Off, Wall::None, Wall::None);
