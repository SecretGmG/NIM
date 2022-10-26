use std::{collections::HashMap, fmt::Display};
use super::closed_generalized::ClosedGeneralizedNimGame;


pub struct DataBase{
    map : HashMap<ClosedGeneralizedNimGame, u16>
}
impl DataBase{

    pub fn get(&self, g: &ClosedGeneralizedNimGame) -> Option<u16>{
        self.map.get(g).copied()
    }
    pub fn set(&mut self, g: &ClosedGeneralizedNimGame, nimber: u16) -> Option<u16>{
        self.map.insert(g.clone(), nimber)
    }
    pub fn new() -> DataBase{
        return DataBase{map: HashMap::new()};
    }
    pub fn len(&self) -> usize{
        return self.map.len();
    }
}
impl Display for DataBase{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        

        let mut str = String::from("");

        for key_value_pair in &self.map{

            let (g, nimber) = key_value_pair;

            str.push_str(&g.to_string());
            str.push_str("\nnimber:");
            str.push_str(&nimber.to_string());
            str.push_str("\n----------------------")
        }

        write!(f, "{}", str)
    }
}