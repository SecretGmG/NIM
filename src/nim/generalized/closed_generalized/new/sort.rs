use std::cmp::{Ordering, min};

pub fn sort(groups: &mut Vec<Vec<u16>>, neighbours: &mut Vec<Vec<u16>>, nodes: u16) {
    
    let permutation = generate_permutation(neighbours, nodes);

    for i in 0..groups.len() {
        for j in 0..groups[i].len() {
            groups[i][j] = permutation[groups[i][j] as usize];
        }
        groups[i].sort();
    }
    groups.sort_by(group_comparer);
    for i in 0..nodes {
        for j in 0..neighbours[i as usize].len() {
            neighbours[i as usize][j] = permutation[neighbours[i as usize][j] as usize];
        }
        neighbours[i as usize].sort();
    }
}
fn generate_permutation2(neighbours: &mut Vec<Vec<u16>>, nodes: u16) -> Vec<u16>{

    let mut refrences = ascending_vec_u16(nodes);

    refrences.sort_by(|a,b| node_comparer(*a,*b, neighbours, nodes, 4));

    let permutation = get_permutation(refrences);
    return permutation;
}


fn node_comparer(a: u16, b: u16, neighbours: &mut Vec<Vec<u16>>, nodes: u16, depth: u16) -> Ordering{

    let mut comparer: Vec<Vec<u16>> = vec![];


    return Ordering::Equal;

    //return node_with_eq_nr_of_neighbours_comparer(a,b,neighbours,nodes,depth);
}
fn node_with_eq_nr_of_neighbours_comparer(a: u16, b: u16, neighbours: &mut Vec<Vec<u16>>, nodes: u16, depth: u16) -> Ordering{
    todo!();
    
}








fn generate_permutation(neighbours: &mut Vec<Vec<u16>>, nodes: u16) -> Vec<u16> {
    //generate a comparer according to numbers of neighbours
    let comparer = &mut get_comparer(neighbours, nodes);
    let mut refrences = ascending_vec_u16(nodes);
    refrences.sort_by_key(|v| comparer[*v as usize]);
    let permutation = get_permutation(refrences);
    return permutation;
}
///gets a comparer initialized with the nr of neighbours of each node
fn get_comparer(neighbours: &mut Vec<Vec<u16>>, nodes: u16) -> Vec<u128> {
    let mut comparer = vec![0; nodes as usize];

    for node in 0..nodes {
        comparer[node as usize] = neighbours[node as usize].len() as u128;
        comparer[node as usize] *= comparer[node as usize];

        //comparer[node as usize] *= comparer[node as usize];
    }
    //16/128 therefore 1/8
    let comparer = propagate_comparer(comparer, neighbours); //2/8
    let comparer = propagate_comparer(comparer, neighbours); //3/8
    let comparer = propagate_comparer(comparer, neighbours); //4/8
    let comparer = differentiate_groups_in_comparer(comparer, neighbours); //5/8
    let comparer = propagate_comparer(comparer, neighbours); //6/8
    let comparer = propagate_comparer(comparer, neighbours); //7/8
    let comparer = propagate_comparer(comparer, neighbours); //8/8

    return comparer;
}
///improves a node comparer by appending the sum of all neighbouring values to a certain value
///this increases the nr of used bits by 16
fn propagate_comparer(comparer: Vec<u128>, neighbours: &mut Vec<Vec<u16>>) -> Vec<u128> {
    let mut better_comparer = vec![0; comparer.len()];
    for node in 0..comparer.len() {
        better_comparer[node] = comparer[node] << 16 as u128;
        for neighbour in &neighbours[node] {
            better_comparer[node] += comparer[*neighbour as usize];
        }
    }
    return better_comparer;
}
///ensures that nodes with the exact same properties get sorted diffrently
///this is achieved by appending a value unique to each groups
///this increases the nr of used bits by 16
fn differentiate_groups_in_comparer(
    comparer: Vec<u128>,
    neighbours: &mut Vec<Vec<u16>>,
) -> Vec<u128> {
    let mut better_comparer = vec![0; comparer.len()];
    for node in 0..comparer.len() {
        better_comparer[node] = comparer[node] << 16 as u128;
        if neighbours[node].len() == 0 {
            continue;
        }
        better_comparer[node] += min(neighbours[node][0], node as u16) as u128
    }
    return better_comparer;
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

