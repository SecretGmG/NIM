use evaluator::Impartial;
use super::{TakingGame, util};


impl Impartial<TakingGame> for TakingGame{
    fn get_parts(self) -> Vec<TakingGame> {
        split_to_independent_sets_of_groups(self.sets_of_nodes).into_iter().map(|sets| TakingGame::new(sets)).collect()
    }
    fn get_max_nimber(&self) -> usize {
        self.node_count
    }

    fn get_unique_moves(&self) -> Vec<TakingGame> {
        self.get_deduped_moves()
    }
    
}


fn split_to_independent_sets_of_groups(groups: Vec<Vec<usize>>) -> Vec<Vec<Vec<usize>>> {
    let mut groups = groups;
    let mut parts = vec![];

    for i in 0..groups.len() {
        groups[i].sort_unstable();
        groups[i].dedup();
    }

    while groups.len() != 0 {
        let mut nodes_in_current_group = groups.swap_remove(0);
        let mut new_part = vec![nodes_in_current_group.clone()];

        let mut prev_len = 0;
        while nodes_in_current_group.len() != prev_len {
            let mut i = 0;
            prev_len = nodes_in_current_group.len();
            while i < groups.len() {
                if util::have_common_element(&groups[i], &nodes_in_current_group) {
                    nodes_in_current_group.append(&mut util::remove_subset(
                        &groups[i],
                        &nodes_in_current_group,
                    ));
                    nodes_in_current_group.sort_unstable();
                    new_part.push(groups.remove(i));
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
