use ip_network::Ipv4Network;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct NetworkInfo {
    pub cidr: String,
    pub mask: String,
    pub network: String,
    pub hosts: usize,
    pub start: Option<String>,
    pub end: Option<String>,
    pub broadcast: String,
    pub is_local_identification: bool,
    pub is_unspecified: bool,
    pub is_loopback: bool,
    pub is_broadcast: bool,
    pub is_private: bool,
    pub is_ietf_protocol_assignments: bool,
    pub is_shared_address_space: bool,
    pub is_link_local: bool,
    pub is_multicast: bool,
    pub is_benchmarking: bool,
    pub is_reserved: bool,
    pub is_documentation: bool,
    pub is_global: bool,
}

impl From<Ipv4Network> for NetworkInfo {
    fn from(item: Ipv4Network) -> Self {
        let mut hosts = item.hosts();
        NetworkInfo {
            cidr: item.to_string(),
            mask: item.full_netmask().to_string(),
            network: item.network_address().to_string(),
            hosts: hosts.len(),
            start: hosts.next().map(|x| x.to_string()),
            end: hosts.last().map(|x| x.to_string()),
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
