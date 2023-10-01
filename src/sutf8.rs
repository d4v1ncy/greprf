use hex;
use std::fmt;
use std::str::from_utf8;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Standard;


// const ASCII_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
// const ASCII_LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
// const ASCII_UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
// const ASCII_DIGITS: &'static str = "0123456789";
// const ASCII_HEXDIGITS: &'static str = "0123456789abcdefABCDEF";
// const ASCII_OCTDIGITS: &'static str = "01234567";
const ASCII_PRINTABLE: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
// const ASCII_PUNCTUATION: &'static str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
// const ASCII_WHITESPACE: &'static str = r" \t\n\r";


pub struct UString {
    soft: Vec<u8>,
    ascii: Vec<u8>,
    refuse :Option<Vec<u8>>,
}

impl fmt::Display for UString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.soft_word())
    }
}
impl fmt::Debug for UString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let gbg = self.garbage();
        write!(fmt, "utf8-string: {}", self.soft_word())?;
        write!(fmt, "\r\ngarbage: 0x{}", hex::encode(gbg))
    }
}

impl UString {
    pub fn new(input: &[u8]) -> UString {
        let mut input = input;
        let mut string: Vec<u8> = Vec::new();
        let mut ascii: Vec<u8> = Vec::new();
        let mut refuse: Vec<u8> = Vec::new();
        let mut rng = thread_rng();
        loop {
            match from_utf8(input) {
                Ok(valid) => {
                    string.extend(valid.as_bytes().iter().filter(|c: &&u8| (**c).is_ascii() && !(**c).is_ascii_whitespace() && match **c {
                        0x3b | 0x05 | 0x15| 0x1f | 0x13 | 0x20 | 0x0a | 0x0b | 0x7f => false,
                            y => y >= 30 && y <= 126,
                        }).collect::<Vec<&u8>>());
                    break;
                }
                Err(error) => {
                    let (valid, after_valid) = input.split_at(error.valid_up_to());
                    string.extend(valid);
                    if let Some(il) = error.error_len() {
                        let is = error.valid_up_to();
                        let ie = is + il;
                        let rf = &input[is..ie];

                        let secret = (&mut rng).sample_iter(Standard).take(rf.len()).filter(|c: &u8| (*c).is_ascii() && !(*c).is_ascii_whitespace() && match *c {
                            0x05 | 0x15| 0x1f | 0x13 | 0x20 | 0x0a | 0x0b | 0x7f => false,
                            y => y >= 30 && y <= 126,
                        })
                            .collect::<Vec<u8>>();
                        string.extend(&secret);

                        refuse.extend(rf);
                        input = &after_valid[il..];
                    } else {
                        break;
                    }
                }
            }
        }

        for (i, c) in string.clone().iter().enumerate() {
            if c.is_ascii() && !c.is_ascii_whitespace() {
                ascii.push(*c);
            } else {
                let mut pool = ASCII_PRINTABLE.as_bytes().to_vec();
                pool.shuffle(&mut rng);
                ascii.push(pool[i % pool.len()]);
                pool.shuffle(&mut rng);
                string[i] = pool[i % pool.len()];
            }
        }
        UString{
            soft: string,
            ascii: ascii,
            refuse: if refuse.len() > 0 {
                Some(refuse)
            } else {
                None
            },
        }
    }
    pub fn soft_word(&self) -> String {
        String::from_utf8(self.soft.clone()).expect("garbage to evidently possess some data")
    }
    pub fn ascii(&self) -> String {
        String::from_utf8(self.ascii.clone()).expect("unexpected")
    }
    pub fn soft_string(&self) -> Vec<u8> {
        self.soft.clone()
    }
    pub fn garbage(&self) -> Vec<u8> {
        match &self.refuse {
            None => Vec::new(),
            Some(v) => v.clone(),
        }
    }
}


#[cfg(test)]
mod rfc4251_tests {
    use super::UString;

    #[test]
    pub fn test_sstring_from_invalid_utf8() {
        let invalid_utf8_bytes: Vec<u8> = vec![
            0x34, 0xff, 0xff, 0xff, 0x3d, 0xfe, 0x37, 0xfd, 0x25, 0x33, 0xff
        ];

        assert_eq!(
            String::from_utf8_lossy(&invalid_utf8_bytes),
            format!("4���=�7�%3�")
        );
        let utf8_string = UString::new(&invalid_utf8_bytes);
        let ss = utf8_string.soft_string();
        assert_eq!(ss[0], b'4');
        assert_eq!(ss[4], b'=');
        assert_eq!(ss[6], b'7');
        assert_eq!(ss[8], b'%');
        assert_eq!(ss[9], b'3');
        let sw = utf8_string.soft_word();
        assert_ne!(sw, format!("r1G1"));
        let gbg = utf8_string.garbage();
        assert_eq!(gbg, vec![0xff, 0xff, 0xff, 0xfe, 0xfd, 0xff]);
    }
}
