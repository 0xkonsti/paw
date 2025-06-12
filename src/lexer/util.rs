pub fn is_identifier(c: char, is_first_char: bool) -> bool {
    c == '_' || c.is_alphabetic() || (!is_first_char && c.is_digit(10))
}

pub fn is_number(c: char, is_float: &mut bool) -> bool {
    if c == '.' {
        if *is_float {
            return false;
        }
        *is_float = true;
        return true;
    }

    c.is_digit(10)
}
