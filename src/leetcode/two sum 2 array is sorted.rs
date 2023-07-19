pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut f = 0;
    let mut s = numbers.len() - 1;

    while s > f {
        if numbers[f] + numbers[s] < target {
            f += 1;
        }
        else if numbers[f] +  numbers[s] > target {
            s -= 1;
        }
        else {
            return vec![f as i32 + 1, s as i32 + 1];
        }
    }

    vec![]
}