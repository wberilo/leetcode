pub fn is_palindrome(s: String) -> bool {
    let arrele: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if arrele.len() < 2 {return true}
    let mut index = 0;
    let mut reverse = arrele.len() - 1;
    while index < reverse {
        if arrele[index] != arrele[reverse] {
            return  false;
        }
        index = index + 1;
        reverse = reverse -1;
    }
    return true;

}