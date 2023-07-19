pub fn is_valid(s: String) -> bool {
    let mut p: Vec<char> = vec![];
    let mut map: HashMap<char, char> = HashMap::with_capacity(3);
    map.insert(')', '(');
    map.insert('}', '{');
    map.insert(']', '[');

    if s.len() % 2 != 0 {
        return false
    }
    for c in s.chars()  {
        let value = map.get(&c);
        if value == None {
            p.push(c)
        }
        else {
            let m = p.pop();
            if m != value.copied() {
                return false
            }
        }
    }

    if p.len() == 0 {return  true}
    return  false;
}