use evaluator::Impartial;
use super::{TakingGame, util};


impl Impartial<TakingGame> for TakingGame{
    fn get_parts(self) -> Vec<TakingGame> {
        split_to_independent_sets_of_nodes(self.sets_of_nodes).into_iter().map(|sets| TakingGame::new(sets)).collect()
    }
    fn get_max_nimber(&self) -> usize {
        match self.find_symmetry() {
            Some(_) => 0,
            None => self.node_count,
        }
    }

    fn get_unique_moves(&self) -> Vec<TakingGame> {
        self.get_deduped_moves()
    }
    
}


fn split_to_independent_sets_of_nodes(mut sets_of_nodes: Vec<Vec<usize>>) -> Vec<Vec<Vec<usize>>> {
    let mut parts = vec![];

    for i in 0..sets_of_nodes.len() {
        sets_of_nodes[i].sort_unstable();
        sets_of_nodes[i].dedup();
    }

    while sets_of_nodes.len() != 0 {
        let mut nodes_in_current_set = sets_of_nodes.swap_remove(0);
        let mut new_part = vec![nodes_in_current_set.clone()];

        let mut prev_len = 0;
        while nodes_in_current_set.len() != prev_len {
            let mut i = 0;
            prev_len = nodes_in_current_set.len();
            while i < sets_of_nodes.len() {
                if util::have_common_element(&sets_of_nodes[i], &nodes_in_current_set) {
                    nodes_in_current_set.append(&mut util::remove_subset(
                        &sets_of_nodes[i],
                        &nodes_in_current_set,
                    ));
                    nodes_in_current_set.sort_unstable();
                    new_part.push(sets_of_nodes.remove(i));
                } else {
                    i += 1;
                }
            }
        }
        if new_part[0].len() != 0 {
            parts.push(new_part);
        }
    }
    return parts;
}
