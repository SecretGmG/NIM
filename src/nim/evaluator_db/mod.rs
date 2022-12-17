use std::{collections::HashMap, fmt::Display, cmp::Ordering};
use crate::{nim::{pit_reconstructor::try_reconstruct}, vec_ops};

use super::generalized::{closed_generalized::ClosedGeneralizedNimGame, GeneralizedNimGame};



pub struct DataBase{
    map : HashMap<ClosedGeneralizedNimGame, u16>
}
impl DataBase{
    fn calculate_nimber(&mut self, game: &ClosedGeneralizedNimGame) -> u16 {
        let unique_child_games = game.get_unique_child_games();
        let nimbers: &mut Vec<u16> = &mut unique_child_games.into_iter().map(|g|self.get_nimber(&g)).collect();
        let nimber = vec_ops::mex(nimbers);
        self.set(game, nimber);
        return nimber;
    }

    pub fn get_nimber_of_closed(&mut self, game: &ClosedGeneralizedNimGame) -> u16 {
        
        if let Some(nimber) = game.get_easy_nimber(){
            return nimber;
        }
        if let Some(nimber) = self.get(game) {
            return nimber;
        }

        return self.calculate_nimber(game);
    }

    pub fn get_nimber(&mut self, game: &GeneralizedNimGame) -> u16 {
        if game.get_parts().len() == 0 {
            return 0;
        }

        let mut nimber = 0;

        for closed in game.get_parts() {
            nimber ^= self.get_nimber_of_closed(closed);
        }

        return nimber;
    }
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
    pub fn get_latex_repr(&self) -> String{

        let mut key_value_pairs: Vec<(ClosedGeneralizedNimGame, u16)> = self.map.clone().into_iter().collect();
        key_value_pairs.sort_by(k_v_comparer);

        let mut str = String::new();
        str.push_str("\\centering\n");
        let mut current_node_count = 0;
        for (g, nimber) in key_value_pairs{

            if g.get_node_count() != current_node_count{
                current_node_count = g.get_node_count();
                str.push_str("\\section{nodes: ");
                str.push_str(&*current_node_count.to_string());
                str.push_str("}\n");
            }

            let latex_repr = match try_reconstruct(&g){
                Some(p) => p.get_latex_repr(),
                None => continue,
            };

            str.push_str(&latex_repr);
            str.push_str(&nimber.to_string());
            str.push('\n');

        }
        return str;

    }
}
fn k_v_comparer((key1, val1): &(ClosedGeneralizedNimGame, u16), (key2, val2): &(ClosedGeneralizedNimGame, u16)) -> Ordering{
    match key1.get_node_count().cmp(&key2.get_node_count()) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => return val1.cmp(&val2),
    }
}
impl Display for DataBase{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        

        let mut str = String::from("");

        for key_value_pair in &self.map{

            let (g, nimber) = key_value_pair;

            str.push_str("\nnimber:");
            str.push_str(&nimber.to_string());
            str.push('\n');
            str.push_str(&g.to_string());
            str.push_str("\n----------------------\n")
        }

        write!(f, "{}", str)
    }
}