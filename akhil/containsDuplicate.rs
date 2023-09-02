// Description: https://leetcode.com/problems/contains-duplicate/description/

use std::collections::HashSet;

// Vec<i32> -> dynamic-sized heap-allocated array of type 32-bit signed int
// Vec<T> -> allows to store elements of type T 
impl Solution{
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        // make variable mutable with keyword mut
        let mut set = HashSet::new();

        for num in nums{
            // HashSet::insert() inserts the given element into the set
            // if HashSet has the element already, it will return false, else true
            if !set.insert(num){
                return true
            }
        }
        false
    }
}
