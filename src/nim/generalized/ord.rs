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