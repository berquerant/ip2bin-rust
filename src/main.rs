mod conv;
mod inspect;
mod mask;
mod parse;
use clap::{self, Args, Parser, Subcommand, ValueEnum};
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
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    Mask {
        #[arg(value_name = "BIT", num_args = 1, value_parser = clap::value_parser!(u64).range(0..=Ipv4Addr::BITS as u64))]
        bit: u64,
    },

    /// Inspect CIDR
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    Inspect {
        #[arg(value_name = "CIDR", num_args = 1, value_parser = Ipv4Network::from_str)]
        cidr: Ipv4Network,
    },

    /// Expand CIDR
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
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
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
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
}

#[derive(Debug, Args)]
struct OpArgs {
    #[command(subcommand)]
    command: OpCommands,
}

#[derive(Debug, Subcommand)]
enum OpCommands {
    /// Bit and
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    And {
        #[arg(value_name="ADDRESS", num_args=1.., value_parser = Ipv4Addr::from_str)]
        addresses: Vec<Ipv4Addr>,
    },
    /// Bit or
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    Or {
        #[arg(value_name="ADDRESS", num_args=1.., value_parser = Ipv4Addr::from_str)]
        addresses: Vec<Ipv4Addr>,
    },
    /// Bit xor
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    Xor {
        #[arg(value_name="ADDRESS", num_args=1..,value_parser = Ipv4Addr::from_str)]
        addresses: Vec<Ipv4Addr>,
    },
    /// Bit not
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    Not {
        #[arg(value_name = "ADDRESS", num_args = 1, value_parser = Ipv4Addr::from_str)]
        address: Ipv4Addr,
    },
    /// Left shift
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    LS {
        #[arg(value_name = "ADDRESS", num_args = 1, value_parser = Ipv4Addr::from_str)]
        address: Ipv4Addr,
        #[arg(value_name = "BIT", num_args = 1, value_parser = clap::value_parser!(u64).range(0..=Ipv4Addr::BITS as u64))]
        bit: u64,
    },
    /// Right shift
    #[command(about, verbatim_doc_comment, arg_required_else_help = true)]
    RS {
        #[arg(value_name = "ADDRESS", num_args = 1, value_parser = Ipv4Addr::from_str)]
        address: Ipv4Addr,
        #[arg(value_name = "BIT", num_args = 1, value_parser = clap::value_parser!(u64).range(0..=Ipv4Addr::BITS as u64))]
        bit: u64,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ConvCategory {
    Bin,
    Dec,
    Int,
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

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Expand { cidr, prefix } => match prefix {
            None => {
                let it = cidr.hosts();
                for x in it {
                    println!("{}", x);
                }
            }
            Some(p) => {
                let it = cidr.subnets_with_prefix(p as u8);
                for x in it {
                    println!("{}", x);
                }
            }
        },
        Commands::In { cidr, address } => {
            if !cidr.contains(address) {
                process::exit(1);
            }
        }
        Commands::Inspect { cidr } => {
            let r = inspect::NetworkInfo::from(cidr);
            let j = serde_json::to_string(&r).expect("jsonify NetworkInfo");
            println!("{}", j);
        }
        Commands::Conv { category, target } => {
            let a = match category {
                ConvCategory::Bin => {
                    let x = parse::Bin::from(target);
                    Ipv4Addr::try_from(x).expect("unsigned int 32")
                }
                ConvCategory::Dec => Ipv4Addr::from_str(&target).expect("ipv4 string"),
                ConvCategory::Int => {
                    let x: u32 = target.parse().expect("unsigned int 32");
                    Ipv4Addr::from(x)
                }
                ConvCategory::Abbrev => {
                    let mut x = parse::Bin::from(target);
                    x.pad_end(Ipv4Addr::BITS as usize, false);
                    Ipv4Addr::try_from(x).expect("unsigned int 32")
                }
                ConvCategory::Dbin => {
                    let s: String = target.chars().filter(|x| *x == '0' || *x == '1').collect();
                    let x = parse::Bin::from(s);
                    Ipv4Addr::try_from(x).expect("unsigned int 32")
                }
            };

            let r = conv::ConvResult::from(a);
            let j = serde_json::to_string(&r).expect("jsonify ConvResult");
            println!("{}", j);
        }
        Commands::Mask { bit } => {
            let a = mask::bits_address(bit as u8);
            println!("{}", a);
        }
        Commands::Op(op_args) => match op_args.command {
            OpCommands::And { addresses } => {
                let init = Ipv4Addr::from(u32::MAX);
                let a = addresses.iter().fold(init, |acc, x| acc & x);
                println!("{}", a);
            }
            OpCommands::Or { addresses } => {
                let init = Ipv4Addr::from(0);
                let a = addresses.iter().fold(init, |acc, x| acc | x);
                println!("{}", a);
            }
            OpCommands::Xor { addresses } => {
                let a = addresses
                    .into_iter()
                    .reduce(|acc, x| {
                        let left = u32::from(acc);
                        let right = u32::from(x);
                        let r = left ^ right;
                        Ipv4Addr::from(r)
                    })
                    .unwrap();
                println!("{}", a);
            }
            OpCommands::Not { address } => {
                let a = !address;
                println!("{}", a);
            }
            OpCommands::LS { address, bit } => {
                let b = u32::from(address);
                let c = b << bit;
                let a = Ipv4Addr::from(c);
                println!("{}", a);
            }
            OpCommands::RS { address, bit } => {
                let b = u32::from(address);
                let c = b >> bit;
                let a = Ipv4Addr::from(c);
                println!("{}", a);
            }
        },
    };
}
