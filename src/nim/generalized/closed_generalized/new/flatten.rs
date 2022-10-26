use std::collections::HashMap;

///flattens the indecies and then returns the nr of nodes
pub fn flatten_and_get_node_count(groups: &mut Vec<Vec<u16>>) -> u16 {
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
