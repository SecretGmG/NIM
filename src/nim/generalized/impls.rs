
use super::GeneralizedNimGame;

impl evaluator::Impartial<GeneralizedNimGame> for GeneralizedNimGame{
    fn get_parts(self) -> Vec<GeneralizedNimGame> {
        self.parts.into_iter().map(|part| Self::from_closed(vec![part])).collect()
    }

    fn get_max_nimber(&self) -> u16 {
        self.parts.iter().map(|part| part.get_node_count()).sum()
    }

    fn get_unique_moves(&self) -> Vec<GeneralizedNimGame> {
        let mut moves = vec![];
        for i in 0..self.parts.len(){
            let mut parts_without_move = self.parts.clone();
            let parts_moves = self.parts[i].get_unique_child_games();
            parts_without_move.swap_remove(i);
            for j in 0..parts_moves.len(){
                let mut _move = parts_without_move.clone();
                _move.append(&mut parts_moves[j].parts.clone());
                moves.push(_move);
            }
        }
        return moves.into_iter().map(|parts| GeneralizedNimGame::from_closed(parts)).collect();
    }
}