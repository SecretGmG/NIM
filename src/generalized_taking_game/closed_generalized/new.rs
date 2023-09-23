use super::ClosedTakingGamePart;
use std::cmp::Ordering;
use std::collections::HashMap;

impl ClosedTakingGamePart {
    #[allow(dead_code)]
    ///creates an empty GeneralizedNimGame
    pub fn empty() -> ClosedTakingGamePart {
        return ClosedTakingGamePart {
            sets_of_nodes: Vec::new(),
            set_indices: Vec::new(),
            nodes: 0,
        };
    }
    ///creates and simplifies a GeneralisedTakingGame from a vec<vec<usize>>
    pub fn new(mut sets_of_nodes: Vec<Vec<usize>>) -> ClosedTakingGamePart {
        let nodes = Self::flatten_and_get_node_count(&mut sets_of_nodes);
        Self::remove_redundant_sets(&mut sets_of_nodes);
        Self::sort_sets_of_nodes(&mut sets_of_nodes, nodes);
        let set_indices = Self::generate_set_indices(&sets_of_nodes, nodes);

        return ClosedTakingGamePart {
            sets_of_nodes,
            set_indices,
            nodes: nodes,
        };
    }
    ///flattens the indecies and then returns the nr of nodes
    fn flatten_and_get_node_count(sets_of_nodes: &mut Vec<Vec<usize>>) -> usize {
        let mut indices: Vec<usize> = sets_of_nodes.iter().flatten().copied().collect();
        indices.sort_unstable();
        indices.dedup();
        let mut map = HashMap::new();
        for i in 0..indices.len() {
            map.insert(indices[i], i );
        }
        {
            for i in 0..sets_of_nodes.len() {
                for j in 0..sets_of_nodes[i].len() {
                    sets_of_nodes[i][j] = *map.get(&sets_of_nodes[i][j]).expect("the indices were generated correctly");
                }
            }
        };
        return indices.len() ;
    }
    ///removes sets that are totally contained in other sets
    fn remove_redundant_sets(sets_of_nodes: &mut Vec<Vec<usize>>) {
        for i in 0..sets_of_nodes.len() {
            sets_of_nodes[i].sort_unstable();
            sets_of_nodes[i].dedup();
        }
        sets_of_nodes.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut i = 0;
        'outer: while i + 1 < sets_of_nodes.len() {
            if sets_of_nodes[i].len() == 0 {
                sets_of_nodes.remove(i);
                continue;
            }

            for potential_bigger_group in &sets_of_nodes[(i + 1)..] {
                if is_subset(&sets_of_nodes[i], potential_bigger_group) {
                    sets_of_nodes.remove(i);
                    continue 'outer;
                }
            }
            i += 1;
        }
    }
    pub fn sort_sets_of_nodes(sets_of_nodes: &mut Vec<Vec<usize>>, nodes: usize) {
        loop {
            Self::partially_sort_sets_of_nodes(sets_of_nodes);
            let permutation = Self::generate_index_remaping(sets_of_nodes, nodes);
            if {
                (0..permutation.len())
                    .zip(&permutation)
                    .all(|(a, b)| a  == *b)
            } {
                return;
            }
            for i in 0..sets_of_nodes.len() {
                for j in 0..sets_of_nodes[i].len() {
                    sets_of_nodes[i][j] = permutation[sets_of_nodes[i][j] ];
                }
            }
        }
    }
    fn generate_index_remaping(groups: &Vec<Vec<usize>>, nodes: usize) -> Vec<usize> {
        let mut refrences: Vec<usize> = (0..nodes).collect();
        let set_indices = &Self::generate_set_indices(groups, nodes);
        refrences.sort_by(|a, b| node_comparer(*a, *b, set_indices));
        let permutation = inverse_permutation(refrences);
        return permutation;
    }

    fn partially_sort_sets_of_nodes(sets_of_nodes: &mut Vec<Vec<usize>>) {
        for i in 0..sets_of_nodes.len() {
            sets_of_nodes[i].sort();
        }
        sets_of_nodes.sort_by(compare_sorted);
    }
    ///gets a vec with each index storing all the groups that contain the node with that index
    pub fn generate_set_indices(groups: &Vec<Vec<usize>>, nodes: usize) -> Vec<Vec<usize>> {
        let mut group_indecies = vec![vec![]; nodes ];

        for i in 0..groups.len() {
            for node in &groups[i] {
                group_indecies[*node ].push(i );
            }
        }
        for i in 0..nodes {
            group_indecies[i ].sort_unstable();
        }
        return group_indecies;
    }
}
//checks if two a sorted array is a subset of another sorted array
fn is_subset(arr1: &Vec<usize>, arr2: &Vec<usize>) -> bool {
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
///calculates the inverse permutation of a given input permutation
///undefined behaviour if the input is not a permutation
fn inverse_permutation(refrences: Vec<usize>) -> Vec<usize> {
    let mut perm = vec![0; refrences.len()];
    for i in 0..refrences.len() {
        perm[refrences[i] ] = i ;
    }
    return perm;
}
fn node_comparer(a: usize, b: usize, set_indices: &Vec<Vec<usize>>) -> Ordering {
    return compare_sorted(&set_indices[a ], &set_indices[b ]);
}
///compares two sorted vecs, first by length, then by their elements
pub fn compare_sorted<T: Ord>(vec1: &Vec<T>, vec2: &Vec<T>) -> Ordering {
    match vec1.len().cmp(&vec2.len()) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => (),
    }
    for i in 0..vec1.len() {
        match vec1[i].cmp(&vec2[i]) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }
    }
    return Ordering::Equal;
}
