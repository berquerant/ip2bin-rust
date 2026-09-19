use clap::{self, Args, Parser, Subcommand};
use ip2bin::conv::{self, ConvCategory};
use ip2bin::expand;
use ip2bin::inspect::NetworkInfo;
use ip2bin::mask::bits_address;
use ip2bin::mcp;
use ip2bin::op;
use ip_network::Ipv4Network;
use std::net::Ipv4Addr;
use std::process;
use std::str::FromStr;

/// IP address conversion utilities
#[derive(Debug, Parser)]
#[command(name = "ip2bin")]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Operations on addresses
    Op(OpArgs),

    /// Display mask
    ///
    /// e.g. mask 28 then 255.255.255.240
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "m"
    )]
    Mask {
        #[arg(value_name = "BIT", num_args = 1, value_parser = clap::value_parser!(u64).range(0..=Ipv4Addr::BITS as u64))]
        bit: u64,
    },

    /// Inspect CIDR
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "i"
    )]
    Inspect {
        #[arg(value_name = "CIDR", num_args = 1, value_parser = Ipv4Network::from_str)]
        cidr: Ipv4Network,
    },

    /// Expand CIDR
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "e"
    )]
    Expand {
        #[arg(value_name = "CIDR", num_args = 1, value_parser = Ipv4Network::from_str)]
        cidr: Ipv4Network,
        /// Enumerate the subnetworks of the prefix length.
        #[arg(short, long, value_parser = clap::value_parser!(u64).range(0..=Ipv4Addr::BITS as u64))]
        prefix: Option<u64>,
    },

    /// Determine whether CIDR contain ADDRESS
    ///
    /// Exit with 0 if CIDR contain ADDRESS, else exit with 1.
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    In {
        #[arg(value_name = "CIDR", num_args = 1, value_parser = Ipv4Network::from_str)]
        cidr: Ipv4Network,
        #[arg(value_name = "ADDRESS", num_args = 1, value_parser = Ipv4Addr::from_str)]
        address: Ipv4Addr,
    },

    /// Converts ip address format
    ///
    /// You can optionally specify a 1st parameter as the format to convert to, followings are valid values:
    /// - bin: binary, e.g. 11000000101010000000000100000100
    /// - dec: string, e.g. 192.168.1.4
    /// - int: integer, e.g. 3232235780
    /// - abbrev: abbreviated binary address, e.g. 00001010 (00001010000000000000000000000000)
    /// - dbin: dotted binary, e.g. 01111111.00000000.00000000.00000001
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "c"
    )]
    Conv {
        #[arg(
            require_equals = true,
            value_name = "CATEGORY",
            num_args = 1,
            value_enum
        )]
        category: ConvCategory,
        #[arg(value_name = "TARGET", num_args = 1)]
        target: String,
    },

    /// Start Model Context Protocol (MCP) server
    #[command(about)]
    Mcp,
}

#[derive(Debug, Args)]
struct OpArgs {
    #[command(subcommand)]
    command: OpCommands,
}

#[derive(Debug, Subcommand)]
enum OpCommands {
    /// Bit and
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "a"
    )]
    And {
        #[arg(value_name="ADDRESS", num_args=1.., value_parser = Ipv4Addr::from_str)]
        addresses: Vec<Ipv4Addr>,
    },
    /// Bit or
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "o"
    )]
    Or {
        #[arg(value_name="ADDRESS", num_args=1.., value_parser = Ipv4Addr::from_str)]
        addresses: Vec<Ipv4Addr>,
    },
    /// Bit xor
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "x"
    )]
    Xor {
        #[arg(value_name="ADDRESS", num_args=1..,value_parser = Ipv4Addr::from_str)]
        addresses: Vec<Ipv4Addr>,
    },
    /// Bit not
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "n"
    )]
    Not {
        #[arg(value_name = "ADDRESS", num_args = 1, value_parser = Ipv4Addr::from_str)]
        address: Ipv4Addr,
    },
    /// Left shift
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "l"
    )]
    LS {
        #[arg(value_name = "BIT", num_args = 1, value_parser = clap::value_parser!(u64).range(0..=Ipv4Addr::BITS as u64))]
        bit: u64,
        #[arg(value_name = "ADDRESS", num_args = 1, value_parser = Ipv4Addr::from_str)]
        address: Ipv4Addr,
    },
    /// Right shift
    #[command(
        about,
        verbatim_doc_comment,
        arg_required_else_help = true,
        visible_alias = "r"
    )]
    RS {
        #[arg(value_name = "BIT", num_args = 1, value_parser = clap::value_parser!(u64).range(0..=Ipv4Addr::BITS as u64))]
        bit: u64,
        #[arg(value_name = "ADDRESS", num_args = 1, value_parser = Ipv4Addr::from_str)]
        address: Ipv4Addr,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Mcp => {
            if let Err(e) = mcp::run_mcp_server().await {
                eprintln!("MCP server error: {}", e);
                process::exit(1);
            }
        }
        Commands::Expand { cidr, prefix } => {
            let items = expand::expand(cidr, prefix.map(|p| p as u8)).expect("expand CIDR");
            for x in items {
                println!("{}", x);
            }
        }
        Commands::In { cidr, address } => {
            if !cidr.contains(address) {
                process::exit(1);
            }
        }
        Commands::Inspect { cidr } => {
            let r = NetworkInfo::from(cidr);
            let j = serde_json::to_string(&r).expect("jsonify NetworkInfo");
            println!("{}", j);
        }
        Commands::Conv { category, target } => {
            let r = conv::conv(category, &target).expect("valid conversion");
            let j = serde_json::to_string(&r).expect("jsonify ConvResult");
            println!("{}", j);
        }
        Commands::Mask { bit } => {
            let a = bits_address(bit as u8).expect("valid bit");
            println!("{}", a);
        }
        Commands::Op(op_args) => match op_args.command {
            OpCommands::And { addresses } => {
                let a = op::op_and(&addresses).expect("valid op_and");
                println!("{}", a);
            }
            OpCommands::Or { addresses } => {
                let a = op::op_or(&addresses).expect("valid op_or");
                println!("{}", a);
            }
            OpCommands::Xor { addresses } => {
                let a = op::op_xor(&addresses).expect("valid op_xor");
                println!("{}", a);
            }
            OpCommands::Not { address } => {
                let a = op::op_not(address);
                println!("{}", a);
            }
            OpCommands::LS { address, bit } => {
                let a = op::op_ls(address, bit as u8).expect("valid op_ls");
                println!("{}", a);
            }
            OpCommands::RS { address, bit } => {
                let a = op::op_rs(address, bit as u8).expect("valid op_rs");
                println!("{}", a);
            }
        },
    };
}
