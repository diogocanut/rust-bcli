//
// constants.rs
//
//

// System and execution

pub const DEBUG_FLAG: bool = false;

// Options

pub const HELP: &str = "?";
pub const ADDR_INFO: &str = "addrinfo";
pub const COLOR: &str = "color";
pub const CONF: &str = "conf";
pub const DATA_DIR: &str = "datadir";
pub const GENERATE: &str = "generate";
pub const GET_INFO: &str = "getinfo";
pub const NAMED: &str = "named";
pub const NET_INFO: &str = "netinfo";
pub const RPC_CLIENT_TIMEOUT: &str = "rpcclienttimeout";
pub const RPC_CONNECT: &str = "rpcconnect";
pub const RPC_COOKIE_FILE: &str = "rpccookiefile";
pub const RPC_PASSWORD: &str = "rpcpassword";
pub const RPC_PORT: &str = "rpcport";
pub const RPC_USER: &str = "rpcuser";
pub const RPC_WAIT: &str = "rpcwait";
pub const RPC_TIMEOUT: &str = "rpcwaittimeout";
pub const RPC_WALLET: &str = "rpcwallet";
pub const STDIN: &str = "stdin";
pub const STDIN_RPC_PASS: &str = "stdinrpcpass";
pub const STDIN_WALLET_PASSPHRASE: &str = "stdinwalletpassphrase";
pub const VERSION: &str = "version";
pub const CHAIN: &str = "chain";
pub const SIGNET: &str = "signet";
pub const SIGNET_CHALLENGE: &str = "signetchallenge";
pub const SIGNET_SEED_NODE: &str = "signetseednode";
pub const TESTNET: &str = "testnet";

// Option messages

pub const HELP_MSG: &str = "Print this help message and exit";

pub const ADDR_INFO_MSG: &str = "Get the number of addresses known to the node, per network and total, \
                                after filtering for quality and recency. The total number of \
                                addresses known to the node may be higher.";

pub const CONF_MSG: &str = "Specify configuration file. Relative paths will be prefixed by datadir \
                           location. (default: bitcoin.conf)";

pub const COLOR_MSG: &str = "Color setting for CLI output (default: auto). Valid values: always, auto \
                            (add color codes when standard output is connected to a terminal
                            and OS is not WIN32), never.";

pub const DATA_DIR_MSG: &str = "Specify data directory";

pub const GENERATE_MSG: &str = "Generate blocks, equivalent to RPC getnewaddress followed by RPC \
                                generatetoaddress. Optional positional integer arguments are \
                                number of blocks to generate (default: 1) and maximum iterations \
                                to try (default: 1000000), equivalent to RPC generatetoaddress \
                                nblocks and maxtries arguments. Example: bitcoin-cli -generate 4 \
                                1000";

pub const GET_INFO_MSG: &str = "Get general information from the remote server. Note that unlike \
                                server-side RPC calls, the output of -getinfo is the result of \
                                multiple non-atomic requests. Some entries in the output may \
                                represent results from different states (e.g. wallet balance may \
                                be as of a different block from the chain state reported)";

pub const NAMED_MSG: &str = "Pass named instead of positional arguments (default: false)";

pub const NET_INFO_MSG: &str = "Get network peer connection information from the remote server. An \
                                optional integer argument from 0 to 4 can be passed for different \
                                peers listings (default: 0). Pass \"help\" for detailed help \
                                documentation.";

pub const RPC_CLIENT_TIMEOUT_MSG: &str = "Timeout in seconds during HTTP requests, or 0 for no timeout. (default: \
                                          900)";

pub const RPC_CONNECT_MSG: &str = "Send commands to node running on <ip> (default: 127.0.0.1)";

pub const RPC_COOKIE_FILE_MSG: &str = "Location of the auth cookie. Relative paths will be prefixed by a \
                                      net-specific datadir location. (default: data dir)";

pub const RPC_PASSWORD_MSG: &str = "Password for JSON-RPC connections";

pub const RPC_PORT_MSG: &str = "Connect to JSON-RPC on <port> (default: 8332, testnet: 18332, signet: \
                                38332, regtest: 18443)";

pub const RPC_USER_MSG: &str = "Username for JSON-RPC connections";

pub const RPC_WAIT_MSG: &str = "Wait for RPC server to start";

pub const RPC_TIMEOUT_MSG: &str = "Timeout in seconds to wait for the RPC server to start, or 0 for no \
                                  timeout. (default: 0)";

pub const RPC_WALLET_MSG: &str = "Send RPC for non-default wallet on RPC server (needs to exactly match \
                                  corresponding -wallet option passed to bitcoind). This changes \
                                  the RPC endpoint used, e.g. \
                                  http://127.0.0.1:8332/wallet/<walletname>";

pub const STDIN_MSG: &str = "Read extra arguments from standard input, one per line until EOF/Ctrl-D \
                            (recommended for sensitive information such as passphrases). When \
                            combined with -stdinrpcpass, the first line from standard input \
                            is used for the RPC password.";

pub const STDIN_RPC_PASS_MSG: &str = "Read RPC password from standard input as a single line. When combined \
                                      with -stdin, the first line from standard input is used for the \
                                      RPC password. When combined with -stdinwalletpassphrase, \
                                      -stdinrpcpass consumes the first line, and -stdinwalletpassphrase \
                                      consumes the second.";

pub const STDIN_WALLET_PASSPHRASE_MSG: &str = "Read wallet passphrase from standard input as a single line. When \
                                              combined with -stdin, the first line from standard input is used \
                                              for the wallet passphrase.";

pub const VERSION_MSG: &str = "Print version and exit";

pub const CHAIN_MSG: &str = "Use the chain <chain> (default: main). Allowed values: main, test, \
                            signet, regtest";

pub const SIGNET_MSG: &str = "Use the signet chain. Equivalent to -chain=signet. Note that the network \
                              is defined by the -signetchallenge parameter";

pub const SIGNET_CHALLENGE_MSG: &str = "Blocks must satisfy the given script to be considered valid (only for \
                                        signet networks; defaults to the global default signet test \
                                        network challenge)";

pub const SIGNET_SEED_NODE_MSG: &str = "Specify a seed node for the signet network, in the hostname[:port] \
                                        format, e.g. sig.net:1234 (may be used multiple times to specify \
                                        multiple seed nodes; defaults to the global default signet test \
                                        network seed node(s))";

pub const TESTNET_MSG: &str = "Use the test chain. Equivalent to -chain=test.";
