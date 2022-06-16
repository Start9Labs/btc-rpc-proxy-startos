import { ConfigRes, ExpectedExports, matches, YAML } from "../deps.ts";

const { any, string, dictionary } = matches;

const matchConfig = dictionary([string, any]);

export const getConfig: ExpectedExports.getConfig = async (effects) => {
  const config = await effects
    .readFile({
      path: "start9/config.yaml",
      volumeId: "main",
    })
    .then((x) => YAML.parse(x))
    .then((x) => matchConfig.unsafeCast(x))
    .catch((e) => {
      effects.warn(`Got error ${e} while trying to read the config`);
      return undefined;
    });
  const spec: ConfigRes["spec"] = {
    "tor-address": {
      "name": "Tor Address",
      "description": "The Tor address for the main interface.",
      "type": "pointer",
      "subtype": "package",
      "package-id": "btc-rpc-proxy",
      "target": "tor-address",
      "interface": "main"
    },
    "bitcoind": {
      "type": "object",
      "name": "Bitcoin Core",
      "description": "The Bitcoin Core node to connect to",
      "spec": {
        "user": {
          "type": "pointer",
          "name": "RPC Username",
          "description": "The username for the RPC user for Bitcoin Core",
          "subtype": "package",
          "package-id": "bitcoind",
          "target": "config",
          "selector": "$.rpc.username",
          "multi": false
        },
        "password": {
          "type": "pointer",
          "name": "RPC Password",
          "description": "The password for the RPC user for Bitcoin Core",
          "subtype": "package",
          "package-id": "bitcoind",
          "target": "config",
          "selector": "$.rpc.password",
          "multi": false
        }
      }
    },
    "users": {
      "type": "list",
      "name": "RPC Users",
      "description": "Credentials and permissions for each RPC User",
      "range": "[0,*)",
      "subtype": "object",
      "spec": {
        "unique-by": "name",
        "display-as": "{{name}}",
        "spec": {
          "name": {
            "type": "string",
            "name": "Username",
            "description": "The username for the RPC User",
            "nullable": false,
            "default": "bitcoin",
            "copyable": true
          },
          "password": {
            "type": "string",
            "name": "Password",
            "description": "The password for the RPC User",
            "nullable": false,
            "copyable": true,
            "masked": true,
            "default": {
              "charset": "a-z,A-Z,0-9",
              "len": 22
            }
          },
          "allowed-calls": {
            "type": "list",
            "name": "Allowed Calls",
            "description": "The list of all RPC methods this user is allowed to make",
            "subtype": "enum",
            "range": "[0, *)",
            "spec": {
              "value-names": {},
              "values": [
                "abandontransaction",
                "abortrescan",
                "addmultisigaddress",
                "addnode",
                "analyzepsbt",
                "backupwallet",
                "bumpfee",
                "clearbanned",
                "combinepsbt",
                "combinerawtransaction",
                "converttopsbt",
                "createmultisig",
                "createpsbt",
                "createrawtransaction",
                "createwallet",
                "decodepsbt",
                "decoderawtransaction",
                "decodescript",
                "deriveaddresses",
                "disconnectnode",
                "dumpprivkey",
                "dumpwallet",
                "echo",
                "encryptwallet",
                "estimatesmartfee",
                "finalizepsbt",
                "fundrawtransaction",
                "generate",
                "generateblock",
                "generatetoaddress",
                "generatetodescriptor",
                "getaddednodeinfo",
                "getaddressesbylabel",
                "getaddressinfo",
                "getbalance",
                "getbalances",
                "getbestblockhash",
                "getblock",
                "getblockchaininfo",
                "getblockcount",
                "getblockfilter",
                "getblockhash",
                "getblockheader",
                "getblockstats",
                "getblocktemplate",
                "getchaintips",
                "getchaintxstats",
                "getconnectioncount",
                "getdescriptorinfo",
                "getdifficulty",
                "getindexinfo",
                "getinfo",
                "getmemoryinfo",
                "getmempoolancestors",
                "getmempooldescendants",
                "getmempoolentry",
                "getmempoolinfo",
                "getmininginfo",
                "getnettotals",
                "getnetworkhashps",
                "getnetworkinfo",
                "getnewaddress",
                "getnodeaddresses",
                "getpeerinfo",
                "getrawchangeaddress",
                "getrawmempool",
                "getrawtransaction",
                "getreceivedbyaddress",
                "getreceivedbylabel",
                "getrpcinfo",
                "gettransaction",
                "gettxout",
                "gettxoutproof",
                "gettxoutsetinfo",
                "getunconfirmedbalance",
                "getwalletinfo",
                "getzmqnotifications",
                "help",
                "importaddress",
                "importdescriptors",
                "importmulti",
                "importprivkey",
                "importprunedfunds",
                "importpubkey",
                "importwallet",
                "joinpsbts",
                "keypoolrefill",
                "listaddressgroupings",
                "listbanned",
                "listlabels",
                "listlockunspent",
                "listreceivedbyaddress",
                "listreceivedbylabel",
                "listsinceblock",
                "listtransactions",
                "listunspent",
                "listwalletdir",
                "listwallets",
                "loadwallet",
                "lockunspent",
                "logging",
                "ping",
                "preciousblock",
                "prioritisetransaction",
                "pruneblockchain",
                "psbtbumpfee",
                "removeprunedfunds",
                "rescanblockchain",
                "savemempool",
                "scantxoutset",
                "send",
                "sendmany",
                "sendrawtransaction",
                "sendtoaddress",
                "setban",
                "sethdseed",
                "setlabel",
                "setnetworkactive",
                "settxfee",
                "setwalletflag",
                "signmessage",
                "signrawtransactionwithkey",
                "signrawtransactionwithwallet",
                "signmessagewithprivkey",
                "stop",
                "submitblock",
                "submitheader",
                "testmempoolaccept",
                "unloadwallet",
                "upgradewallet",
                "uptime",
                "utxoupdatepsbt",
                "validateaddress",
                "verifychain",
                "verifymessage",
                "verifytxoutproof",
                "walletcreatefundedpsbt",
                "walletlock",
                "walletpassphrase",
                "walletpassphrasechange",
                "walletprocesspsbt"
              ]
            },
            "default": [
              "abandontransaction",
              "abortrescan",
              "addmultisigaddress",
              "addnode",
              "analyzepsbt",
              "backupwallet",
              "bumpfee",
              "clearbanned",
              "combinepsbt",
              "combinerawtransaction",
              "converttopsbt",
              "createmultisig",
              "createpsbt",
              "createrawtransaction",
              "createwallet",
              "decodepsbt",
              "decoderawtransaction",
              "decodescript",
              "deriveaddresses",
              "disconnectnode",
              "dumpwallet",
              "echo",
              "encryptwallet",
              "estimatesmartfee",
              "finalizepsbt",
              "fundrawtransaction",
              "generate",
              "generateblock",
              "generatetoaddress",
              "generatetodescriptor",
              "getaddednodeinfo",
              "getaddressesbylabel",
              "getaddressinfo",
              "getbalance",
              "getbalances",
              "getbestblockhash",
              "getblock",
              "getblockchaininfo",
              "getblockcount",
              "getblockfilter",
              "getblockhash",
              "getblockheader",
              "getblockstats",
              "getblocktemplate",
              "getchaintips",
              "getchaintxstats",
              "getconnectioncount",
              "getdescriptorinfo",
              "getdifficulty",
              "getindexinfo",
              "getinfo",
              "getmemoryinfo",
              "getmempoolancestors",
              "getmempooldescendants",
              "getmempoolentry",
              "getmempoolinfo",
              "getmininginfo",
              "getnettotals",
              "getnetworkhashps",
              "getnetworkinfo",
              "getnewaddress",
              "getnodeaddresses",
              "getpeerinfo",
              "getrawchangeaddress",
              "getrawmempool",
              "getrawtransaction",
              "getreceivedbyaddress",
              "getreceivedbylabel",
              "getrpcinfo",
              "gettransaction",
              "gettxout",
              "gettxoutproof",
              "gettxoutsetinfo",
              "getunconfirmedbalance",
              "getwalletinfo",
              "getzmqnotifications",
              "help",
              "importaddress",
              "importdescriptors",
              "importmulti",
              "importprivkey",
              "importprunedfunds",
              "importpubkey",
              "importwallet",
              "joinpsbts",
              "keypoolrefill",
              "listaddressgroupings",
              "listbanned",
              "listlabels",
              "listlockunspent",
              "listreceivedbyaddress",
              "listreceivedbylabel",
              "listsinceblock",
              "listtransactions",
              "listunspent",
              "listwalletdir",
              "listwallets",
              "loadwallet",
              "lockunspent",
              "logging",
              "ping",
              "preciousblock",
              "prioritisetransaction",
              "pruneblockchain",
              "psbtbumpfee",
              "removeprunedfunds",
              "rescanblockchain",
              "savemempool",
              "scantxoutset",
              "send",
              "sendmany",
              "sendrawtransaction",
              "sendtoaddress",
              "setban",
              "sethdseed",
              "setlabel",
              "setnetworkactive",
              "settxfee",
              "setwalletflag",
              "signmessage",
              "signrawtransactionwithkey",
              "signrawtransactionwithwallet",
              "signmessagewithprivkey",
              "submitblock",
              "submitheader",
              "testmempoolaccept",
              "unloadwallet",
              "upgradewallet",
              "uptime",
              "utxoupdatepsbt",
              "validateaddress",
              "verifychain",
              "verifymessage",
              "verifytxoutproof",
              "walletcreatefundedpsbt",
              "walletlock",
              "walletpassphrase",
              "walletpassphrasechange",
              "walletprocesspsbt"
            ]
          },
          "fetch-blocks": {
            "type": "boolean",
            "name": "Fetch Blocks",
            "description": "Fetch blocks from the network if pruned from disk",
            "default": true
          }
        }
      },
      "default": [

      ]
    },
    "advanced": {
      "type": "object",
      "name": "Advanced",
      "description": "Advanced settings for Bitcoin Proxy",
      "spec": {
        "tor-only": {
          "type": "boolean",
          "name": "Only Tor Peers",
          "description": "Use Tor for all peer connections",
          "default": false
        },
        "peer-timeout": {
          "type": "number",
          "name": "Peer Message Timeout",
          "description": "How long to wait for a response from a peer before failing",
          "nullable": false,
          "integral": true,
          "units": "Seconds",
          "range": "[0, *)",
          "default": 30
        },
        "max-peer-age": {
          "type": "number",
          "name": "Maximum Peer Age",
          "description": "How long to wait before refreshing the peer list",
          "nullable": false,
          "integral": true,
          "units": "Seconds",
          "range": "[0, *)",
          "default": 300
        },
        "max-peer-concurrency": {
          "type": "number",
          "name": "Maximum Peer Concurrency",
          "description": "How many peers to reach out to concurrently for block data",
          "nullable": true,
          "integral": true,
          "range": "[1, *)",
          "default": 1
        },
        "log-level": {
          "type": "enum",
          "name": "Log Level",
          "values": [
            "CRITICAL",
            "ERROR",
            "WARN",
            "INFO",
            "DEBUG",
            "TRACE"
          ],
          "value-names": {
            "CRITICAL": "Critical",
            "ERROR": "Error",
            "WARN": "Warning",
            "INFO": "Info",
            "DEBUG": "Debug",
            "TRACE": "Trace"
          },
          "default": "DEBUG"
        }
      }
    }
  };
  return {
    result: {
      config,
      spec,
    },
  };
};
