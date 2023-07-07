

fn main() {
    println!("Hello, world!");
    println!("{}",find_median_sorted_arrays(vec![100001], vec![100000]));
}


pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut index = 0;
    let mut index1 = 0;
    let mut index2 = 0;

    let mut last:f64= 0.0;
    let mut second:f64 = 0.0;

    let length = nums1.len() + nums2.len();

    while index <= length/2 {
        if !(nums2.len() > 0) {
            second = last;
            last = nums1[index1].into();
            index1 += 1;
        }
        else if !(nums1.len() > 0) {
            second = last;
            last = nums2[index2].into();
            index2 += 1;
        }


        else if index1 < nums1.len() && (index2 >= nums2.len() || nums1[index1] <= nums2[index2]) {
            second = last;
            last = nums1[index1].into();
            index1 += 1;
        }
        else if nums2.len() > index2 {
            second = last;
            last = nums2[index2].into();
            index2 += 1;
        }
        index += 1;
    }

    if length % 2 == 0 {
        return ((last+ second)/2.0).into();
    }
    return last.into();
}