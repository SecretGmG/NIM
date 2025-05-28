use std::cmp::Ordering;

//checks if two a sorted array is a subset of another sorted array
pub fn is_subset(arr1: &Vec<usize>, arr2: &Vec<usize>) -> bool {
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
pub fn inverse_permutation(refrences: Vec<usize>) -> Vec<usize> {
    let mut perm = vec![0; refrences.len()];
    for i in 0..refrences.len() {
        perm[refrences[i] ] = i ;
    }
    return perm;
}
pub fn node_comparer(a: usize, b: usize, set_indices: &Vec<Vec<usize>>) -> Ordering {
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
///retures true if a and b share any elements
pub fn have_common_element(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            i += 1;
        } else if a[i] > b[j] {
            j += 1;
        } else {
            return true;
        }
    }
    return false;
}
pub fn remove_subset(vec1: &Vec<usize>, vec2: &Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;
    let mut r = vec![];
    while i < vec1.len() {
        if j >= vec2.len() {
            r.push(vec1[i]);
            i += 1;
        } else if vec1[i] < vec2[j] {
            r.push(vec1[i]);
            i += 1;
        } else if vec1[i] == vec2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    return r;
}