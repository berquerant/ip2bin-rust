# ip2bin-rust

This is a rewrite of [ip2bin](https://github.com/berquerant/ip2bin) in Rust.

```shell
❯ ip2bin
IP address conversion utilities

Usage: ip2bin <COMMAND>

Commands:
  op       Operations on addresses
  mask     Display mask [alias: m]
  inspect  Inspect CIDR [alias: i]
  expand   Expand CIDR [alias: e]
  in       Determine whether CIDR contain ADDRESS
  conv     Converts ip address format [alias: c]
  mcp      Start Model Context Protocol (MCP) server
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## MCP Server

`ip2bin` can run as a [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server over stdio.

### Usage in MCP Client Settings

Example configuration for MCP client configuration:

```json
{
  "mcpServers": {
    "ip2bin": {
      "command": "ip2bin",
      "args": ["mcp"]
    }
  }
}
```

### Supported Tools

- `mask`: Get subnet mask address for a given prefix bit length.
- `inspect`: Inspect an IPv4 CIDR block and return detailed network information.
- `expand`: Expand an IPv4 CIDR block into host addresses or subnets.
- `contains`: Check if an IPv4 CIDR block contains a given IPv4 address.
- `conv`: Convert an IPv4 address between binary, decimal, integer, abbreviated binary, and dotted binary representations.
- `op_and`, `op_or`, `op_xor`, `op_not`, `op_ls`, `op_rs`: Bitwise operations and shifts on IPv4 addresses.
