use std::char;

const BASE64ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

pub fn encode(string: &str) -> String {
    let chars: Vec<char> = string.chars().collect();
    let alphabet: Vec<char> = BASE64ALPHABET.chars().collect();
    let mut result = String::new();

    let mut i = 0;

    while i < chars.len() {
        let chr1 = chars[i] as u32;
        i += 1;

        let chr2 = if i == chars.len() { 0 } else { chars[i] as u32 };
        i += 1;

        let chr3 = if i >= chars.len() { 0 } else { chars[i] as u32 };
        i += 1;

        let res1 = chr1 >> 2;
        let res2 = ((chr1 & 3) << 4) | (chr2 >> 4);

        let mut res3 = ((chr2 & 15) << 2) | (chr3 >> 6);
        let mut res4 = chr3 & 63;

        if chr2 == 0 {
            res3 = 64;
            res4 = 64;
        } else if chr3 == 0 {
            res4 = 64;
        }

        result.push(alphabet[res1 as usize]);
        result.push(alphabet[res2 as usize]);
        result.push(alphabet[res3 as usize]);
        result.push(alphabet[res4 as usize]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(encode("Hi!"), String::from("SGkh"));
        assert_eq!(encode("Test"), String::from("VGVzdA=="));
        assert_eq!(encode("Rust"), String::from("UnVzdA=="));
        assert_eq!(
            encode("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
            String::from(
                "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdA=="
            )
        );
    }
}
