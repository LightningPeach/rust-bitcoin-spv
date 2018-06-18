//
// Copyright 2018 Tamas Blummer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//! # Bitcoin DNS lookup
//!
//! Find initial Bitcoin node addresses by looking up trusted DNS servers
//! This should only be used if the peer has no own knowledge where to find a node of the
//! Bitcoin network
//!
//!

use std::net::{SocketAddr, ToSocketAddrs};

const SEEDER : [&str;5] = [
    "seed.bitcoin.sipa.be",
    "dnsseed.bluematt.me",
    "dnsseed.bitcoin.dashjr.org",
    "seed.bitcoinstats.com",
    "seed.btc.petertodd.org"
];

pub fn dns_seed () -> Vec<SocketAddr> {
    info!("reaching out for DNS seed...");
    let mut seeds = Vec::new ();
    for seedhost in SEEDER.iter() {
        if let Ok(lookup) = (*seedhost, 8333).to_socket_addrs() {
            for host in lookup {
                seeds.push(host);
            }
        }
        else {
            trace!("{} did not answer", seedhost);
        }
    }
    info!("received {} DNS seeds", seeds.len());
    seeds
}