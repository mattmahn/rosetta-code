fn cusip_check(cusip: &str) -> bool {
    if cusip.len() != 9 {
        return false;
    }

    let mut v: u8 = 0;
    let mut total = 0u8;
    let capital_cusip = cusip.to_uppercase();
    let foo = capital_cusip.as_str().char_indices().take(7);
    for (i, c) in foo {
        if c.is_digit(10) {
            v = c.to_digit(10).unwrap() as u8;
        } else if c.is_alphabetic() {
            let p = (c as u8) - ('A' as u8) + 1;
            v = p + 9;
        } else if c == '*' {
            v = 36;
        } else if c == '@' {
            v = 37;
        } else if c == '#' {
            v = 38;
        }

        if i % 2 != 0 {
            v *= 2
        }
        //println!("v = {}", v);
        total += (v / 10) + v % 10;
    }
    let check: u8 = (10 - (total % 10)) % 10;
    (check.to_string().chars().nth(0).unwrap()) == cusip.chars().nth(cusip.len() - 1).unwrap()
}

fn main() {
    let codes = [
        "037833100",
        "17275R102",
        "38259P508",
        "594918104",
        "68389X106",
        "68389X105",
    ];
    for code in codes.iter() {
        println!("{} -> {}", code, cusip_check(code))
    }
}
