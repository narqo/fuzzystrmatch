const CODE_LEN: usize = 4;

pub fn soundex(text: &str) -> String {
    let mut code = String::with_capacity(CODE_LEN);

    let mut chars = text.chars();
    let mut prevc = chars.next().unwrap();
    code.push(prevc.to_ascii_uppercase());

    let mut prevd = char_to_digit(prevc).unwrap_or('0');
    while code.len() < CODE_LEN {
        let c = if let Some(c) = chars.next() {
            c
        } else {
            break;
        };
        let d = char_to_digit(c).unwrap_or(prevd);
        if d != prevd || is_vowel(prevc) {
            code.push(d);
            prevd = d;
        }
        prevc = c;
    }
    if code.len() < CODE_LEN {
        code.push_str("0".repeat(CODE_LEN).as_str());
        code.truncate(CODE_LEN);
    }
    code
}

fn char_to_digit(c: char) -> Option<char> {
    let d = match c.to_ascii_lowercase() {
        // drop all occurrences of the following
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'h' | 'w' => {
            return None;
        }
        'b' | 'f' | 'p' | 'v' => '1',
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => '2',
        'd' | 't' => '3',
        'l' => '4',
        'm' | 'n' => '5',
        'r' => '6',
        _ => panic!("unexpected char {:?}", c),
    };
    Some(d)
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(soundex("ammonium"), "A555");

        assert_eq!(soundex("Robert"), "R163");
        assert_eq!(soundex("Rubin"), "R150");
        assert_eq!(soundex("Ashcraft"), "A261");

        // 'z' and 'k' in the name are coded as 2 twice since a vowel lies in between them
        assert_eq!(soundex("Tymczak"), "T522");

        // the first two letters have the same number and are coded once as 'P'
        assert_eq!(soundex("Pfister"), "P236");

        assert_eq!(soundex("Honeyman"), "H555");
    }

    #[test]
    fn same_code() {
        assert_eq!(soundex("Ashcraft"), soundex("Ashcroft"));
        assert_eq!(soundex("Robert"), soundex("Rupert"));
    }

    #[test]
    fn short_code() {
        assert_eq!(soundex("Ji"), "J000");
        assert_eq!(soundex("Zhu"), "Z000");
        assert_eq!(soundex("Olya"), "O400");
    }
}
