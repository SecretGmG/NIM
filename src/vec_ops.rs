use std::cmp::Ordering;

pub fn contains_any_sorted(a: &Vec<u16>, b: &Vec<u16>) -> bool {
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
#[allow(dead_code)]
pub fn insert_sorted_deduped<T: Ord>(vec: &mut Vec<T>, val: T) -> Result<usize, usize> {
    match vec.binary_search(&val) {
        Ok(index) => return Err(index),
        Err(index) => {
            vec.insert(index, val);
            return Ok(index);
        }
    }
}
pub fn sorted_without(vec1: &Vec<u16>, vec2: &Vec<u16>) -> Vec<u16> {
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
/// generates a vec filled with u16s in ascending order
/// # Examples
/// ```
/// let size: u16 = 6;
/// let ascending_vec = ascending_vec_u16(6);
/// assert_eq!(vec![0,1,2,3,4,5], ascending_vec_u16(6));
/// ```
pub fn ascending_vec_u16(len: u16) -> Vec<u16> {
    let mut r = vec![];
    for i in 0..len {
        r.push(i);
    }
    return r;
}
///minimal excluded
///calculates the smallest number not in the list
pub fn mex(nums: &mut Vec<u16>) -> u16 {
    nums.sort_unstable();
    let mut r: u16 = 0;
    let mut i = 0;
    while i < nums.len() {
        if r < nums[i] {
            //r is the smallest not in the list
            break;
        }
        if r == nums[i] {
            //r is in the list
            r += 1;
        }
        i += 1;
    }
    return r;
}
///similiar to dedup
///removes both of the values if they are equal
pub fn remove_pairs_sorted<T: Eq>(vec: &mut Vec<T>) {
    let mut i = vec.len();
    while i > 1 {
        if vec[i - 1] == vec[i - 2] {
            vec.remove(i - 1);
            vec.remove(i - 2);
            i -= 2;
        } else {
            i -= 1;
        }
    }
}
#[allow(dead_code)]
pub fn compare_sorted2<T: Ord>(vec1: &Vec<Vec<Vec<T>>>, vec2: &Vec<Vec<Vec<T>>>) -> Ordering {
    match vec1.len().cmp(&vec2.len()) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => (),
    }

    for i in 0..vec1.len() {
        match vec1[i].len().cmp(&vec2[i].len()) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }
    }

    for i in 0..vec1.len() {
        match compare_sorted1(&vec1[i], &vec2[i]) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }
    }
    return Ordering::Equal;
}

pub fn compare_sorted1<T: Ord>(vec1: &Vec<Vec<T>>, vec2: &Vec<Vec<T>>) -> Ordering {
    match vec1.len().cmp(&vec2.len()) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => (),
    }

    for i in 0..vec1.len() {
        match vec1[i].len().cmp(&vec2[i].len()) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }
    }

    for i in 0..vec1.len() {
        match compare_sorted(&vec1[i], &vec2[i]) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }
    }
    return Ordering::Equal;
}

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
