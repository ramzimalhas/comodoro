//! Email server CLI module.
//!
//! This module contains the command matcher, the subcommands and the
//! arguments related to the email server domain.

use anyhow::Result;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use log::debug;

use crate::Protocol;

const ARG_PROTOCOLS: &str = "protocols";
const CMD_START: &str = "start";

pub(crate) const CMD_SERVER: &str = "server";

/// Represents the server commands.
#[derive(Debug, PartialEq, Eq)]
pub enum Cmd {
    Start(Vec<Protocol>),
}

/// Represents the server command matcher.
pub fn matches<'a>(m: &'a ArgMatches) -> Result<Option<Cmd>> {
    let cmd = if let Some(m) = m.subcommand_matches(CMD_SERVER) {
        if let Some(m) = m.subcommand_matches(CMD_START) {
            let protocols = parse_protocols(m);
            debug!("start server command matched");
            Some(Cmd::Start(protocols))
        } else {
            None
        }
    } else {
        None
    };

    Ok(cmd)
}

/// Represents the server protocols argument.
pub fn protocols() -> Arg {
    Arg::new(ARG_PROTOCOLS)
        .help("Define protocols the server should use to accept requests")
        .num_args(1..)
        .action(ArgAction::Append)
        .value_parser(value_parser!(Protocol))
}

/// Represents the server protocols argument parser.
pub fn parse_protocols(m: &ArgMatches) -> Vec<Protocol> {
    m.get_many::<Protocol>(ARG_PROTOCOLS)
        .unwrap_or_default()
        .map(ToOwned::to_owned)
        .collect()
}

/// Represents the server subcommands.
pub fn subcmd() -> Command {
    Command::new(CMD_SERVER)
        .about("Server commands")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(CMD_START)
                .about("Start the timer server")
                .arg(protocols()),
        )
}
