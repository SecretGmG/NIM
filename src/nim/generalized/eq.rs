use super::GeneralizedNimGame;

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


