// due to code generated by JsonSchema
#![allow(clippy::field_reassign_with_default)]

use cosmwasm_std::{Addr, Coin};
use crate::{IdentityKey, SphinxKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct Gateway {
    pub mix_host: String,
    pub clients_host: String,
    pub location: String,
    pub sphinx_key: SphinxKey,
    /// Base58 encoded ed25519 EdDSA public key of the gateway used to derive shared keys with clients
    pub identity_key: IdentityKey,
    pub version: String,
}

impl Gateway {
    pub fn new(
        mix_host: String,
        clients_host: String,
        location: String,
        sphinx_key: SphinxKey,
        identity_key: IdentityKey,
        version: String,
    ) -> Self {
        Gateway {
            mix_host,
            clients_host,
            location,
            sphinx_key,
            identity_key,
            version,
        }
    }

    pub fn try_resolve_hostname(&self) -> Result<SocketAddr, io::Error> {
        self.mix_host
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no valid socket address"))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct GatewayBond {
    pub amount: Vec<Coin>,
    pub owner: Addr,
    pub gateway: Gateway,
}

impl GatewayBond {
    pub fn new(amount: Vec<Coin>, owner: Addr, gateway: Gateway) -> Self {
        GatewayBond {
            amount,
            owner,
            gateway,
        }
    }

    pub fn identity(&self) -> &String {
        &self.gateway.identity_key
    }

    pub fn amount(&self) -> &[Coin] {
        &self.amount
    }

    pub fn owner(&self) -> &Addr {
        &self.owner
    }

    pub fn gateway(&self) -> &Gateway {
        &self.gateway
    }
}

impl Display for GatewayBond {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.amount.len() != 1 {
            write!(f, "amount: {:?}, owner: {}", self.amount, self.owner)
        } else {
            write!(
                f,
                "amount: {} {}, owner: {}",
                self.amount[0].amount, self.amount[0].denom, self.owner
            )
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct PagedGatewayResponse {
    pub nodes: Vec<GatewayBond>,
    pub per_page: usize,
    pub start_next_after: Option<IdentityKey>,
}

impl PagedGatewayResponse {
    pub fn new(nodes: Vec<GatewayBond>, per_page: usize, start_next_after: Option<IdentityKey>) -> Self {
        PagedGatewayResponse {
            nodes,
            per_page,
            start_next_after,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct GatewayOwnershipResponse {
    pub address: Addr,
    pub has_gateway: bool,
}
