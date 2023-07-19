pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = std::collections::HashSet::new();

    for value in nums {
        if map.contains(&value) {
            return true;
        } else {
            map.insert(value);
        }
    }
    false
}