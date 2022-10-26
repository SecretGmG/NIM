pub mod closed_generalized;
pub mod data_base;
mod impls; 
use closed_generalized::ClosedGeneralizedNimGame;
use data_base::DataBase;

use self::closed_generalized::new;

///A generalized version of any impartial "taking game"
///implements many tools to effitiently find the nimber of any complex taking game

pub struct GeneralizedNimGame{
    parts :Vec<ClosedGeneralizedNimGame>
}

impl GeneralizedNimGame{

    pub fn new(groups: Vec<Vec<u16>>) -> GeneralizedNimGame{

        let prime_groups = split(groups);

        let mut parts: Vec<ClosedGeneralizedNimGame> = prime_groups.into_iter().map(ClosedGeneralizedNimGame::new).collect();

        parts.sort();

        return GeneralizedNimGame{parts};
    }
    pub fn get_nimber(&self, prev_seen: &mut DataBase) -> u16{
        if self.parts.len() == 0 {return 0;}
        if self.is_symmetric() {return 0;}


        let mut nimber = 0;

        for closed in &self.parts{
            nimber ^= closed.get_nimber(prev_seen);
        }

        return nimber;
    }
    pub fn get_unique_child_games(&self) -> Vec<GeneralizedNimGame>{

        todo!();
    }
    pub fn is_symmetric(&self) -> bool{
        
        let mut i = 0;
        while i < self.parts.len() {
            if self.parts[i].is_symmetric() { i+= 1;}
            else{
                if i+1 == self.parts.len() {return false;}

                if self.parts[i] != self.parts[i+1] {return false;}

                i+= 2;
            }
        }
        return true;
    }



}
fn split(groups: Vec<Vec<u16>>) -> Vec<Vec<Vec<u16>>>{
    
    let mut groups = groups; 
    let mut parts = vec![];

    'outer: while groups.len() > 0{

        for i in 0..parts.len(){
            if contains_any(&groups[0], &parts[i]){
                parts[i].push(groups.swap_remove(0));
                
                continue 'outer;
            }
        }
        parts.push(vec![groups.swap_remove(0)]);
    }
    
    return parts;
}
fn contains_any(groups: &Vec<u16>, part: &Vec<Vec<u16>>) -> bool{

    for other_group in part{
        if contains_any_sorted(groups, other_group) {return true;}
    }
    return false;
}
fn contains_any_sorted(a: &Vec<u16>, b: &Vec<u16>) -> bool{
    let mut i = 0;
    let mut j = 0;
    
    while i<a.len() && j<b.len(){

        if a[i] < b[j]{
            i += 1;
        }
        else if a[i] > b[j]{
            j += 1;
        }
        else{
            return true;
        }
    }
    return false;
}
