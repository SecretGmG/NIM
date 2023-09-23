use std::vec;
use super::TakingGame;
use super::ClosedTakingGamePart;

//implements the generation of moves;
impl ClosedTakingGamePart {
    pub fn get_unique_moves(&self) -> Vec<TakingGame> {
        let mut moves = vec![];

        let mut processed_sets:Vec<&Vec<usize>> = vec![];

        for group in &self.sets_of_nodes {
            processed_sets.push(group);
            let (lone_nodes, other_nodes) = self.collect_lone_nodes_and_other_nodes(group);
            self.append_moves_of_group(lone_nodes, other_nodes, &mut moves);
        }
        moves.sort_unstable();
        moves.dedup();
        return moves;
    }
    fn append_moves_of_group(
        &self,
        lone_nodes: Vec<usize>,
        other_nodes: Vec<usize>,
        child_games: &mut Vec<TakingGame>,
    ) {
        if other_nodes.len() > 128 {
            panic!("This game is too complex!")
        }

        for lone_nodes_to_remove in 0..(lone_nodes.len() + 1) {
            let mask_bound = 1 << other_nodes.len();
            let start_value = if lone_nodes_to_remove == 0 { 1 } else { 0 };
            for mask in start_value..mask_bound {
                let child_game =
                    self.get_child(&lone_nodes, &other_nodes, lone_nodes_to_remove , mask);

                child_games.push(child_game);
            }
        }
    }
    fn collect_lone_nodes_and_other_nodes(&self, group: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
        let mut lone_nodes_in_group = vec![];
        let mut other_nodes_in_group = vec![];

        for node in group {
            let node = *node;
            if self.set_indices[node ].len() == 0 {
                lone_nodes_in_group.push(node);
            } else {
                other_nodes_in_group.push(node);
            }
        }
        return (lone_nodes_in_group, other_nodes_in_group);
    }

    fn get_child(
        &self,
        lone_nodes: &Vec<usize>,
        other_nodes: &Vec<usize>,
        lone_nodes_to_remove: usize,
        mask: u128,
    ) -> TakingGame {
        let mut nodes_to_remove = vec![];
        for i in 0..lone_nodes_to_remove {
            nodes_to_remove.push(lone_nodes[i ]);
        }
        let mut mask = mask;
        for i in 0..other_nodes.len() {
            if mask % 2 == 1 {
                nodes_to_remove.push(other_nodes[i]);
            }
            mask >>= 1;
        }
        let child = self.make_move_unchecked(&mut nodes_to_remove);

        return child;
    }
    ///removes all nodes specified in the argument
    pub fn make_move_unchecked(&self, nodes_to_remove: &mut Vec<usize>) -> TakingGame {
        nodes_to_remove.sort_unstable();

        let mut new_groups = vec![];

        for group in &self.sets_of_nodes {
            new_groups.push(super::super::remove_subset(group, nodes_to_remove));
        }

        let new_game = TakingGame::new(new_groups);

        return new_game;
    }
}
