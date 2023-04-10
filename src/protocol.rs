use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, ValueEnum};
use pimalaya::time::pomodoro::{Client, ServerBind, TcpBind, TcpClient};
use serde::{Deserialize, Serialize};

use crate::Config;

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    #[cfg(feature = "tcp-client")]
    Tcp,
    #[default]
    None,
}

impl Protocol {
    pub fn to_binders(config: &Config, protocols: Vec<Self>) -> Vec<Box<dyn ServerBind>> {
        let protocols = if protocols.is_empty() {
            vec![
                #[cfg(feature = "tcp-binder")]
                Self::Tcp,
            ]
        } else {
            protocols
        };

        protocols
            .iter()
            .filter_map(|protocol| match protocol {
                #[cfg(feature = "tcp-binder")]
                Protocol::Tcp => {
                    if let Some(ref config) = config.tcp {
                        Some(TcpBind::new(&config.host, config.port))
                    } else {
                        None
                    }
                }
                Protocol::None => None,
            })
            .collect()
    }

    pub fn to_client(&self, config: &Config) -> Result<Box<dyn Client>> {
        match self {
            #[cfg(feature = "tcp-client")]
            Self::Tcp => {
                if let Some(ref config) = config.tcp {
                    Ok(TcpClient::new(&config.host, config.port))
                } else {
                    Err(anyhow!("cannot build tcp client: missing tcp config"))
                }
            }
            Self::None => Err(anyhow!("cannot build client: missing protocol")),
        }
    }
}

impl ValueEnum for Protocol {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        match input {
            #[cfg(any(feature = "tcp-binder", feature = "tcp-client"))]
            p if "tcp" == p || ignore_case && p.eq_ignore_ascii_case("tcp") => Ok(Self::Tcp),
            p => Err(format!("invalid protocol {p}")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            #[cfg(any(feature = "tcp-binder", feature = "tcp-client"))]
            Self::Tcp,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            #[cfg(any(feature = "tcp-binder", feature = "tcp-client"))]
            Self::Tcp => Some(PossibleValue::new("tcp")),
            Self::None => None,
        }
    }
}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            #[cfg(any(feature = "tcp-binder", feature = "tcp-client"))]
            Self::Tcp => "tcp".into(),
            Self::None => "none".into(),
        }
    }
}
