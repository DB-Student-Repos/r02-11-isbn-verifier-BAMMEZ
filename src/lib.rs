pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut multiplier = 10;

    for c in isbn.chars().filter(|&c| c.is_ascii_digit() || c == 'X') {
        if let Some(digit) = c.to_digit(10) {
            sum += multiplier * digit;
            multiplier -= 1;
        } else if c == 'X' && multiplier == 1 {
            sum += 10;
            multiplier -= 1;
        } else {
            return false;
        }
    }

    sum % 11 == 0 && multiplier == 0
}




