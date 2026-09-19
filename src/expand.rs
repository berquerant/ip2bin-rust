use ip_network::Ipv4Network;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ExpandError {
    #[error("prefix must be between 0 and 32, got {0}")]
    InvalidPrefix(u8),
}

/// Expand a CIDR block into host addresses or subnets.
pub fn expand(cidr: Ipv4Network, prefix: Option<u8>) -> Result<Vec<String>, ExpandError> {
    match prefix {
        None => Ok(cidr.hosts().map(|x| x.to_string()).collect()),
        Some(p) => {
            if p > 32 {
                return Err(ExpandError::InvalidPrefix(p));
            }
            Ok(cidr.subnets_with_prefix(p).map(|x| x.to_string()).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_expand_hosts() {
        let cidr = Ipv4Network::from_str("192.168.1.0/30").unwrap();
        let hosts = expand(cidr, None).unwrap();
        assert_eq!(hosts, vec!["192.168.1.1", "192.168.1.2"]);
    }

    #[test]
    fn test_expand_subnets() {
        let cidr = Ipv4Network::from_str("192.168.1.0/24").unwrap();
        let subnets = expand(cidr, Some(25)).unwrap();
        assert_eq!(subnets, vec!["192.168.1.0/25", "192.168.1.128/25"]);
    }

    #[test]
    fn test_expand_invalid_prefix() {
        let cidr = Ipv4Network::from_str("192.168.1.0/24").unwrap();
        assert_eq!(expand(cidr, Some(33)), Err(ExpandError::InvalidPrefix(33)));
    }
}
