
use super::TakingGame;

impl evaluator::Impartial<TakingGame> for TakingGame{
    fn get_parts(self) -> Vec<TakingGame> {
        self.parts.into_iter().map(|part| Self::from_closed(vec![part])).collect()
    }
    fn get_max_nimber(&self) -> u16 {
        self.parts.iter().map(|part| part.get_node_count()).sum()
    }
    fn get_possible_nimbers(&self) -> Vec<u16> {
        (0..self.get_max_nimber()).collect()
    }
    fn get_unique_moves(&self) -> Vec<TakingGame> {
        let mut moves = vec![];
        for i in 0..self.parts.len(){
            let mut parts_without_move = self.parts.clone();
            let parts_moves = self.parts[i].get_unique_moves();
            parts_without_move.swap_remove(i);
            for j in 0..parts_moves.len(){
                let mut _move = parts_without_move.clone();
                _move.append(&mut parts_moves[j].parts.clone());
                moves.push(_move);
            }
        }
        return moves.into_iter().map(|parts| TakingGame::from_closed(parts)).collect();
    }
}