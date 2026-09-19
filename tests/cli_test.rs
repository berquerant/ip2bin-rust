use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

fn cmd() -> Command {
    Command::cargo_bin("ip2bin").unwrap()
}

macro_rules! test_cli_stdout {
    ($name:ident, $args:expr, $want:expr) => {
        #[test]
        fn $name() {
            cmd().args($args).assert().success().stdout($want);
        }
    };
}

macro_rules! test_cli_code {
    ($name:ident, $args:expr, $code:expr) => {
        #[test]
        fn $name() {
            cmd().args($args).assert().code($code);
        }
    };
}

macro_rules! test_cli_failure {
    ($name:ident, $args:expr) => {
        #[test]
        fn $name() {
            cmd().args($args).assert().failure();
        }
    };
}

macro_rules! test_cli_contains {
    ($name:ident, $args:expr, $($substr:expr),+ $(,)?) => {
        #[test]
        fn $name() {
            cmd().args($args).assert().success()
                $(.stdout(predicate::str::contains($substr)))+;
        }
    };
}

// op command
test_cli_stdout!(test_op_ls, ["op", "ls", "8", "192.168.1.1"], "168.1.1.0\n");
test_cli_stdout!(
    test_op_rs,
    ["op", "rs", "8", "192.168.1.1"],
    "0.192.168.1\n"
);
test_cli_stdout!(
    test_op_and,
    ["op", "and", "192.168.1.10", "255.255.255.0"],
    "192.168.1.0\n"
);
test_cli_stdout!(
    test_op_or,
    ["op", "or", "192.168.1.0", "0.0.0.255"],
    "192.168.1.255\n"
);
test_cli_stdout!(
    test_op_xor,
    ["op", "xor", "192.168.1.1", "192.168.1.1"],
    "0.0.0.0\n"
);
test_cli_stdout!(test_op_not, ["op", "not", "255.255.255.0"], "0.0.0.255\n");

// mask command
test_cli_stdout!(test_mask_28, ["mask", "28"], "255.255.255.240\n");
test_cli_stdout!(test_mask_24, ["mask", "24"], "255.255.255.0\n");

// expand command
test_cli_stdout!(
    test_expand_hosts,
    ["expand", "192.168.1.0/30"],
    "192.168.1.1\n192.168.1.2\n"
);
test_cli_stdout!(
    test_expand_subnets,
    ["expand", "192.168.1.0/24", "--prefix", "25"],
    "192.168.1.0/25\n192.168.1.128/25\n"
);

// in command
test_cli_code!(test_in_ok, ["in", "192.168.1.0/24", "192.168.1.10"], 0);
test_cli_code!(test_in_ng, ["in", "192.168.1.0/24", "10.0.0.1"], 1);

// conv command
test_cli_contains!(
    test_conv_dec,
    ["conv", "dec", "192.168.1.4"],
    r#""bin":"11000000101010000000000100000100""#,
    r#""dec":"192.168.1.4""#,
    r#""int":3232235780"#,
    r#""abbrev":"110000001010100000000001000001""#,
    r#""dbin":"11000000.10101000.00000001.00000100""#,
);
test_cli_contains!(
    test_conv_bin,
    ["conv", "bin", "11000000101010000000000100000100"],
    r#""dec":"192.168.1.4""#,
);
test_cli_contains!(
    test_conv_int,
    ["conv", "int", "3232235780"],
    r#""dec":"192.168.1.4""#,
);
test_cli_contains!(
    test_conv_abbrev,
    ["conv", "abbrev", "110000001010100000000001000001"],
    r#""dec":"192.168.1.4""#,
);
test_cli_contains!(
    test_conv_dbin,
    ["conv", "dbin", "11000000.10101000.00000001.00000100"],
    r#""dec":"192.168.1.4""#,
);

// inspect command
test_cli_contains!(
    test_inspect,
    ["inspect", "192.168.1.0/24"],
    r#""cidr":"192.168.1.0/24""#,
    r#""mask":"255.255.255.0""#,
    r#""network":"192.168.1.0""#,
    r#""broadcast":"192.168.1.255""#,
    r#""hosts":254"#,
    r#""start":"192.168.1.1""#,
    r#""end":"192.168.1.254""#,
    r#""is_private":true"#,
);

// Alias tests
test_cli_stdout!(test_alias_mask, ["m", "28"], "255.255.255.240\n");
test_cli_stdout!(
    test_alias_expand,
    ["e", "192.168.1.0/30"],
    "192.168.1.1\n192.168.1.2\n"
);
test_cli_contains!(
    test_alias_inspect,
    ["i", "192.168.1.0/24"],
    r#""cidr":"192.168.1.0/24""#,
);
test_cli_contains!(
    test_alias_conv_d,
    ["c", "d", "192.168.1.4"],
    r#""int":3232235780"#,
);
test_cli_contains!(
    test_alias_conv_b,
    ["c", "b", "11000000101010000000000100000100"],
    r#""dec":"192.168.1.4""#,
);
test_cli_contains!(
    test_alias_conv_i,
    ["c", "i", "3232235780"],
    r#""dec":"192.168.1.4""#,
);
test_cli_contains!(
    test_alias_conv_a,
    ["c", "a", "110000001010100000000001000001"],
    r#""dec":"192.168.1.4""#,
);
test_cli_stdout!(
    test_alias_op_and,
    ["op", "a", "192.168.1.10", "255.255.255.0"],
    "192.168.1.0\n"
);
test_cli_stdout!(
    test_alias_op_or,
    ["op", "o", "192.168.1.0", "0.0.0.255"],
    "192.168.1.255\n"
);
test_cli_stdout!(
    test_alias_op_xor,
    ["op", "x", "192.168.1.1", "192.168.1.1"],
    "0.0.0.0\n"
);
test_cli_stdout!(
    test_alias_op_not,
    ["op", "n", "255.255.255.0"],
    "0.0.0.255\n"
);
test_cli_stdout!(
    test_alias_op_ls,
    ["op", "l", "8", "192.168.1.1"],
    "168.1.1.0\n"
);
test_cli_stdout!(
    test_alias_op_rs,
    ["op", "r", "8", "192.168.1.1"],
    "0.192.168.1\n"
);

// Invalid argument tests
test_cli_failure!(test_invalid_mask, ["mask", "33"]);
test_cli_failure!(test_invalid_ip, ["conv", "dec", "not.an.ip"]);
test_cli_failure!(test_invalid_shift_order, ["op", "ls", "192.168.1.1", "8"]);

#[test]
fn test_mcp_initialize_and_call() {
    let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}"#;
    let initialized_notif = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
    let list_tools_req = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#;
    let call_mask_req = r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"mask","arguments":{"bit":24}}}"#;

    let input = format!("{init_req}\n{initialized_notif}\n{list_tools_req}\n{call_mask_req}\n");

    let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin("ip2bin"))
        .arg("mcp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn ip2bin mcp");

    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin.write_all(input.as_bytes()).expect("write to stdin");
    }

    let output = child.wait_with_output().expect("wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains(r#""name":"ip2bin""#) || stdout.contains(r#""serverInfo""#), "stdout: {stdout}");
    assert!(stdout.contains(r#""name":"mask""#), "stdout: {stdout}");
    assert!(stdout.contains(r#""mask":"255.255.255.0""#), "stdout: {stdout}");
}
