use std::{collections::HashMap, cmp::{min, Ordering}};

use super::GeneralizedNimGame;

impl GeneralizedNimGame {
    ///creates an empty GeneralizedNimGame
    pub fn empty() -> GeneralizedNimGame {
        return GeneralizedNimGame {
            groups: Vec::new(),
            neighbours: Vec::new(),
            nodes: 0,
        };
    }
    ///creates and simplifies a GeneralizedNimGame from a vec<vec<u16>>
    pub fn new(groups: Vec<Vec<u16>>) -> GeneralizedNimGame {
        let mut groups = groups;

        let nodes = Self::flatten_indecies_get_node_count(&mut groups);

        Self::remove_unneccesary_data(&mut groups);

        let mut neighbours = Self::build_neighbours(&groups, nodes);

        Self::assign_indecies(&mut groups, &mut neighbours, nodes);
        return GeneralizedNimGame {
            groups: groups,
            nodes: nodes,
            neighbours: neighbours,
        };
    }

    fn build_neighbours(groups: &Vec<Vec<u16>>, nodes: u16) -> Vec<Vec<u16>> {
        let mut neighbours = vec![];

        for node in 0..nodes {
            let mut nodes_neighbours = vec![];

            for group in groups {
                if group.contains(&node) {
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

    fn remove_unneccesary_data(groups: &mut Vec<Vec<u16>>) {
        for i in 0..groups.len() {
            groups[i].dedup();
            groups[i].sort();
        }

        groups.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut i = 0;
        'outer: while i + 1 < groups.len() {
            if groups[i].len() == 0 {
                groups.remove(i);
                continue;
            }

            for j in (i + 1)..groups.len() {
                if Self::contains_all(&groups[i], &groups[j]) {
                    groups.remove(i);
                    continue 'outer;
                }
            }

            i += 1;
        }
    }

    fn contains_all(arr1: &Vec<u16>, arr2: &Vec<u16>) -> bool {
        let mut index1 = 0;
        let mut index2 = 0;
        while index1 < arr1.len() && index2 < arr2.len() {
            if arr1[index1] < arr2[index2] {
                break;
            }

            if arr1[index1] == arr2[index2] {
                index1 += 1;
                index2 += 1;
            } else {
                index2 += 1;
            }
        }
        let result = index1 == arr1.len();

        return result;
    }

    ///flattens the indecies and then returns the nr of nodes
    fn flatten_indecies_get_node_count(groups: &mut Vec<Vec<u16>>) -> u16 {
        let indecies = Self::get_indecies(groups);
        let map = Self::get_map_of_indecies(&indecies);
        Self::reassign_indecies(groups, &map);
        return indecies.len() as u16;
    }
    ///gets all indecies in a sorted vec
    fn get_indecies(groups: &Vec<Vec<u16>>) -> Vec<u16> {
        let mut indecies = vec![];

        //generate map of new indecies
        for i in 0..groups.len() {
            for j in 0..groups[i].len() {
                let index = groups[i][j];
                indecies.push(index);
            }
        }

        indecies.sort();
        indecies.dedup();
        return indecies;
    }
    ///generates a map to reasign the flattened indecies
    fn get_map_of_indecies(indecies: &Vec<u16>) -> HashMap<u16, u16> {
        let mut map: HashMap<u16, u16> = HashMap::new();

        for i in 0..indecies.len() {
            map.insert(indecies[i], i as u16);
        }
        return map;
    }
    ///reassigns the indecies of a groups via a map
    fn reassign_indecies(groups: &mut Vec<Vec<u16>>, map: &HashMap<u16, u16>) {
        for i in 0..groups.len() {
            for j in 0..groups[i].len() {
                groups[i][j] = map[&groups[i][j]];
            }
        }
    }

    ///Reasigns indecies in a way that doesn't alter the data to standartisize the format
    fn assign_indecies(
        groups: &mut Vec<Vec<u16>>,
        neighbours: &mut Vec<Vec<u16>>,
        nodes: u16,
    ) {
        //generate a comparer according to numbers of neighbours
        let comparer = &mut Self::get_comparer(neighbours, nodes);

        let mut refrence = Self::ascending_vec_u16(nodes);
        refrence.sort_by_key(|v| comparer[*v as usize]);
        let permutation = Self::get_permutation(refrence);

        for i in 0..groups.len() {
            for j in 0..groups[i].len() {
                groups[i][j] = permutation[groups[i][j] as usize];
            }
            groups[i].sort();
        }
        groups.sort_by(Self::group_comparer);
        for i in 0..nodes {
            for j in 0..neighbours[i as usize].len() {
                neighbours[i as usize][j] = permutation[neighbours[i as usize][j] as usize];
            }
            neighbours[i as usize].sort();
        }
    }
    ///gets a comparer initialized with the nr of neighbours of each node
    fn get_comparer(neighbours: &mut Vec<Vec<u16>>, nodes: u16) -> Vec<u128> {
        let comparer = &mut vec![0; nodes as usize];

        for node in 0..nodes {
            comparer[node as usize] = neighbours[node as usize].len() as u128;
        }

        let comparer = &mut Self::propagate_comparer(comparer, neighbours);
        let comparer = &mut Self::propagate_comparer(comparer, neighbours);
        let comparer = &mut Self::differentiate_groups_in_comparer(comparer, neighbours);
        let comparer = &mut Self::propagate_comparer(comparer, neighbours);
        let comparer = Self::propagate_comparer(comparer, neighbours);

        return comparer;
    }
    ///improves a node comparer by appending the sum of all neighbouring values to a certain value
    ///this increases the nr of used bits by 16
    fn propagate_comparer(comparer: &mut Vec<u128>, neighbours: &mut Vec<Vec<u16>>) -> Vec<u128> {
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
        comparer: &mut Vec<u128>,
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
}
