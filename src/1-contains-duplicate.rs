pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = std::collections::HashMap::new();

    for value in nums {
        if map.contains_key(&value) {
            return true;
        } else {
            map.insert(value, true);
        }
    }
    false
}