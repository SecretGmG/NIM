use crate::generalized_taking_game::TakingGame;
use super::{Pit, Cell, Wall};

const ON: (Cell, Wall, Wall) = (Cell::On, Wall::None, Wall::None);
const OFF: (Cell, Wall, Wall) = (Cell::Off, Wall::None, Wall::None);

impl Pit {
    pub fn try_reconstruct(g: &TakingGame) -> Option<Pit> {
        if let Some(value) = try_get_basic_case(g) {
            return value;
        }
        let (lone_nodes, other_nodes) = get_lone_and_other_nodes(g)?;
        let (h_sets, v_sets) = get_v_h_sets(g, &other_nodes)?;
        let mut board = vec![vec![ON; h_sets.len()]; v_sets.len()];
        set_connected_nodes(other_nodes, &v_sets, &h_sets, g, &mut board);
        append_lone_nodes(lone_nodes, g, v_sets, &mut board, h_sets);
        return Some(Pit::new(board));
    }
}

fn append_lone_nodes(
    lone_nodes: Vec<usize>,
    g: &TakingGame,
    v_sets: Vec<usize>,
    board: &mut Vec<Vec<(Cell, Wall, Wall)>>,
    h_sets: Vec<usize>,
) {
    for lone_node in lone_nodes {
        let gi = g.get_set_indices()[lone_node][0];
        if let Ok(index) = v_sets.binary_search(&gi) {
            append_row(board, index);
        } else if let Ok(index) = h_sets.binary_search(&gi) {
            append_collumn(board, index);
        } else {
            unreachable!()
        }
    }
}
fn append_collumn(board: &mut Vec<Vec<(Cell, Wall, Wall)>>, index: usize) {
    let mut new_collumn = vec![OFF; board[0].len()];
    new_collumn[index] = ON;
    board.push(new_collumn);
}
fn append_row(board: &mut Vec<Vec<(Cell, Wall, Wall)>>, index: usize) {
    for i in 0..board.len() {
        if i == index {
            board[i].push(ON);
        } else {
            board[i].push(OFF);
        }
    }
}
fn set_connected_nodes(
    other_nodes: Vec<usize>,
    v_sets: &Vec<usize>,
    h_sets: &Vec<usize>,
    g: &TakingGame,
    board: &mut Vec<Vec<(Cell, Wall, Wall)>>,
) {
    for node in other_nodes {
        let (x, y) = get_indecies(v_sets, h_sets, &g, node);
        board[x][y] = ON;
    }
}
fn get_v_h_sets(
    g: &TakingGame,
    other_nodes: &Vec<usize>,
) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut h_sets: Vec<usize> = vec![g.get_set_indices()[other_nodes[0] ][0]];
    let mut v_sets: Vec<usize> = vec![];
    let mut nodes_todo = other_nodes.clone();
    while nodes_todo.len() != 0 {
        let mut i = 0;
        while i < nodes_todo.len() {
            let node = nodes_todo[i];
            let g1 = g.get_set_indices()[node][0];
            let g2 = g.get_set_indices()[node][1];

            match try_insert_set(g1, g2, &mut h_sets, &mut v_sets) {
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
    Some((h_sets, v_sets))
}
fn get_lone_and_other_nodes(g: &TakingGame) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut lone_nodes = vec![];
    let mut other_nodes = vec![];
    for node in 0..g.get_node_count() {
        match g.get_set_indices()[node ].len() {
            1 => lone_nodes.push(node),
            2 => other_nodes.push(node),
            _ => return None,
        }
    }
    Some((lone_nodes, other_nodes))
}
fn try_get_basic_case(g: &TakingGame) -> Option<Option<Pit>> {
    if g.get_set_indices().len() == 1 {
        let board = vec![vec![ON; g.get_node_count() ]];

        return Some(Some(Pit::new(board)));
    }
    None
}
pub fn try_insert_set(
    g1: usize,
    g2: usize,
    h_set: &mut Vec<usize>,
    v_set: &mut Vec<usize>,
) -> Option<bool> {
    let h1 = h_set.binary_search(&g1);
    let h2 = h_set.binary_search(&g2);
    let v1 = v_set.binary_search(&g1);
    let v2 = v_set.binary_search(&g2);
    match (h1, h2, v1, v2) {
        (_, _, Ok(_), Ok(_)) => return None,
        (Ok(_), Ok(_), _, _) => return None,
        (Ok(_), Err(_), Err(_), Err(index)) => v_set.insert(index, g2),
        (Err(_), Ok(_), Err(index), Err(_)) => v_set.insert(index, g1),
        (Err(_), Err(index), Ok(_), Err(_)) => h_set.insert(index, g2),
        (Err(index), Err(_), Err(_), Ok(_)) => h_set.insert(index, g1),
        (Ok(_), Err(_), Err(_), Ok(_)) => return Some(true),
        (Err(_), Ok(_), Ok(_), Err(_)) => return Some(true),
        (_, _, _, _) => return Some(false),
    }
    return Some(true);
}
fn get_indecies(
    v_set: &Vec<usize>,
    h_set: &Vec<usize>,
    g: &TakingGame,
    node: usize,
) -> (usize, usize) {
    let g1 = g.get_set_indices()[node][0];
    let g2 = g.get_set_indices()[node][1];
    return match v_set.binary_search(&g1) {
        Ok(x) => (x, h_set.binary_search(&g2).unwrap()),
        Err(_) => (
            v_set.binary_search(&g2).unwrap(),
            h_set.binary_search(&g1).unwrap(),
        ),
    };
}
