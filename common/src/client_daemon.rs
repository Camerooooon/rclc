use chrono::{offset::Utc, DateTime};
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};

use std::{
    fmt::{Display, Formatter},
    net::IpAddr,
    num::ParseIntError,
    str::ParseBoolError,
};

#[derive(Debug)]
pub struct PacketParseError;

impl Display for PacketParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Packet parse error occured")
    }
}

impl From<ParseIntError> for PacketParseError {
    fn from(_err: ParseIntError) -> Self {
        PacketParseError
    }
}

impl From<ParseBoolError> for PacketParseError {
    fn from(_err: ParseBoolError) -> Self {
        PacketParseError
    }
}

/// a literal message sent from one peer
#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub content: String,
    pub time: DateTime<Utc>,
    pub origin: IpAddr,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ClientToDaemonMsg {
    Block(IpAddr),
    Send(String),
    Connect,
    Disconnect,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum DaemonToClientMsg {
    Recieved(Message),
    Unknown,
}

pub fn parse_client_to_daemon_message(packet: &str) -> Result<ClientToDaemonMsg, PacketParseError> {
    serde_json::from_str(packet).map_err(|_| PacketParseError)
}
