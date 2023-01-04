#[warn(clippy::todo)]
///
use crate::nim::generalized::{closed_generalized::ClosedGeneralizedNimGame, GeneralizedNimGame};

use super::impartial::{Impartial, ImpartialPart};

impl ImpartialPart<ClosedGeneralizedNimGame, GeneralizedNimGame> for ClosedGeneralizedNimGame{
    
    fn get_easy_nimber(&self) -> Option<u16> {
        self.get_easy_nimber()
    }

    fn get_max_nimber(&self) -> u16 {
        self.get_node_count()
    }

    fn get_unique_leafs(&self) -> Vec<GeneralizedNimGame> {
        self.get_unique_child_games()
    }
}
impl Impartial<ClosedGeneralizedNimGame, GeneralizedNimGame> for GeneralizedNimGame{
    fn get_parts(&self) -> &Vec<ClosedGeneralizedNimGame> {
        self.get_parts()
    }
}