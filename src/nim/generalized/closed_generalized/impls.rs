use std::{collections::hash_map::DefaultHasher, fmt::Debug, hash::Hash};
use std::fmt;
use super::ClosedGeneralizedNimGame;

impl Ord for ClosedGeneralizedNimGame {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.groups.len() != other.groups.len() {
            return self.groups.len().cmp(&other.groups.len());
        }
        for i in 0..self.groups.len() {
            match self.groups[i].len().cmp(&other.groups[i].len()) {
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                std::cmp::Ordering::Equal => {
                    for j in 0..self.groups[i].len() {
                        match self.groups[i][j].cmp(&other.groups[i][j]) {
                            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                            std::cmp::Ordering::Equal => (),
                        }
                    }
                }
            }
        }
        return std::cmp::Ordering::Equal;
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
        groups_eq(&self.groups, &other.groups)
    }
}
fn groups_eq(a: &Vec<Vec<u16>>, b: &Vec<Vec<u16>>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let h = &mut DefaultHasher::new();
    if a.hash(h) != b.hash(h) {
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
            neighbours: self.neighbours.clone(),
            nodes: self.nodes.clone(),
        }
    }
}
impl core::hash::Hash for ClosedGeneralizedNimGame {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        //return self.groups.hash(state); <- this is slower
        self.nodes.hash(state);
        for i in 0..self.nodes {
            for node in &self.neighbours[i as usize] {
                (*node).hash(state);
            }
        }
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
