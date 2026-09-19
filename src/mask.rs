use std::net::Ipv4Addr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum MaskError {
    #[error("bit must be between 0 and 32, got {0}")]
    InvalidBit(u8),
}

/// Return `bit` mask, or error if bit > 32.
pub fn bits_address(bit: u8) -> Result<Ipv4Addr, MaskError> {
    if bit > 32 {
        return Err(MaskError::InvalidBit(bit));
    }
    let b = bits(bit);
    Ok(Ipv4Addr::from(b))
}

fn bits(bit: u8) -> u32 {
    let s = Ipv4Addr::BITS - bit as u32;
    let m = u32::MAX as u64;
    let r = m << s;
    r as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_bits {
        ($name:ident, $bit:expr, $want:expr) => {
            #[test]
            fn $name() {
                let got = bits($bit);
                assert_eq!($want, got);
            }
        };
    }

    test_bits!(test_bits_32, 32, 0xFFFFFFFF as u32);
    test_bits!(test_bits_24, 24, 0xFFFFFF00 as u32);
    test_bits!(test_bits_16, 16, 0xFFFF0000 as u32);
    test_bits!(test_bits_8, 8, 0xFF000000 as u32);
    test_bits!(test_bits_0, 0, 0x00000000 as u32);

    #[test]
    fn test_bits_address() {
        assert_eq!(
            bits_address(24).unwrap(),
            "255.255.255.0".parse::<Ipv4Addr>().unwrap()
        );
        assert_eq!(bits_address(33), Err(MaskError::InvalidBit(33)));
    }
}
