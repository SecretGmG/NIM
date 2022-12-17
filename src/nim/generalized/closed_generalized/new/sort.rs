use std::cmp::Ordering;

use crate::vec_ops::{self, ascending_vec_u16};

pub fn sort(groups: &mut Vec<Vec<u16>>, nodes: u16){
    //here is a lot of potential for speed increase
    //store which nodes where "equal, then only look at those"
    //like this it should also be easily possible to do many passes
    //use the permuatation to generate the new group_indecies    
    loop {
        sort_groups(groups);
        let permutation = generate_permutation(groups, nodes);
        if is_unit_permutation(&permutation) {return;}
        apply_permutation(groups, permutation);
    }

    //for 4x5 3343 entries ~42 s
    
    /*
    sort_groups(groups);
    let permutation = generate_permutation(groups, nodes);
    apply_permutation(groups, permutation);
    sort_groups(groups);
    let permutation = generate_permutation(groups, nodes);
    apply_permutation(groups, permutation);
    sort_groups(groups);
    //for 4x5: 3714 entries 48 s
    */

    //for 4x5 4
}

fn is_unit_permutation(permutation: &Vec<u16>) -> bool{
    for i in 0..permutation.len(){
        if permutation[i] != i as u16{
            return false;
        }
    }
    return true;
}

fn apply_permutation(groups: &mut Vec<Vec<u16>>, permutation: Vec<u16>){
    for i in 0..groups.len(){
        for j in 0..groups[i].len(){
            groups[i][j] = permutation[groups[i][j] as usize];
        }
    }
}
fn sort_groups(groups: &mut Vec<Vec<u16>>){
    for i in 0..groups.len(){
        groups[i].sort();
    }
    groups.sort_by(vec_ops::compare_sorted);
}
fn generate_permutation(groups: &Vec<Vec<u16>>, nodes: u16) -> Vec<u16> {
    let mut refrences = ascending_vec_u16(nodes);

    let group_indecies = &get_group_indecies(groups, nodes);

    refrences.sort_by(|a, b| node_comparer(*a, *b, group_indecies));

    let permutation = get_permutation(refrences);
    return permutation;

}
fn node_comparer(a: u16, b: u16, group_indecies: &Vec<Vec<u16>>) -> Ordering{
    return vec_ops::compare_sorted(&group_indecies[a as usize],&group_indecies[b as usize]);
}
///calculates the inverse permutation of a given input permutation
///undefined behaviour if the input is not a permutation
fn get_permutation(refrences: Vec<u16>) -> Vec<u16> {
    let mut perm = vec![0; refrences.len()];

    for i in 0..refrences.len() {
        perm[refrences[i] as usize] = i as u16;
    }
    return perm;
}

///gets a vec with each index storing all the groups that contain the node with that index
pub fn get_group_indecies(groups: &Vec<Vec<u16>>, nodes: u16) -> Vec<Vec<u16>>{

    let mut group_indecies = vec![vec![];nodes as usize];

    for i in 0..groups.len(){
        for node in &groups[i]{
            group_indecies[*node as usize].push(i as u16);
        }
    }
    for i in 0..nodes{
        group_indecies[i as usize].sort_unstable();
    }
    return group_indecies;
}