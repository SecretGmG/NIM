use std::{vec, collections::HashMap, cmp::Ordering};

pub mod display;
pub mod symmetries;
pub mod moves;
pub mod nimber;
pub mod impls;

///A generalized version of any impartial "taking game"
///implements many tools to effitiently find the nimber of any complex taking game
#[derive(Debug,Eq)]
pub struct GeneralizedNimGame{
    groups :Vec<Vec<u16>>,
    /// neighbours[i] stores all nodes neighbouring i in ascending order (deduped)
    neighbours :Vec<Vec<u16>>,
    ///the amount of nodes in groups <=> the biggest index
    nodes :u16
}

//Constructors and basic functions for the GeneralizedNimGame
impl GeneralizedNimGame{
    /// gets all neighbours of a given node
    pub fn get_neighbours(&self, node :u16) -> &Vec<u16>{
        return &self.neighbours[node as usize];
    }
    
    ///creates an empty GeneralizedNimGame
    pub fn empty() -> GeneralizedNimGame{
        return GeneralizedNimGame{
            groups: Vec::new(),
            neighbours: Vec::new(),
            nodes: 0
        }
    }
    
    ///creates and simplifies a GeneralizedNimGame from a vec<vec<u16>>
    pub fn new(vec_of_groups :Vec<Vec<u16>>) -> GeneralizedNimGame{         
        let mut groups = vec_of_groups;
        

        let nodes = Self::flatten_indecies_get_node_count(&mut groups);
        
        Self::remove_unneccesary_data(&mut groups);
                
        let mut neighbours = Self::build_neighbours(&groups, nodes);

        Self::assign_indecies(&mut groups, &mut neighbours, nodes);
        return GeneralizedNimGame{groups: groups,nodes:  nodes,neighbours: neighbours};
    }
    
    fn build_neighbours(vec_of_groups :&Vec<Vec<u16>>, nodes: u16) -> Vec<Vec<u16>>{
        let mut neighbours = vec![];
    
        for node in 0..nodes{
            let mut nodes_neighbours = vec![];
            
            for group in vec_of_groups {
                if group.contains(&node){
                    nodes_neighbours.append(&mut group.clone());
                }
            }
            nodes_neighbours.sort();
            nodes_neighbours.dedup();

            nodes_neighbours.remove(nodes_neighbours.binary_search(&node).ok().unwrap());

            neighbours.push(nodes_neighbours);
        }
        return neighbours;
    }
          
    fn remove_unneccesary_data(vec_of_groups :&mut Vec<Vec<u16>>){
        for i in 0..vec_of_groups.len()
        {
            vec_of_groups[i].dedup();
            vec_of_groups[i].sort();
        }

        vec_of_groups.sort_by(|a,b|a.len().cmp(&b.len()));

        let mut i = 0;
        'outer: while i+1<vec_of_groups.len() {
            if vec_of_groups[i].len() == 0 {
                vec_of_groups.remove(i);
                continue;
            }

            for j in (i+1)..vec_of_groups.len(){
                
                if Self::contains_all(&vec_of_groups[i], &vec_of_groups[j]) {
                    vec_of_groups.remove(i);
                    continue 'outer;
                }
            }

            i+=1;
        }
    }

    fn contains_all(arr1 : &Vec<u16>, arr2 : &Vec<u16>) -> bool{
        
        let mut index1 = 0;
        let mut index2 = 0;
        while index1<arr1.len() && index2<arr2.len() {
            
            if arr1[index1] < arr2[index2]{
                break;
            }

            if arr1[index1] == arr2[index2]{
                index1+=1;
                index2+=1;
            }
            else{
                index2+=1;
            }
        }
        let result = index1 == arr1.len();

        return result;
    }
    

    fn flatten_indecies_get_node_count(vec_of_groups :&mut Vec<Vec<u16>>) -> u16{

        let mut indecies = vec![];
        
        //generate map of new indecies
        for i in 0..vec_of_groups.len(){
            for j in 0..vec_of_groups[i].len(){

                let index = vec_of_groups[i][j]; 
                indecies.push(index);
            }
        }
        
        indecies.sort();
        indecies.dedup();

        let nodes = indecies.len();
        
        let mut map :HashMap<u16, u16> = HashMap::new();

        for i in 0..indecies.len(){
            map.insert(indecies[i], i as u16);
        }

        //assign new indecies
        for i in 0..vec_of_groups.len(){
            for j in 0..vec_of_groups[i].len(){


                vec_of_groups[i][j] = map[&vec_of_groups[i][j]];
            }
        }
        
        return nodes as  u16;
    }
    
    
    //Reasigns indecies in a way that doesn't alter the data to standartisize the format
    fn assign_indecies(vec_of_groups :&mut Vec<Vec<u16>>, neighbours :&mut Vec<Vec<u16>>, nodes :u16)
    {
        //Count Occurences
        let mut comparer: Vec<u64> = vec![0; nodes as usize];
        for i in 0..vec_of_groups.len()
        {
            for node in &vec_of_groups[i]{
                comparer[*node as usize] += 1;
            }
        }
        let mut better_comparer = vec![0; nodes as usize];
        for i in 0..comparer.len(){
            better_comparer[i] = comparer[i]*nodes as u64;
            for j in 0..neighbours[i].len(){
                better_comparer[i] += comparer[neighbours[i][j] as usize];
            }
        }
        
        
        (comparer, better_comparer) = (better_comparer, comparer);

        for i in 0..comparer.len(){
            better_comparer[i] = comparer[i]*nodes as u64;
            for j in 0..neighbours[i].len(){
                better_comparer[i] += comparer[neighbours[i][j] as usize];
            }
        }

        let mut refrence = Self::ascending_vec_u16(nodes);
        refrence.sort_by_key(|v| better_comparer[*v as usize]);

        let permutation = Self::get_permutation(refrence);


        for i in 0..vec_of_groups.len(){
            for j in 0..vec_of_groups[i].len(){
                vec_of_groups[i][j] = permutation[vec_of_groups[i][j] as usize];
            }
            vec_of_groups[i].sort();
        }
        vec_of_groups.sort_by(Self::group_comparer);
        for i in 0..nodes{
            for j in 0..neighbours[i as usize].len(){
                neighbours[i as usize][j] = permutation[neighbours[i as usize][j] as usize];
            }
            neighbours[i as usize].sort();
        }
    }

    fn get_permutation(refrences :Vec<u16>) -> Vec<u16>{
        
        let mut perm = vec![0;refrences.len()];
        
        for i in 0..refrences.len(){
            perm[refrences[i] as usize] = i as u16;
        }
        return perm;
    }

    fn group_comparer(vec1 :&Vec<u16>, vec2 :&Vec<u16> ) -> Ordering{
        
        match vec1.len().cmp(&vec2.len()){
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        }

        for i in 0..vec1.len(){
            match vec1[i].cmp(&vec2[i]){
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
            }
        }

        return Ordering::Equal;
    }
    fn ascending_vec_u16(len :u16) -> Vec<u16>{
        let mut r = vec![];
        for i in 0..len{
            r.push(i);
        }
        return r;
    }

}


