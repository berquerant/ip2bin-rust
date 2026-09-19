use crate::conv::ConvResult;
use crate::inspect::NetworkInfo;
use crate::mask::bits_address;
use crate::parse::Bin;
use ip_network::Ipv4Network;
use rmcp::{
    handler::server::wrapper::{Json, Parameters},
    schemars,
    tool,
    tool_router,
    ErrorData,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct Ip2binMcpServer;

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct MaskParams {
    #[schemars(description = "Prefix length (0-32)")]
    pub bit: u8,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct InspectParams {
    #[schemars(description = "IPv4 CIDR notation (e.g. 192.168.1.0/24)")]
    pub cidr: String,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct ExpandParams {
    #[schemars(description = "IPv4 CIDR notation (e.g. 192.168.1.0/24)")]
    pub cidr: String,
    #[schemars(description = "Optional subnet prefix length to expand into (0-32)")]
    pub prefix: Option<u8>,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct InParams {
    #[schemars(description = "IPv4 CIDR notation (e.g. 192.168.1.0/24)")]
    pub cidr: String,
    #[schemars(description = "IPv4 address (e.g. 192.168.1.5)")]
    pub address: String,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum ConvCategoryParam {
    #[default]
    Bin,
    Dec,
    Int,
    Abbrev,
    Dbin,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct ConvParams {
    #[schemars(description = "Format category of input: 'bin', 'dec', 'int', 'abbrev', or 'dbin'")]
    pub category: ConvCategoryParam,
    #[schemars(description = "Target representation to convert")]
    pub target: String,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct OpNotParams {
    #[schemars(description = "IPv4 address")]
    pub address: String,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct OpShiftParams {
    #[schemars(description = "IPv4 address")]
    pub address: String,
    #[schemars(description = "Shift bit count (0-32)")]
    pub bit: u8,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct OpBitwiseParams {
    #[schemars(description = "List of IPv4 addresses")]
    pub addresses: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct MaskResult {
    pub mask: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ExpandResult {
    pub items: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ContainsResult {
    pub contains: bool,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct AddressResult {
    pub address: String,
}

#[tool_router(server_handler)]
impl Ip2binMcpServer {
    #[tool(description = "Get subnet mask address for a given prefix bit length")]
    fn mask(&self, Parameters(params): Parameters<MaskParams>) -> Result<Json<MaskResult>, ErrorData> {
        if params.bit > 32 {
            return Err(ErrorData::invalid_params("bit must be between 0 and 32", None));
        }
        let addr = bits_address(params.bit);
        Ok(Json(MaskResult {
            mask: addr.to_string(),
        }))
    }

    #[tool(description = "Inspect an IPv4 CIDR block and return detailed network information")]
    fn inspect(&self, Parameters(params): Parameters<InspectParams>) -> Result<Json<NetworkInfo>, ErrorData> {
        let cidr = Ipv4Network::from_str(&params.cidr)
            .map_err(|e| ErrorData::invalid_params(format!("Invalid CIDR: {e}"), None))?;
        let info = NetworkInfo::from(cidr);
        Ok(Json(info))
    }

    #[tool(description = "Expand an IPv4 CIDR block into host addresses or subnets")]
    fn expand(&self, Parameters(params): Parameters<ExpandParams>) -> Result<Json<ExpandResult>, ErrorData> {
        let cidr = Ipv4Network::from_str(&params.cidr)
            .map_err(|e| ErrorData::invalid_params(format!("Invalid CIDR: {e}"), None))?;

        let items: Vec<String> = match params.prefix {
            None => cidr.hosts().map(|x| x.to_string()).collect(),
            Some(p) => {
                if p > 32 {
                    return Err(ErrorData::invalid_params("prefix must be between 0 and 32", None));
                }
                cidr.subnets_with_prefix(p).map(|x| x.to_string()).collect()
            }
        };

        Ok(Json(ExpandResult { items }))
    }

    #[tool(description = "Check if an IPv4 CIDR block contains a given IPv4 address")]
    fn contains(&self, Parameters(params): Parameters<InParams>) -> Result<Json<ContainsResult>, ErrorData> {
        let cidr = Ipv4Network::from_str(&params.cidr)
            .map_err(|e| ErrorData::invalid_params(format!("Invalid CIDR: {e}"), None))?;
        let address = Ipv4Addr::from_str(&params.address)
            .map_err(|e| ErrorData::invalid_params(format!("Invalid address: {e}"), None))?;

        Ok(Json(ContainsResult {
            contains: cidr.contains(address),
        }))
    }

    #[tool(description = "Convert an IPv4 address between binary, decimal, integer, abbreviated binary, and dotted binary representations")]
    fn conv(&self, Parameters(params): Parameters<ConvParams>) -> Result<Json<ConvResult>, ErrorData> {
        let addr = match params.category {
            ConvCategoryParam::Bin => {
                let x = Bin::from(params.target);
                Ipv4Addr::try_from(x).map_err(|e| ErrorData::invalid_params(format!("Invalid binary: {e:?}"), None))?
            }
            ConvCategoryParam::Dec => {
                Ipv4Addr::from_str(&params.target).map_err(|e| ErrorData::invalid_params(format!("Invalid decimal IP: {e}"), None))?
            }
            ConvCategoryParam::Int => {
                let x: u32 = params.target.parse().map_err(|e| ErrorData::invalid_params(format!("Invalid integer: {e}"), None))?;
                Ipv4Addr::from(x)
            }
            ConvCategoryParam::Abbrev => {
                let mut x = Bin::from(params.target);
                x.pad_end(Ipv4Addr::BITS as usize, false);
                Ipv4Addr::try_from(x).map_err(|e| ErrorData::invalid_params(format!("Invalid binary: {e:?}"), None))?
            }
            ConvCategoryParam::Dbin => {
                let s: String = params.target.chars().filter(|x| *x == '0' || *x == '1').collect();
                let x = Bin::from(s);
                Ipv4Addr::try_from(x).map_err(|e| ErrorData::invalid_params(format!("Invalid dotted binary: {e:?}"), None))?
            }
        };

        let result = ConvResult::from(addr);
        Ok(Json(result))
    }

    #[tool(description = "Bitwise AND operation across multiple IPv4 addresses")]
    fn op_and(&self, Parameters(params): Parameters<OpBitwiseParams>) -> Result<Json<AddressResult>, ErrorData> {
        if params.addresses.is_empty() {
            return Err(ErrorData::invalid_params("addresses cannot be empty", None));
        }
        let mut addrs = Vec::new();
        for a in params.addresses {
            addrs.push(Ipv4Addr::from_str(&a).map_err(|e| ErrorData::invalid_params(format!("Invalid address '{a}': {e}"), None))?);
        }
        let init = Ipv4Addr::from(u32::MAX);
        let res = addrs.iter().fold(init, |acc, x| acc & x);
        Ok(Json(AddressResult {
            address: res.to_string(),
        }))
    }

    #[tool(description = "Bitwise OR operation across multiple IPv4 addresses")]
    fn op_or(&self, Parameters(params): Parameters<OpBitwiseParams>) -> Result<Json<AddressResult>, ErrorData> {
        if params.addresses.is_empty() {
            return Err(ErrorData::invalid_params("addresses cannot be empty", None));
        }
        let mut addrs = Vec::new();
        for a in params.addresses {
            addrs.push(Ipv4Addr::from_str(&a).map_err(|e| ErrorData::invalid_params(format!("Invalid address '{a}': {e}"), None))?);
        }
        let init = Ipv4Addr::from(0);
        let res = addrs.iter().fold(init, |acc, x| acc | x);
        Ok(Json(AddressResult {
            address: res.to_string(),
        }))
    }

    #[tool(description = "Bitwise XOR operation across multiple IPv4 addresses")]
    fn op_xor(&self, Parameters(params): Parameters<OpBitwiseParams>) -> Result<Json<AddressResult>, ErrorData> {
        if params.addresses.is_empty() {
            return Err(ErrorData::invalid_params("addresses cannot be empty", None));
        }
        let mut addrs = Vec::new();
        for a in params.addresses {
            addrs.push(Ipv4Addr::from_str(&a).map_err(|e| ErrorData::invalid_params(format!("Invalid address '{a}': {e}"), None))?);
        }
        let res = addrs
            .into_iter()
            .reduce(|acc, x| {
                let left = u32::from(acc);
                let right = u32::from(x);
                Ipv4Addr::from(left ^ right)
            })
            .unwrap();
        Ok(Json(AddressResult {
            address: res.to_string(),
        }))
    }

    #[tool(description = "Bitwise NOT operation on an IPv4 address")]
    fn op_not(&self, Parameters(params): Parameters<OpNotParams>) -> Result<Json<AddressResult>, ErrorData> {
        let addr = Ipv4Addr::from_str(&params.address)
            .map_err(|e| ErrorData::invalid_params(format!("Invalid address: {e}"), None))?;
        let res = !addr;
        Ok(Json(AddressResult {
            address: res.to_string(),
        }))
    }

    #[tool(description = "Bitwise left-shift operation on an IPv4 address")]
    fn op_ls(&self, Parameters(params): Parameters<OpShiftParams>) -> Result<Json<AddressResult>, ErrorData> {
        if params.bit > 32 {
            return Err(ErrorData::invalid_params("bit must be between 0 and 32", None));
        }
        let addr = Ipv4Addr::from_str(&params.address)
            .map_err(|e| ErrorData::invalid_params(format!("Invalid address: {e}"), None))?;
        let b = u32::from(addr);
        let c = b << params.bit;
        let res = Ipv4Addr::from(c);
        Ok(Json(AddressResult {
            address: res.to_string(),
        }))
    }

    #[tool(description = "Bitwise right-shift operation on an IPv4 address")]
    fn op_rs(&self, Parameters(params): Parameters<OpShiftParams>) -> Result<Json<AddressResult>, ErrorData> {
        if params.bit > 32 {
            return Err(ErrorData::invalid_params("bit must be between 0 and 32", None));
        }
        let addr = Ipv4Addr::from_str(&params.address)
            .map_err(|e| ErrorData::invalid_params(format!("Invalid address: {e}"), None))?;
        let b = u32::from(addr);
        let c = b >> params.bit;
        let res = Ipv4Addr::from(c);
        Ok(Json(AddressResult {
            address: res.to_string(),
        }))
    }
}

pub async fn run_mcp_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use rmcp::serve_server;
    use rmcp::transport::io::stdio;
    let server = Ip2binMcpServer::default();
    let service = serve_server(server, stdio()).await?;
    service.waiting().await?;
    Ok(())
}
