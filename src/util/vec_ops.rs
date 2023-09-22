
pub fn insert_sorted_deduped<T: Ord>(vec: &mut Vec<T>, val: T) -> Result<usize, usize> {
    match vec.binary_search(&val) {
        Ok(index) => return Err(index),
        Err(index) => {
            vec.insert(index, val);
            return Ok(index);
        }
    }
}
