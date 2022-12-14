use crate::nim::generalized::closed_generalized::ClosedGeneralizedNimGame;
use super::{Pit, cell};

impl Pit {
    pub fn try_reconstruct(g: &ClosedGeneralizedNimGame) -> Option<Pit> {
        if let Some(value) = try_get_basic_case(g) {
            return value;
        }
        let (lone_nodes, other_nodes) = get_lone_and_other_nodes(g)?;

        let (h_groups, v_groups) = get_v_h_groups(g, &other_nodes)?;

        let mut board = vec![vec![cell::ON; h_groups.len()]; v_groups.len()];

        set_connected_nodes(other_nodes, &v_groups, &h_groups, g, &mut board);

        append_lone_nodes(lone_nodes, g, v_groups, &mut board, h_groups);

        return Some(Pit::new(board));
    }
}

fn append_lone_nodes(
    lone_nodes: Vec<u16>,
    g: &ClosedGeneralizedNimGame,
    v_groups: Vec<u16>,
    board: &mut Vec<Vec<(cell::Cell, cell::Wall, cell::Wall)>>,
    h_groups: Vec<u16>,
) {
    for lone_node in lone_nodes {
        let gi = g.get_group_indecies()[lone_node as usize][0];
        if let Ok(index) = v_groups.binary_search(&gi) {
            append_row(board, index);
        } else if let Ok(index) = h_groups.binary_search(&gi) {
            append_collumn(board, index);
        } else {
            unreachable!()
        }
    }
}

fn append_collumn(board: &mut Vec<Vec<(cell::Cell, cell::Wall, cell::Wall)>>, index: usize) {
    let mut new_collumn = vec![cell::OFF; board[0].len()];
    new_collumn[index] = cell::ON;
    board.push(new_collumn);
}

fn append_row(board: &mut Vec<Vec<(cell::Cell, cell::Wall, cell::Wall)>>, index: usize) {
    for i in 0..board.len() {
        if i == index {
            board[i].push(cell::ON);
        } else {
            board[i].push(cell::OFF);
        }
    }
}

fn set_connected_nodes(
    other_nodes: Vec<u16>,
    v_groups: &Vec<u16>,
    h_groups: &Vec<u16>,
    g: &ClosedGeneralizedNimGame,
    board: &mut Vec<Vec<(cell::Cell, cell::Wall, cell::Wall)>>,
) {
    for node in other_nodes {
        let (x, y) = get_indecies(v_groups, h_groups, g, node);
        board[x][y] = cell::ON;
    }
}

fn get_v_h_groups(
    g: &ClosedGeneralizedNimGame,
    other_nodes: &Vec<u16>,
) -> Option<(Vec<u16>, Vec<u16>)> {
    let mut h_groups: Vec<u16> = vec![g.get_group_indecies()[other_nodes[0] as usize][0]];
    let mut v_groups: Vec<u16> = vec![];
    let mut nodes_todo = other_nodes.clone();
    while nodes_todo.len() != 0 {
        let mut i = 0;
        while i < nodes_todo.len() {
            let node = nodes_todo[i];
            let g1 = g.get_group_indecies()[node as usize][0];
            let g2 = g.get_group_indecies()[node as usize][1];

            match try_insert_group_indecies(g1, g2, &mut h_groups, &mut v_groups) {
                Some(b) => {
                    if b {
                        nodes_todo.remove(i);
                    } else {
                        i += 1;
                    }
                }
                None => return None,
            }
        }
    }
    Some((h_groups, v_groups))
}

fn get_lone_and_other_nodes(g: &ClosedGeneralizedNimGame) -> Option<(Vec<u16>, Vec<u16>)> {
    let mut lone_nodes = vec![];
    let mut other_nodes = vec![];
    for node in 0..g.get_node_count() {
        match g.get_group_indecies()[node as usize].len() {
            1 => lone_nodes.push(node),
            2 => other_nodes.push(node),
            _ => return None,
        }
    }
    Some((lone_nodes, other_nodes))
}

fn try_get_basic_case(g: &ClosedGeneralizedNimGame) -> Option<Option<Pit>> {
    if g.get_groups().len() == 1 {
        let board = vec![vec![cell::ON; g.get_node_count() as usize]];

        return Some(Some(Pit::new(board)));
    }
    None
}
pub fn try_insert_group_indecies(
    g1: u16,
    g2: u16,
    h_groups: &mut Vec<u16>,
    v_groups: &mut Vec<u16>,
) -> Option<bool> {
    let h1 = h_groups.binary_search(&g1);
    let h2 = h_groups.binary_search(&g2);
    let v1 = v_groups.binary_search(&g1);
    let v2 = v_groups.binary_search(&g2);
    match (h1, h2, v1, v2) {
        (_, _, Ok(_), Ok(_)) => return None,
        (Ok(_), Ok(_), _, _) => return None,
        (Ok(_), Err(_), Err(_), Err(index)) => v_groups.insert(index, g2),
        (Err(_), Ok(_), Err(index), Err(_)) => v_groups.insert(index, g1),
        (Err(_), Err(index), Ok(_), Err(_)) => h_groups.insert(index, g2),
        (Err(index), Err(_), Err(_), Ok(_)) => h_groups.insert(index, g1),
        (Ok(_), Err(_), Err(_), Ok(_)) => return Some(true),
        (Err(_), Ok(_), Ok(_), Err(_)) => return Some(true),
        (_, _, _, _) => return Some(false),
    }
    return Some(true);
}
fn get_indecies(
    v_groups: &Vec<u16>,
    h_groups: &Vec<u16>,
    g: &ClosedGeneralizedNimGame,
    node: u16,
) -> (usize, usize) {
    let g1 = g.get_group_indecies()[node as usize][0];
    let g2 = g.get_group_indecies()[node as usize][1];
    return match v_groups.binary_search(&g1) {
        Ok(x) => (x, h_groups.binary_search(&g2).unwrap()),
        Err(_) => (
            v_groups.binary_search(&g2).unwrap(),
            h_groups.binary_search(&g1).unwrap(),
        ),
    };
}
