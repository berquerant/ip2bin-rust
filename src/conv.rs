use crate::parse::Bin;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConvResult {
    bin: String,
    dec: String,
    int: u32,
    abbrev: String,
    dbin: String,
}

impl From<Ipv4Addr> for ConvResult {
    fn from(item: Ipv4Addr) -> Self {
        let mut b = Bin::from(item);
        let bin = String::from(b.clone());
        let dec = item.to_string();
        let int = u32::from(item);
        b.rstrip(false);
        let abbrev = String::from(b);
        let v: Vec<String> = item
            .octets()
            .into_iter()
            .map(|x| {
                let mut b = Bin::from(x as u64);
                b.pad_start(8, false);
                String::from(b)
            })
            .collect();
        let dbin = v.join(".");

        ConvResult {
            bin,
            dec,
            int,
            abbrev,
            dbin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_conv_result_from {
        ($name:ident, $ip:expr, $want:expr) => {
            #[test]
            fn $name() {
                let got = ConvResult::from($ip);
                assert_eq!($want, got);
            }
        };
    }

    test_conv_result_from!(
        test_conv_result_from_0,
        Ipv4Addr::new(192, 168, 1, 4),
        ConvResult {
            bin: "11000000101010000000000100000100".to_string(),
            dec: "192.168.1.4".to_string(),
            int: 3232235780,
            abbrev: "110000001010100000000001000001".to_string(),
            dbin: "11000000.10101000.00000001.00000100".to_string(),
        }
    );
}
