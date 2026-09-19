use std::net::Ipv4Addr;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum OpError {
    #[error("addresses cannot be empty")]
    EmptyAddresses,
    #[error("invalid address: {0}")]
    InvalidAddress(String),
    #[error("shift bit must be between 0 and 32, got {0}")]
    InvalidShiftBit(u8),
}

/// Parse address strings into Ipv4Addrs.
pub fn parse_addrs<T: AsRef<str>>(addresses: &[T]) -> Result<Vec<Ipv4Addr>, OpError> {
    if addresses.is_empty() {
        return Err(OpError::EmptyAddresses);
    }
    addresses
        .iter()
        .map(|a| {
            let s = a.as_ref();
            Ipv4Addr::from_str(s).map_err(|_| OpError::InvalidAddress(s.to_string()))
        })
        .collect()
}

/// Bitwise AND across multiple IPv4 addresses.
pub fn op_and(addresses: &[Ipv4Addr]) -> Result<Ipv4Addr, OpError> {
    if addresses.is_empty() {
        return Err(OpError::EmptyAddresses);
    }
    let init = Ipv4Addr::from(u32::MAX);
    Ok(addresses.iter().fold(init, |acc, &x| acc & x))
}

/// Bitwise OR across multiple IPv4 addresses.
pub fn op_or(addresses: &[Ipv4Addr]) -> Result<Ipv4Addr, OpError> {
    if addresses.is_empty() {
        return Err(OpError::EmptyAddresses);
    }
    let init = Ipv4Addr::from(0);
    Ok(addresses.iter().fold(init, |acc, &x| acc | x))
}

/// Bitwise XOR across multiple IPv4 addresses.
pub fn op_xor(addresses: &[Ipv4Addr]) -> Result<Ipv4Addr, OpError> {
    addresses
        .iter()
        .copied()
        .reduce(|acc, x| {
            let left = u32::from(acc);
            let right = u32::from(x);
            Ipv4Addr::from(left ^ right)
        })
        .ok_or(OpError::EmptyAddresses)
}

/// Bitwise NOT on an IPv4 address.
pub fn op_not(address: Ipv4Addr) -> Ipv4Addr {
    !address
}

/// Left shift on an IPv4 address.
pub fn op_ls(address: Ipv4Addr, bit: u8) -> Result<Ipv4Addr, OpError> {
    if bit > 32 {
        return Err(OpError::InvalidShiftBit(bit));
    }
    let b = u32::from(address);
    let c = b << bit;
    Ok(Ipv4Addr::from(c))
}

/// Right shift on an IPv4 address.
pub fn op_rs(address: Ipv4Addr, bit: u8) -> Result<Ipv4Addr, OpError> {
    if bit > 32 {
        return Err(OpError::InvalidShiftBit(bit));
    }
    let b = u32::from(address);
    let c = b >> bit;
    Ok(Ipv4Addr::from(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_and() {
        let a = "192.168.1.10".parse().unwrap();
        let b = "255.255.255.0".parse().unwrap();
        assert_eq!(
            op_and(&[a, b]).unwrap(),
            "192.168.1.0".parse::<Ipv4Addr>().unwrap()
        );
    }

    #[test]
    fn test_op_or() {
        let a = "192.168.1.0".parse().unwrap();
        let b = "0.0.0.255".parse().unwrap();
        assert_eq!(
            op_or(&[a, b]).unwrap(),
            "192.168.1.255".parse::<Ipv4Addr>().unwrap()
        );
    }

    #[test]
    fn test_op_xor() {
        let a = "192.168.1.1".parse().unwrap();
        assert_eq!(
            op_xor(&[a, a]).unwrap(),
            "0.0.0.0".parse::<Ipv4Addr>().unwrap()
        );
    }

    #[test]
    fn test_op_not() {
        let a = "255.255.255.0".parse().unwrap();
        assert_eq!(op_not(a), "0.0.0.255".parse::<Ipv4Addr>().unwrap());
    }

    #[test]
    fn test_op_ls() {
        let a = "192.168.1.1".parse().unwrap();
        assert_eq!(
            op_ls(a, 8).unwrap(),
            "168.1.1.0".parse::<Ipv4Addr>().unwrap()
        );
        assert_eq!(op_ls(a, 33), Err(OpError::InvalidShiftBit(33)));
    }

    #[test]
    fn test_op_rs() {
        let a = "192.168.1.1".parse().unwrap();
        assert_eq!(
            op_rs(a, 8).unwrap(),
            "0.192.168.1".parse::<Ipv4Addr>().unwrap()
        );
        assert_eq!(op_rs(a, 33), Err(OpError::InvalidShiftBit(33)));
    }
}
