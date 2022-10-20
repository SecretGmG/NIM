use super::GeneralizedNimGame;
use std::{fmt};

impl fmt::Display for GeneralizedNimGame{



    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{

        let mut display_string = String::from("");

        display_string.push_str("Groups:\n");

        for group in &self.groups{
            for node in group{
                display_string.push_str(format!("{:2} ", *node).as_str());
            }
            display_string.push('\n');
        }
        
        write!(f, "{}", display_string)
    }
}