use std::fmt::Debug;
use std::fmt;
use crate::util::vec_ops;

use super::ClosedGeneralizedNimGame;

impl Ord for ClosedGeneralizedNimGame {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return vec_ops::compare_sorted1(&self.groups, &other.groups);
    }
}
impl PartialOrd for ClosedGeneralizedNimGame{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}
impl Eq for ClosedGeneralizedNimGame{

}
impl PartialEq for ClosedGeneralizedNimGame {
    fn eq(&self, other: &Self) -> bool {
        if self.nodes != other.nodes {return false;}
        groups_eq(&self.groups, &other.groups)
    }
}
fn groups_eq(a: &Vec<Vec<u16>>, b: &Vec<Vec<u16>>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i].len() != b[i].len() {
            return false;
        }

        for j in 0..a[i].len() {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    return true;
}
impl Clone for ClosedGeneralizedNimGame {
    fn clone(&self) -> Self {
        Self {
            groups: self.groups.clone(),
            group_indecies: self.group_indecies.clone(),
            nodes: self.nodes.clone(),
        }
    }
}
impl core::hash::Hash for ClosedGeneralizedNimGame {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return self.groups.hash(state);
    }
}
impl Debug for ClosedGeneralizedNimGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeneralizedNimGame")
            .field("groups", &self.groups)
            .finish()
    }
}


impl fmt::Display for ClosedGeneralizedNimGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = String::from("");

        display_string.push_str("Groups:\n");

        for group in &self.groups {
            for node in group {
                display_string.push_str(format!("{:2} ", *node).as_str());
            }
            display_string.push('\n');
        }

        write!(f, "{}", display_string)
    }
}
