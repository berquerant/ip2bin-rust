use crate::parse::{Bin, ParseError};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::net::{AddrParseError, Ipv4Addr};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum ConvCategory {
    #[default]
    #[value(alias = "b")]
    Bin,
    #[value(alias = "d")]
    Dec,
    #[value(alias = "i")]
    Int,
    #[value(alias = "a")]
    Abbrev,
    Dbin,
}

impl std::fmt::Display for ConvCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ConvError {
    #[error("binary parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("decimal address parse error: {0}")]
    Addr(#[from] AddrParseError),
    #[error("integer parse error: {0}")]
    Int(#[from] ParseIntError),
}

impl ConvCategory {
    pub fn parse_target(&self, target: &str) -> Result<Ipv4Addr, ConvError> {
        match self {
            ConvCategory::Bin => {
                let x = Bin::from(target.to_string());
                Ok(Ipv4Addr::try_from(x)?)
            }
            ConvCategory::Dec => Ok(Ipv4Addr::from_str(target)?),
            ConvCategory::Int => {
                let x: u32 = target.parse()?;
                Ok(Ipv4Addr::from(x))
            }
            ConvCategory::Abbrev => {
                let mut x = Bin::from(target.to_string());
                x.pad_end(Ipv4Addr::BITS as usize, false);
                Ok(Ipv4Addr::try_from(x)?)
            }
            ConvCategory::Dbin => {
                let s: String = target.chars().filter(|&x| x == '0' || x == '1').collect();
                let x = Bin::from(s);
                Ok(Ipv4Addr::try_from(x)?)
            }
        }
    }
}

/// Helper function to perform format conversion in one call.
pub fn conv(category: ConvCategory, target: &str) -> Result<ConvResult, ConvError> {
    let addr = category.parse_target(target)?;
    Ok(ConvResult::from(addr))
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct ConvResult {
    pub bin: String,
    pub dec: String,
    pub int: u32,
    pub abbrev: String,
    pub dbin: String,
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

    #[test]
    fn test_parse_target() {
        let ip = Ipv4Addr::new(192, 168, 1, 4);
        assert_eq!(ConvCategory::Bin.parse_target("11000000101010000000000100000100").unwrap(), ip);
        assert_eq!(ConvCategory::Dec.parse_target("192.168.1.4").unwrap(), ip);
        assert_eq!(ConvCategory::Int.parse_target("3232235780").unwrap(), ip);
        assert_eq!(ConvCategory::Abbrev.parse_target("110000001010100000000001000001").unwrap(), ip);
        assert_eq!(ConvCategory::Dbin.parse_target("11000000.10101000.00000001.00000100").unwrap(), ip);
    }

    #[test]
    fn test_conv_fn() {
        let res = conv(ConvCategory::Dec, "192.168.1.4").unwrap();
        assert_eq!(res.dec, "192.168.1.4");
        assert_eq!(res.int, 3232235780);
    }
}
