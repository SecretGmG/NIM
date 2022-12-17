use std::{cmp::Ordering, fmt::{Display, Debug}};

use crate::vec_ops;

use super::GeneralizedNimGame;

impl Ord for GeneralizedNimGame {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return vec_ops::compare_sorted(&self.parts, &other.parts);
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
}
impl PartialOrd for GeneralizedNimGame{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for GeneralizedNimGame{

}
impl PartialEq for GeneralizedNimGame{
    fn eq(&self, other: &Self) -> bool {
        if self.parts.len() != other.parts.len() {return false;}
        for i in 0..self.parts.len(){
            if self.parts[i] != other.parts[i] {return false;}
        }
        return true;
    }
}
impl Display for GeneralizedNimGame{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n-----Generalized-Nim-Game-----\n")?;
        for closed in &self.parts{
            write!(f, "{}", closed)?;
        }
        Ok(())
    }
}
impl Debug for GeneralizedNimGame{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}