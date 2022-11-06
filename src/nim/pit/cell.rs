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
pub const On: CellWalls = (Cell::On, Wall::None, Wall::None);
pub const Off: CellWalls = (Cell::Off, Wall::None, Wall::None);
