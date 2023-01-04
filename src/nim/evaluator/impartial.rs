pub trait ImpartialPart<Part, Whole>: Sized + Clone + Eq + std::hash::Hash
where
    Part: ImpartialPart<Part, Whole>,
    Whole: Impartial<Part, Whole>,
{
    fn get_easy_nimber(&self) -> Option<u16>;
    fn get_max_nimber(&self) -> u16;
    fn get_unique_leafs(&self) -> Vec<Whole>;
}
pub trait Impartial<Part, Whole>: Sized
where
    Part: ImpartialPart<Part, Whole>,
    Whole: Impartial<Part, Whole>,
{
    fn get_parts(&self) -> &Vec<Part>;
    
    fn get_part_count(&self) -> usize{
        self.get_parts().len()
    }
    fn get_max_nimber(&self) -> u16 {
        self.get_parts().iter().map(|g| g.get_max_nimber()).sum()
    }
}
