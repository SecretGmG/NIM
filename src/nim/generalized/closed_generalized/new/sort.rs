use std::cmp::{min, Ordering};

use crate::nim::vec_ops;

pub fn sort(groups: &mut Vec<Vec<u16>>, nodes: u16){
    let permutation = generate_permutation(groups, nodes);
    apply_permutation(groups, permutation);
}

fn apply_permutation(groups: &mut Vec<Vec<u16>>, permutation: Vec<u16>){
    for i in 0..groups.len(){
        for j in 0..groups[i].len(){
            groups[i][j] = permutation[groups[i][j] as usize];
        }
    }
}
fn generate_permutation(groups: &Vec<Vec<u16>>, nodes: u16) -> Vec<u16> {
    let mut refrences = ascending_vec_u16(nodes);

    refrences.sort_by(|a, b| node_comparer(*a, *b, &get_group_indecies(groups, nodes)));

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
///compares two groups, first by length, then by each element
fn group_comparer(vec1: &Vec<u16>, vec2: &Vec<u16>) -> Ordering {
    match vec1.len().cmp(&vec2.len()) {
        Ordering::Less => return Ordering::Less,
        Ordering::Equal => (),
        Ordering::Greater => return Ordering::Greater,
    }

    for i in 0..vec1.len() {
        match vec1[i].cmp(&vec2[i]) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        }
    }

    return Ordering::Equal;
}
///generates a vec filled with u16s in ascending order
/// # Examples
///```
///assert_eq!(ascending_vec_u16(6), [0,1,2,3,4,5]);
///```
fn ascending_vec_u16(len: u16) -> Vec<u16> {
    let mut r = vec![];
    for i in 0..len {
        r.push(i);
    }
    return r;
}
///gets a vec with each index storing all the groups that contain the node with that index
pub fn get_group_indecies(groups: &Vec<Vec<u16>>, nodes: u16) -> Vec<Vec<u16>>{

    let mut group_indecies = vec![vec![];nodes as usize];

    for i in 0..groups.len(){
        for node in &groups[i]{
            group_indecies[*node as usize].push(i as u16);
        }
    }
    return group_indecies;
}