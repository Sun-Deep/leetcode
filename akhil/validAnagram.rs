impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = std::collections::HashMap::new();
        // iterate over both strings, make an entry of each char in the map, increase the count if exists in s
        // decrease the count, if exists in t. 
        // the remaining should be default value in count for mapped key, value
        // default count value is set to 0
        s.chars().for_each(|i| *map.entry(i).or_insert(0) += 1);
        t.chars().for_each(|i| *map.entry(i).or_insert(0) -= 1);
        return !map.into_values().any(|count| count != 0);
    }
}