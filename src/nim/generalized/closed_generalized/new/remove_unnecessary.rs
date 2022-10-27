pub fn remove_unnecessary_data(groups: &mut Vec<Vec<u16>>) {
    for i in 0..groups.len() {
        groups[i].sort();
        groups[i].dedup();
    }

    groups.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut i = 0;
    'outer: while i + 1 < groups.len() {
        if groups[i].len() == 0 {
            groups.remove(i);
            continue;
        }

        for potential_bigger_group in &groups[(i + 1)..] {
            if is_subset(&groups[i], potential_bigger_group) {
                groups.remove(i);
                continue 'outer;
            }
        }
        i += 1;
    }
}

fn is_subset(arr1: &Vec<u16>, arr2: &Vec<u16>) -> bool {
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