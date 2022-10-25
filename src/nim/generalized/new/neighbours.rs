pub fn generate_all_neighbours(groups: &Vec<Vec<u16>>, nodes: u16) -> Vec<Vec<u16>> {
    let mut neighbours = vec![];

    for node in 0..nodes {
        generate_neighbours_of_node(groups, node, &mut neighbours);
    }
    return neighbours;
}

fn generate_neighbours_of_node(groups: &Vec<Vec<u16>>, node: u16, neighbours: &mut Vec<Vec<u16>>) {
    let mut nodes_neighbours = vec![];
    for group in groups {
        if group.contains(&node) {
            nodes_neighbours.append(&mut group.clone());
        }
    }
    nodes_neighbours.sort();
    nodes_neighbours.dedup();
    //remove the node itself
    nodes_neighbours.remove(nodes_neighbours.binary_search(&node).ok().unwrap());
    neighbours.push(nodes_neighbours);
}