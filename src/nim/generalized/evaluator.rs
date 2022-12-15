use std::collections::{HashSet, HashMap};

use super::{closed_generalized::ClosedGeneralizedNimGame, GeneralizedNimGame};




struct Evaluator{
    data: Vec<Entry>,
    indecies: HashMap<ClosedGeneralizedNimGame,usize>
    tasks: Queue<>
}
enum EvalTask{

}

impl Evaluator{
    pub fn index_of(&self, g: &ClosedGeneralizedNimGame) -> Option<usize>{
        return self.indecies.get(g).copied();
    }
    pub fn addGame(&mut self, game: ClosedGeneralizedNimGame){

    }
    pub fn addEntry(&mut self, entry: Entry){
        self.indecies.insert(entry.game.clone(), self.data.len());
        self.data.push(entry);
    }
}
struct Entry{
    game: ClosedGeneralizedNimGame, //maby make ref?
    unprocessed_child_games: Vec<GeneralizedNimGame>,
    child_games: Vec<Vec<usize>>
}