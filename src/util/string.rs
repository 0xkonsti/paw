pub fn shorten_string(string: &impl ToString, max_len: usize) -> String {
    let mut s = String::new();
    for (i, c) in string.to_string().chars().enumerate() {
        if i >= max_len {
            break;
        }
        s.push(c);
    }

    return s;
}
