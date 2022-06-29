use std::convert::TryInto;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashSet, env::var};

use anyhow::Error;
use btc_rpc_proxy::{AuthSource, Peers, RpcClient, State, TorState, User, Users};
use linear_map::LinearMap;
use slog::{Drain, Level};
use tokio::sync::RwLock;

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    pub tor_address: String,
    pub bitcoind: BitcoinCoreConfig,
    pub users: Vec<UserInfo>,
    pub advanced: AdvancedConfig,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct UserInfo {
    pub name: String,
    pub password: String,
    pub allowed_calls: HashSet<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AdvancedConfig {
    pub tor_only: bool,
    pub peer_timeout: u64,
    pub max_peer_age: u64,
    pub max_peer_concurrency: Option<usize>,
    pub log_level: String,
}

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
struct BitcoinCoreConfig {
    user: String,
    password: String,
}

#[derive(serde::Serialize)]
pub struct Properties {
    version: u8,
    data: Data,
}

#[derive(serde::Serialize)]
pub struct Data {
    #[serde(rename = "Quick Connect URLs")]
    quick_connect_urls: Property,
    #[serde(rename = "RPC Users")]
    rpc_users: Property,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum Property {
    String {
        value: String,
        description: Option<String>,
        copyable: bool,
        qr: bool,
        masked: bool,
    },
    Object {
        value: LinearMap<String, Property>,
        description: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cfg: Config = tokio::task::spawn_blocking(move || -> Result<_, Error> {
        let cfg: Config =
            serde_yaml::from_reader(std::fs::File::open("/root/start9/config.yaml")?)?;
        let tor_addr = &cfg.tor_address;
        serde_yaml::to_writer(
            std::fs::File::create("/root/start9/stats.yaml")?,
            &Properties {
                version: 2,
                data: Data {
                    quick_connect_urls: Property::Object {
                        value: cfg
                            .users
                            .iter()
                            .map(|user| {
                                (
                                    user.name.clone(),
                                    Property::String {
                                        value: format!(
                                            "btcstandup://{}:{}@{}:8332/",
                                            user.name, user.password, tor_addr
                                        ),
                                        description: Some(format!(
                                            "Quick Connect URL for {}",
                                            user.name
                                        )),
                                        copyable: true,
                                        qr: true,
                                        masked: true,
                                    },
                                )
                            })
                            .collect(),
                        description: Some("Quick Connect URLs for each user".to_owned()),
                    },
                    rpc_users: Property::Object {
                        value: cfg
                            .users
                            .iter()
                            .map(|user| {
                                (
                                    user.name.to_owned(),
                                    Property::Object {
                                        value: std::iter::empty()
                                            .chain(std::iter::once((
                                                "Username".to_owned(),
                                                Property::String {
                                                    value: format!("{}", user.name),
                                                    description: None,
                                                    copyable: true,
                                                    qr: false,
                                                    masked: false,
                                                },
                                            )))
                                            .chain(std::iter::once((
                                                "Password".to_owned(),
                                                Property::String {
                                                    value: format!("{}", user.password),
                                                    description: None,
                                                    copyable: true,
                                                    qr: false,
                                                    masked: true,
                                                },
                                            )))
                                            .collect(),
                                        description: Some(format!(
                                            "RPC Credentials for {}",
                                            user.name
                                        )),
                                    },
                                )
                            })
                            .collect(),
                        description: Some("RPC Credentials for each user".to_owned()),
                    },
                },
            },
        )?;
        Ok(cfg)
    })
    .await??;
    let log_level = match cfg.advanced.log_level.as_str() {
        "CRITICAL" => Level::Critical,
        "ERROR" => Level::Error,
        "WARN" => Level::Warning,
        "INFO" => Level::Info,
        "DEBUG" => Level::Debug,
        "TRACE" => Level::Trace,
        _ => Level::Debug,
    };
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain)
        .build()
        .filter_level(log_level)
        .fuse();
    let logger = slog::Logger::root(drain, slog::o!());
    btc_rpc_proxy::main(
        State {
            rpc_client: RpcClient::new(
                AuthSource::from_config(
                    Some(cfg.bitcoind.user),
                    Some(cfg.bitcoind.password),
                    None,
                )?,
                format!("http://bitcoind.embassy:8332").parse()?,
                &logger,
            ),
            tor: Some(TorState {
                proxy: format!("{}:9050", var("HOST_IP")?).parse()?,
                only: cfg.advanced.tor_only,
            }),
            users: Users(
                cfg.users
                    .into_iter()
                    .map(|user| {
                        (
                            user.name,
                            User {
                                password: user.password.try_into().unwrap(),
                                allowed_calls: user.allowed_calls,
                                fetch_blocks: true,
                            },
                        )
                    })
                    .collect(),
            ),
            logger,
            peer_timeout: Duration::from_secs(cfg.advanced.peer_timeout),
            peers: RwLock::new(Arc::new(Peers::new())),
            max_peer_age: Duration::from_secs(cfg.advanced.max_peer_age),
            max_peer_concurrency: cfg.advanced.max_peer_concurrency,
        }
        .arc(),
        ([0, 0, 0, 0], 8332).into(),
    )
    .await
}
