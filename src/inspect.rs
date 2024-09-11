use ip_network::Ipv4Network;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NetworkInfo {
    cidr: String,
    mask: String,
    network: String,
    hosts: usize,
    start: String,
    end: String,
    broadcast: String,
    is_local_identification: bool,
    is_unspecified: bool,
    is_loopback: bool,
    is_broadcast: bool,
    is_private: bool,
    is_ietf_protocol_assignments: bool,
    is_shared_address_space: bool,
    is_link_local: bool,
    is_multicast: bool,
    is_benchmarking: bool,
    is_reserved: bool,
    is_documentation: bool,
    is_global: bool,
}

impl From<Ipv4Network> for NetworkInfo {
    fn from(item: Ipv4Network) -> Self {
        let mut hosts = item.hosts();
        NetworkInfo {
            cidr: item.to_string(),
            mask: item.full_netmask().to_string(),
            network: item.network_address().to_string(),
            hosts: hosts.len(),
            start: hosts.next().unwrap().to_string(),
            end: hosts.last().unwrap().to_string(),
            broadcast: item.broadcast_address().to_string(),
            is_local_identification: item.is_local_identification(),
            is_unspecified: item.is_unspecified(),
            is_loopback: item.is_loopback(),
            is_broadcast: item.is_broadcast(),
            is_private: item.is_private(),
            is_ietf_protocol_assignments: item.is_ietf_protocol_assignments(),
            is_shared_address_space: item.is_shared_address_space(),
            is_link_local: item.is_link_local(),
            is_multicast: item.is_multicast(),
            is_benchmarking: item.is_benchmarking(),
            is_reserved: item.is_reserved(),
            is_documentation: item.is_documentation(),
            is_global: item.is_global(),
        }
    }
}
