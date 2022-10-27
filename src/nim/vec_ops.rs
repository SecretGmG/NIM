pub fn contains_any_sorted(a: &Vec<u16>, b: &Vec<u16>) -> bool{
    let mut i = 0;
    let mut j = 0;
    
    while i<a.len() && j<b.len(){

        if a[i] < b[j]{
            i += 1;
        }
        else if a[i] > b[j]{
            j += 1;
        }
        else{
            return true;
        }
    }
    return false;
}
pub fn sorted_without(vec1: &Vec<u16>, vec2: &Vec<u16>) -> Vec<u16> {
    let mut i = 0;
    let mut j = 0;
    let mut r = vec![];
    while i < vec1.len(){
        if j >= vec2.len(){
            r.push(vec1[i]);
            i+=1;
        }
        else if vec1[i] < vec2[j] {
            r.push(vec1[i]);
            i += 1;
        }
        else if vec1[i] == vec2[j] {
            i += 1;
        }
        else{
            j += 1;
        }
    }
    return r;
}
///minimal excluded
///calculates the smallest number not in the list
pub fn mex(nums: &mut Vec<u16>) -> u16 {
    nums.sort();
    let mut r: u16 = 0;
    let mut i = 0;
    while i < nums.len() {
        if r < nums[i] { //r is the smallest not in the list
            break;
        } 
        if r == nums[i] { //r is in the list
            r += 1;
        } 
        i += 1;
    }
    return r;
}
///similiar to dedup
///removes both of the values if they are equal
pub fn remove_pairs_sorted<T: Eq>(vec: &mut Vec<T>){
    let mut i= vec.len();
    while i>1{
        if vec[i-1] == vec[i-2]{
            vec.remove(i-1);
            vec.remove(i-2);
            i-=2;
        }
        else{
            i-=1;
        }
    }
}