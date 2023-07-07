pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = Vec::new();
    let mut hash: HashMap<String,Vec<String>> = HashMap::new();

    for ele in strs {
        let mut arrele: Vec<char> = ele.chars().collect();
        arrele.sort();

        let s: String = arrele.iter().collect();
        let key = hash.get_mut(&s);
        if let Some(v) = key {
            v.push(ele);
            continue;
        }

        hash.insert(s, vec![ele]);
    }

    for ele in hash {
        res.push(ele.1)
    }

    return res;

}