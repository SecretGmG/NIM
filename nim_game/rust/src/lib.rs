use evaluator::{Evaluator, Impartial};
use godot::prelude::*;
use sorted_vec::SortedSet;
use taking_game::TakingGame;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct NimBot {
    eval: Evaluator<TakingGame>,
    base: Base<Node>,
}
#[godot_api]
impl INode for NimBot {
    fn init(base: Base<Node>) -> Self {
        Self {
            eval: Evaluator::new(),
            base,
        }
    }
}

#[godot_api]
impl NimBot {
    #[func]
    pub fn sample_move(&mut self, &sets_of_nodes: Array<PackedInt32Array>) -> Array<i32> {
        let sets_of_nodes: Vec<SortedSet<usize>> = sets_of_nodes
            .iter_shared()
            .map(|arr| {
                SortedSet::from_unsorted(arr.as_slice().iter().map(|val| *val as usize).collect())
            })
            .collect();

        let game = TakingGame::from_sets_of_nodes(sets_of_nodes);

        let mut moves = game.get_moves();

        let mut best_move = match moves.pop() {
            Some(_move) => _move,
            None => {
                return Array::new();
            }
        };

        while let Some(_move) = moves.pop() {
            let nimber = self.eval.get_bounded_nimber(_move.clone(), 0);
            if nimber.is_some_and(|nimber| nimber == 0) {
                best_move = _move;
                break;
            }
        }
        Array::from_iter(game.get_nodes().iter().filter_map(|node| {
            if best_move.get_nodes().contains(node) {
                None
            } else {
                Some(*node as i32)
            }
        }))
    }
}
