pub fn search(nums: Vec<i32>, target: i32) -> i32 {

    return binarySearch(&nums, target, 0, nums.len());

}

pub fn binar_search(nums: &Vec<i32>, target: i32, start: usize, end:usize) -> i32{
    
    let mid = (start+ end)/2;
    if end - start == 0 {
        return -1;
    }
    if end - start <= 1 &&  nums[mid] != target {
        return -1;
    }
    
    if nums[mid] == target {
        return mid as i32;
    }
    else if nums[mid] < target {
        return binarySearch(nums, target, mid, end);
    }
    else if nums[mid] > target {
        return binarySearch(nums, target, start, mid);
    }

    return -1;
}