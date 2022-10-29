use std::collections::HashMap;


pub fn flatten(groups: &mut Vec<Vec<u16>>) -> u16{
    let nodes = flatten_and_get_node_count(groups);
    remove_unnecessary_data(groups);
    return nodes;
}


///flattens the indecies and then returns the nr of nodes
fn flatten_and_get_node_count(groups: &mut Vec<Vec<u16>>) -> u16 {
    let indecies = get_indecies(groups);
    let map = get_map_of_indecies(&indecies);
    reassign_indecies(groups, &map);
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
fn remove_unnecessary_data(groups: &mut Vec<Vec<u16>>) {
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