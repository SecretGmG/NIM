pub mod closed_generalized;
pub mod constructor;
mod impls;
use crate::util::vec_ops::{self, contains_any_sorted};
use closed_generalized::ClosedGeneralizedNimGame;

///A generalized version of any impartial "taking game"
///implements many tools to effitiently find the nimber of any complex taking game
#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GeneralizedNimGame {
    parts: Vec<ClosedGeneralizedNimGame>,
}
impl GeneralizedNimGame {
    pub fn new(groups: Vec<Vec<u16>>) -> GeneralizedNimGame {
        let closed_groups = split(groups);

        let parts: Vec<ClosedGeneralizedNimGame> = closed_groups
            .into_iter()
            .map(ClosedGeneralizedNimGame::new)
            .collect();

        return Self::from_closed(parts);
    }
    pub fn from_closed(parts: Vec<ClosedGeneralizedNimGame>) -> GeneralizedNimGame {
        let mut parts = parts;
        parts.sort_unstable();

        vec_ops::remove_pairs_sorted(&mut parts);

        return GeneralizedNimGame { parts };
    }
    pub fn get_parts(&self) -> &Vec<ClosedGeneralizedNimGame> {
        return &self.parts;
    }
    pub fn get_node_count(&self) -> u16 {
        self.parts.iter().map(|p| p.get_node_count()).sum()
    }
    pub fn get_groups(&self) -> Vec<Vec<u16>>{
        let mut offset = 0;
        let mut groups = vec![];
        for part in &self.parts{
            let mut new_group = part.get_groups().clone();
            for i in 0..new_group.len() {
                for j in 0..new_group[i].len(){
                    new_group[i][j] += offset;
                }
            }
            offset += part.get_node_count();
            groups.append(&mut new_group);
        }
        return groups;
    }
    pub fn as_closed(&self) -> Result<&ClosedGeneralizedNimGame,&ClosedGeneralizedNimGame>{
        if self.parts.len() == 1 {return Ok(&self.parts[0])};
        return Err(&self.parts[0]);
    }
}

fn split(groups: Vec<Vec<u16>>) -> Vec<Vec<Vec<u16>>> {
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
                if contains_any_sorted(&groups[i], &nodes_in_current_group) {
                    nodes_in_current_group.append(&mut vec_ops::sorted_without(
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
