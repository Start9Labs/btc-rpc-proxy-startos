use std::convert::TryInto;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashSet, env::var};

use anyhow::Error;
use btc_rpc_proxy::{
    util::deserialize_parse, AuthSource, Peers, RpcClient, State, TorState, User, Users,
};
use http::uri;
use hyper::Uri;
use linear_map::LinearMap;
use slog::Drain;
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
    pub fetch_blocks: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AdvancedConfig {
    pub tor_only: bool,
    pub peer_timeout: u64,
    pub max_peer_age: u64,
    pub max_peer_concurrency: Option<usize>,
}

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
enum BitcoinCoreConfig {
    #[serde(rename_all = "kebab-case")]
    Internal { user: String, password: String },
    #[serde(rename_all = "kebab-case")]
    External {
        connection_settings: ExternalBitcoinCoreConfig,
    },
}

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
enum ExternalBitcoinCoreConfig {
    #[serde(rename_all = "kebab-case")]
    Manual {
        #[serde(deserialize_with = "deserialize_parse")]
        addressext: Uri,
        userext: String,
        passwordext: String,
    },
    #[serde(rename_all = "kebab-case")]
    QuickConnect {
        #[serde(deserialize_with = "deserialize_parse")]
        quick_connect_url: Uri,
    },
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
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, slog::o!());
    btc_rpc_proxy::main(
        State {
            rpc_client: match cfg.bitcoind {
                BitcoinCoreConfig::Internal { user, password } => RpcClient::new(
                    AuthSource::from_config(Some(user), Some(password), None)?,
                    format!("http://bitcoind.embassy:8332").parse()?,
                    &logger,
                ),
                BitcoinCoreConfig::External {
                    connection_settings:
                        ExternalBitcoinCoreConfig::Manual {
                            addressext,
                            userext,
                            passwordext,
                        },
                } => RpcClient::new(
                    AuthSource::from_config(Some(userext), Some(passwordext), None)?,
                    Uri::from_parts({
                        let mut addr = addressext.into_parts();
                        addr.scheme = Some(uri::Scheme::HTTP);
                        addr.path_and_query = None;
                        if let Some(ref auth) = addr.authority {
                            if auth.port().is_none() {
                                addr.authority = Some(format!("{}:8332", auth).parse()?);
                            }
                        }
                        addr
                    })?,
                    &logger,
                ),
                BitcoinCoreConfig::External {
                    connection_settings:
                        ExternalBitcoinCoreConfig::QuickConnect { quick_connect_url },
                } => {
                    let auth = quick_connect_url
                        .authority()
                        .ok_or_else(|| anyhow::anyhow!("invalid Quick Connect URL"))?;
                    let mut auth_split = auth.as_str().split(|c| c == ':' || c == '@');
                    let user = auth_split.next().map(|s| s.to_owned());
                    let password = auth_split.next().map(|s| s.to_owned());
                    RpcClient::new(
                        AuthSource::from_config(user, password, None)?,
                        format!(
                            "http://{}:{}/",
                            auth.host(),
                            auth.port_u16().unwrap_or(8332)
                        )
                        .parse()?,
                        &logger,
                    )
                }
            },
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
                                fetch_blocks: user.fetch_blocks,
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
