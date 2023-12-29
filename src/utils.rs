static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58, 
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8, 
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33
];

pub fn bit_scan(bit: u64) -> usize {
    let remainder = (bit % 67) as usize;
    return MOD67TABLE[remainder];
}

pub fn split_on(s: &str, sep: char) -> (&str, &str) {
    for (i, item) in s.chars().enumerate() {
        if item == sep {
            return (&s[0..i], &s[i+1..]);
        }
    }
    (&s[..], "")
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_on_space_works() {
        let test_string = "A B C D";
        let (should_be_a, rest) = split_on(test_string, ' ');
        assert_eq!(should_be_a, "A");
        assert_eq!(rest, "B C D");
    }

    #[test]
    fn split_on_ascii_works() {
        for i in 0..128 {
            let ch = char::from(i);
            if ch == 'A' {
                continue;
            }

            let test_string = format!("AA{}BB{}CC{}DD", ch, ch, ch);
            let (should_be_a, rest) = split_on(&test_string, ch);
            assert_eq!(should_be_a, "AA", "{},{}, {}", test_string, ch, i);
            assert_eq!(rest, &format!("BB{}CC{}DD", ch, ch));        }
    }

    #[test] 
    fn bit_scan_works() {
        for i in 0..64 {
            let bit = (1 as u64) << i;
            let index = bit_scan(bit);
            assert_eq!(i, index);
        }
    }
}