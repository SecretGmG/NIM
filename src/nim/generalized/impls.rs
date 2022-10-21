use super::GeneralizedNimGame;

impl GeneralizedNimGame{
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.groups.len() != other.groups.len() {return self.groups.len().cmp(&other.groups.len());}
        for i in 0..self.groups.len(){
            match self.groups[i].len().cmp(&other.groups[i].len()){
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                std::cmp::Ordering::Equal => {

                    for j in 0..self.groups[i].len(){

                        match self.groups[i][j].cmp(&other.groups[i][j]){

                            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                            std::cmp::Ordering::Equal => ()
                        }
                    }
                }
            }
        }
        return std::cmp::Ordering::Equal;
        
    }
}

impl PartialEq for GeneralizedNimGame{
    fn eq(&self, other: &Self) -> bool {
        groups_eq(&self.groups, &other.groups)
    }   
}
fn groups_eq(a: &Vec<Vec<u16>>, b: &Vec<Vec<u16>>) -> bool{
    if a.len() != b.len() {return false;}
    for i in 0..a.len(){
        if a[i].len() != b[i].len() {return false;}
 
        for j in 0..a[i].len(){
            if a[i][j] != b[i][j] {return false;}
        }
    }
    return true;
}
impl Clone for GeneralizedNimGame{
    fn clone(&self) -> Self {
        Self { groups: self.groups.clone(), neighbours: self.neighbours.clone(), nodes: self.nodes.clone() }
    }
}
impl core::hash::Hash for GeneralizedNimGame{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nodes.hash(state);
        for i in 0..self.nodes{
            for node in &self.neighbours[i as usize]{
                (*node).hash(state);
            }
        }
    }
}