pub mod base62 {
    const BASE: u64 = 62;
    const ALPHABET: [char; BASE as usize] =
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
        'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
        'v', 'w', 'x', 'y', 'z'];

    pub fn encode(mut num: u64) -> String {
        if num == 0 { return "0".to_owned() }
        let mut result  = String::new();

        while num > 0 {
            let char_holder = ALPHABET[(num % BASE) as usize];
            &result.push(char_holder);
            num /= BASE
        }

        return result.chars().rev().collect();
    }

    pub fn decode(string: &str) -> u64 {
        let mut result = 0;

        for (i, c) in string.chars().rev().enumerate() {
            let num = BASE.pow(i as u32);
            result += ALPHABET.iter().position(|x| *x == c).unwrap() as u64 * num;
        }

        return result;
    }
}

#[test]
fn it_encodes() {
    assert_eq!(base62::encode(1337), "LZ");
}

#[test]
fn it_decodes() {
    assert_eq!(base62::decode("LZ"), 1337);
}