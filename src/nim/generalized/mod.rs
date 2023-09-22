pub mod closed_generalized;
pub mod constructor;
mod impls;
use closed_generalized::ClosedTakingGamePart;

///A generalized version of any impartial "taking game"
///implements many tools to effitiently find the nimber of any complex taking game
#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TakingGame {
    parts: Vec<ClosedTakingGamePart>,
}
impl TakingGame {
    pub fn new(groups: Vec<Vec<u16>>) -> TakingGame {
        let closed_groups = split_to_independent_sets_of_groups(groups);
        let parts: Vec<ClosedTakingGamePart> = closed_groups
            .into_iter()
            .map(ClosedTakingGamePart::new)
            .collect();
        return Self::from_closed(parts);
    }
    pub fn from_closed(parts: Vec<ClosedTakingGamePart>) -> TakingGame {
        let mut parts = parts;
        parts.sort_unstable();
        return TakingGame { parts };
    }
    pub fn get_node_count(&self) -> u16 {
        self.parts.iter().map(|p| p.get_node_count()).sum()
    }
    pub fn get_groups(&self) -> Vec<Vec<u16>>{
        let mut offset = 0;
        let mut groups = vec![];
        for part in &self.parts{
            let mut new_group = part.get_sets_of_nodes().clone();
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
}
fn split_to_independent_sets_of_groups(groups: Vec<Vec<u16>>) -> Vec<Vec<Vec<u16>>> {
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
                if have_common_element(&groups[i], &nodes_in_current_group) {
                    nodes_in_current_group.append(&mut remove_subset(
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
///retures true if a and b share any elements
pub fn have_common_element(a: &Vec<u16>, b: &Vec<u16>) -> bool {
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            i += 1;
        } else if a[i] > b[j] {
            j += 1;
        } else {
            return true;
        }
    }
    return false;
}
pub fn remove_subset(vec1: &Vec<u16>, vec2: &Vec<u16>) -> Vec<u16> {
    let mut i = 0;
    let mut j = 0;
    let mut r = vec![];
    while i < vec1.len() {
        if j >= vec2.len() {
            r.push(vec1[i]);
            i += 1;
        } else if vec1[i] < vec2[j] {
            r.push(vec1[i]);
            i += 1;
        } else if vec1[i] == vec2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    return r;
}