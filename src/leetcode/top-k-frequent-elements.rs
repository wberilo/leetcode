pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hashm: HashMap<i32, i32> = HashMap::new();

    let length = nums.len() + 1;
    for ele in nums {
        if hashm.contains_key(&ele) {
            hashm.insert(ele, hashm[&ele] + 1);
        }
        else {
            hashm.insert(ele, 1);
        }
    }

    let mut res: Vec<i32> = Vec::with_capacity(length);
    let mut pos: Vec<Vec<i32>> = Vec::with_capacity(length);

    for e in 0..length {
        pos.push(vec![]);
    }

    for elem in hashm{
        pos[elem.1 as usize].push(elem.0)
    }


    let mut index = length - 1;
    let mut spaces: usize = k as usize;
    while index != 0 && spaces != 0 {
        if !pos[index].is_empty() {
            spaces -= pos[index].len();
            res.append(&mut pos[index]);
        }
        index -= 1;
    }

    return res;
}