use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct BigNum {
    left: Vec<u8>,
    right: Vec<u8>,
}

static U4_MASK: u8 = 0b1111;

impl BigNum {
    pub fn new() -> BigNum {
        BigNum {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn from_raw(mut left: Vec<u8>, mut right: Vec<u8>) -> BigNum {
        BigNum::trim_data(&mut left);
        BigNum::trim_data(&mut right);

        BigNum {
            left,
            right,
        }
    }

    fn trim_data(data: &mut Vec<u8>) {
        let mut to_trim = 0usize;
        for v in data.iter().rev() {
            if v == &0u8 {
                to_trim += 1;
            } else {
                break;
            }
        }

        data.truncate(data.len() - to_trim);
    }

    pub fn add_left(&mut self, other: &[u8]) {
        let mut carry = 0;
        for i in 0..other.len() {
            if self.left.len() > i {
                let tmp = other[i] + self.left[i] + carry;
                carry = if tmp > 9 { 1 } else { 0 };
                self.left[i] = tmp & U4_MASK;
            } else {
                let tmp = other[i] + carry;
                carry = if tmp > 9 { 1 } else { 0 };
                self.left.push(tmp & U4_MASK);
            }
        }
    }

    pub fn add_right(&mut self, other: &[u8]) {
        let offset;
        if other.len() > self.right.len() {
            let dif = other.len() - self.right.len();
            for i in /*other.len()-1-*/dif..other.len() {
                self.right.push(other[i]); // TODO: optimization?
            }
            offset = dif; // self.right.len() - offset
        } else {
            offset = 0;
        }

        let mut carry = 0;
        for i in (0..other.len()-offset).rev() {
            if i > 0 {
                let tmp = other[i] + self.right[i] + carry;
                carry = if tmp > 9 { 1 } else { 0 };
                self.right[i] = tmp & U4_MASK;
            } else {
                let tmp = other[i] + carry;
                carry = if tmp > 9 { 1 } else { 0 };
                self.left.push(tmp & U4_MASK);
                break;
            }
        }
    }
}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::with_capacity(self.left.len() + 1 + self.right.len());
        for v in self.left.iter().rev() {
            string.push((v + 48) as char);
        }
        string.push('.');
        for v in &self.right {
            string.push((v + 48) as char);
        }

        write!(f, "{}", string)
    }
}

impl FromStr for BigNum {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let divided_string: Vec<&[u8]> = s.split('.').map(|s| s.as_bytes()).collect();

        Ok(BigNum::from_raw(divided_string[0].iter().rev().map(|n| *n - 48).collect(), divided_string[1].iter().map(|n| *n - 48).collect()))
    }
}
