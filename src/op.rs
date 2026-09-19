use std::net::Ipv4Addr;

/// Bitwise AND across multiple IPv4 addresses.
pub fn op_and(addresses: &[Ipv4Addr]) -> Ipv4Addr {
    let init = Ipv4Addr::from(u32::MAX);
    addresses.iter().fold(init, |acc, &x| acc & x)
}

/// Bitwise OR across multiple IPv4 addresses.
pub fn op_or(addresses: &[Ipv4Addr]) -> Ipv4Addr {
    let init = Ipv4Addr::from(0);
    addresses.iter().fold(init, |acc, &x| acc | x)
}

/// Bitwise XOR across multiple IPv4 addresses.
pub fn op_xor(addresses: &[Ipv4Addr]) -> Option<Ipv4Addr> {
    addresses.iter().copied().reduce(|acc, x| {
        let left = u32::from(acc);
        let right = u32::from(x);
        Ipv4Addr::from(left ^ right)
    })
}

/// Bitwise NOT on an IPv4 address.
pub fn op_not(address: Ipv4Addr) -> Ipv4Addr {
    !address
}

/// Left shift on an IPv4 address.
pub fn op_ls(address: Ipv4Addr, bit: u8) -> Ipv4Addr {
    let b = u32::from(address);
    let c = b << bit;
    Ipv4Addr::from(c)
}

/// Right shift on an IPv4 address.
pub fn op_rs(address: Ipv4Addr, bit: u8) -> Ipv4Addr {
    let b = u32::from(address);
    let c = b >> bit;
    Ipv4Addr::from(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_and() {
        let a = "192.168.1.10".parse().unwrap();
        let b = "255.255.255.0".parse().unwrap();
        assert_eq!(op_and(&[a, b]), "192.168.1.0".parse::<Ipv4Addr>().unwrap());
    }

    #[test]
    fn test_op_or() {
        let a = "192.168.1.0".parse().unwrap();
        let b = "0.0.0.255".parse().unwrap();
        assert_eq!(op_or(&[a, b]), "192.168.1.255".parse::<Ipv4Addr>().unwrap());
    }

    #[test]
    fn test_op_xor() {
        let a = "192.168.1.1".parse().unwrap();
        assert_eq!(op_xor(&[a, a]), Some("0.0.0.0".parse::<Ipv4Addr>().unwrap()));
    }

    #[test]
    fn test_op_not() {
        let a = "255.255.255.0".parse().unwrap();
        assert_eq!(op_not(a), "0.0.0.255".parse::<Ipv4Addr>().unwrap());
    }

    #[test]
    fn test_op_ls() {
        let a = "192.168.1.1".parse().unwrap();
        assert_eq!(op_ls(a, 8), "168.1.1.0".parse::<Ipv4Addr>().unwrap());
    }

    #[test]
    fn test_op_rs() {
        let a = "192.168.1.1".parse().unwrap();
        assert_eq!(op_rs(a, 8), "0.192.168.1".parse::<Ipv4Addr>().unwrap());
    }
}
