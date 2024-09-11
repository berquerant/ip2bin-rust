use std::cmp::PartialEq;
use std::convert::From;
use std::convert::TryFrom;
use std::net::Ipv4Addr;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParseError {
    #[error("too big")]
    TooBig,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bin(Vec<bool>);

impl Bin {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn pad_start(&mut self, size: usize, val: bool) {
        if self.len() >= size {
            return;
        }
        let s = size - self.len();
        for _ in 0..s {
            self.0.insert(0, val);
        }
    }
    pub fn pad_end(&mut self, size: usize, val: bool) {
        if self.len() >= size {
            return;
        }
        let s = size - self.len();
        self.0.extend(vec![val; s]);
    }
    pub fn rstrip(&mut self, val: bool) {
        loop {
            match self.0.pop() {
                Some(x) if x == val => continue,
                Some(x) => {
                    self.0.push(x);
                    return;
                }
                _ => return,
            }
        }
    }
}

impl TryFrom<Bin> for Ipv4Addr {
    type Error = ParseError;
    fn try_from(item: Bin) -> Result<Self, Self::Error> {
        let u = u64::from(item);
        if u > u32::MAX as u64 {
            Err(ParseError::TooBig)
        } else {
            Ok(Ipv4Addr::from(u as u32))
        }
    }
}

impl From<Ipv4Addr> for Bin {
    fn from(item: Ipv4Addr) -> Self {
        let u = u32::from(item);
        Bin::from(u as u64)
    }
}

impl From<u64> for Bin {
    fn from(item: u64) -> Self {
        let mut r: Vec<bool> = Vec::new();
        let mut acc = item;
        while acc > 0 {
            let lsb = acc & 1 == 1;
            r.push(lsb);
            acc >>= 1;
        }
        r.reverse();
        Bin(r)
    }
}

impl From<Bin> for u64 {
    fn from(item: Bin) -> Self {
        item.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| if *x { 1 << i } else { 0 })
            .sum()
    }
}

impl From<String> for Bin {
    fn from(item: String) -> Self {
        let r: Vec<_> = item.chars().map(|x| x != '0').collect();
        Bin(r)
    }
}

impl From<Bin> for String {
    fn from(item: Bin) -> Self {
        let r = item.0.iter().map(|x| if *x { '1' } else { '0' }).collect();
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_bin_rstrip {
        ($name:ident, $bin:expr, $val:expr, $want:expr) => {
            #[test]
            fn $name() {
                let mut b = $bin;
                b.rstrip($val);
                assert_eq!($want, b);
            }
        };
    }

    test_bin_rstrip!(test_bin_rstrip_empty_t, Bin(vec![]), true, Bin(vec![]));
    test_bin_rstrip!(test_bin_rstrip_t_t, Bin(vec![true]), true, Bin(vec![]));
    test_bin_rstrip!(test_bin_rstrip_t_f, Bin(vec![true]), false, Bin(vec![true]));
    test_bin_rstrip!(
        test_bin_rstrip_ft_t,
        Bin(vec![false, true]),
        true,
        Bin(vec![false])
    );
    test_bin_rstrip!(
        test_bin_rstrip_tf_t,
        Bin(vec![true, false]),
        true,
        Bin(vec![true, false])
    );
    test_bin_rstrip!(
        test_bin_rstrip_ftt_t,
        Bin(vec![false, true, true]),
        true,
        Bin(vec![false])
    );

    macro_rules! test_bin_pad {
        ($name:ident, $bin:expr, $size:expr, $val:expr, $start:expr, $end:expr) => {
            #[test]
            fn $name() {
                let mut b = $bin.clone();
                let mut c = $bin.clone();
                b.pad_start($size, $val);
                assert_eq!($start, b, "pad_start");
                c.pad_end($size, $val);
                assert_eq!($end, c, "pad_end");
            }
        };
    }

    test_bin_pad!(
        test_bin_pad_empty_0,
        Bin(vec![]),
        0,
        true,
        Bin(vec![]),
        Bin(vec![])
    );
    test_bin_pad!(
        test_bin_pad_empty_1t,
        Bin(vec![]),
        1,
        true,
        Bin(vec![true]),
        Bin(vec![true])
    );
    test_bin_pad!(
        test_bin_pad_1_0,
        Bin(vec![false]),
        0,
        true,
        Bin(vec![false]),
        Bin(vec![false])
    );
    test_bin_pad!(
        test_bin_pad_1_1,
        Bin(vec![false]),
        1,
        true,
        Bin(vec![false]),
        Bin(vec![false])
    );
    test_bin_pad!(
        test_bin_pad_1_2t,
        Bin(vec![false]),
        2,
        true,
        Bin(vec![true, false]),
        Bin(vec![false, true])
    );
    test_bin_pad!(
        test_bin_pad_1_3f,
        Bin(vec![true]),
        3,
        false,
        Bin(vec![false, false, true]),
        Bin(vec![true, false, false])
    );

    macro_rules! test_bin_str {
        ($name:ident, $bin:expr, $s:expr) => {
            #[test]
            fn $name() {
                let got = String::from($bin);
                assert_eq!($s, got, "String::from");
                let got = Bin::from($s);
                assert_eq!($bin, got, "Bin::from");
            }
        };
    }

    test_bin_str!(test_bin_str_empty, Bin(vec![]), "".to_string());
    test_bin_str!(test_bin_str_0, Bin(vec![false]), "0".to_string());
    test_bin_str!(test_bin_str_1, Bin(vec![true]), "1".to_string());
    test_bin_str!(test_bin_str_2, Bin(vec![true, false]), "10".to_string());

    macro_rules! test_bin_u64 {
        ($name:ident, $bin:expr, $ui:expr) => {
            #[test]
            fn $name() {
                let got = u64::from($bin);
                assert_eq!($ui, got, "u64::from");
                let got = Bin::from($ui);
                assert_eq!($bin, got, "Bin::from");
            }
        };
    }

    test_bin_u64!(test_bin_u64_empty, Bin(vec![]), 0);
    test_bin_u64!(test_bin_u64_1, Bin(vec![true]), 1);
    test_bin_u64!(test_bin_u64_2, Bin(vec![true, false]), 2);
}
