fn next_bigger_number(n: u64) -> Option<u64> {
    let mut digits = n.to_string()
                  .chars()
                  .map(|c| c.to_digit(10).unwrap() as u64)
                  .collect::<Vec<u64>>();

    let mut index = digits.len() - 1;

    while index > 0 {
        if digits[index] >  digits[index -1] {
            let (left, right) = digits.split_at_mut(index - 1);

            let mut vecleft = left.to_vec();
            let mut sorted_right = right.to_vec();
            sorted_right.sort();



            let mut updatedarray = sorted_right.clone();
            for (i, item) in sorted_right.iter().enumerate() {
                if item > &right[0] {
                    updatedarray.remove(i);
                    updatedarray.reverse();
                    updatedarray.push(*item);
                    updatedarray.reverse();
                    break;
                }
            }


            vecleft.append(&mut updatedarray);




            let num = vecleft.iter().fold(0, |acc, x| acc * 10 + x);
            return Some(num);
        }
        index -= 1;
    }
    return None;
}