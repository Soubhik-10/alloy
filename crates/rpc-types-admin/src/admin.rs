//! Types for the `admin` API.

use alloy_genesis::ChainConfig;
use alloy_primitives::{B256, U256};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    net::{IpAddr, SocketAddr},
};

/// This includes general information about a running node, spanning networking and protocol
/// details.
///
/// See [geth's `NodeInfo` struct](https://github.com/ethereum/go-ethereum/blob/v1.14.0/p2p/server.go#L1078)
/// for the source of each field.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Unique node identifier (also the encryption key).
    pub id: String,
    /// The node's user agent, containing a client name, version, OS, and other metadata.
    pub name: String,
    /// The enode URL of the connected node.
    pub enode: String,
    /// The [ENR](https://eips.ethereum.org/EIPS/eip-778) of the running client.
    pub enr: String,
    /// The IP address of the connected node.
    pub ip: IpAddr,
    /// The node's listening ports.
    pub ports: Ports,
    /// The node's listening address.
    #[serde(rename = "listenAddr")]
    pub listen_addr: SocketAddr,
    /// The protocols that the node supports, with protocol metadata.
    pub protocols: ProtocolInfo,
}

/// Represents a node's discovery and listener ports.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ports {
    /// The node's discovery port.
    pub discovery: u16,
    /// The node's listener port.
    pub listener: u16,
}

/// Represents protocols that the connected RPC node supports.
///
/// This contains protocol information reported by the connected RPC node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolInfo {
    /// Details about the node's supported eth protocol. `None` if unsupported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth: Option<EthProtocolInfo>,
    /// Details about the node's supported snap protocol. `None` if unsupported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snap: Option<SnapProtocolInfo>,
}

/// Represents a short summary of the `eth` sub-protocol metadata known about the host peer.
///
/// See [geth's `NodeInfo`
/// struct](https://github.com/ethereum/go-ethereum/blob/c2e0abce2eedc1ba2a1b32c46fd07ef18a25354a/eth/protocols/eth/handler.go#L129)
/// for how these fields are determined.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthProtocolInfo {
    /// The eth network version.
    pub network: u64,
    /// The total difficulty of the host's blockchain.
    ///
    /// NOTE: This is deprecated as total difficulty related fields are being removed from RPC,
    /// since the merge has long passed.
    ///
    /// See changes to geth's `NodeInfo` structs:
    /// * <https://github.com/ethereum/go-ethereum/pull/30744>
    /// * <https://github.com/ethereum/go-ethereum/blob/314e18193eeca3e47b627408da47e33132d72aa8/eth/protocols/eth/handler.go#L119-L126>
    #[deprecated(
        since = "0.8.2",
        note = "`difficulty` is being removed from `admin_nodeInfo`, see https://github.com/ethereum/go-ethereum/pull/30744"
    )]
    pub difficulty: Option<U256>,
    /// The Keccak hash of the host's genesis block.
    pub genesis: B256,
    /// The chain configuration for the host's fork rules.
    pub config: ChainConfig,
    /// The hash of the host's best known block.
    pub head: B256,
}

/// Represents a short summary of the host's `snap` sub-protocol metadata.
///
/// This is just an empty struct, because [geth's internal representation is
/// empty](https://github.com/ethereum/go-ethereum/blob/c2e0abce2eedc1ba2a1b32c46fd07ef18a25354a/eth/protocols/snap/handler.go#L571-L576).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapProtocolInfo {}

/// Represents the protocols that a peer supports.
///
/// This differs from [`ProtocolInfo`] in that [`PeerProtocolInfo`] contains protocol information
/// gathered from the protocol handshake, and [`ProtocolInfo`] contains information reported by the
/// connected RPC node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerProtocolInfo {
    /// Details about the peer's supported eth protocol. `None` if unsupported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth: Option<EthPeerInfo>,
    /// Details about the peer's supported snap protocol. `None` if unsupported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snap: Option<SnapPeerInfo>,
    /// Placeholder for any other protocols
    #[serde(flatten, default)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Can contain either eth protocol info or a string "handshake", which geth uses if the peer is
/// still completing the handshake for the protocol.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EthPeerInfo {
    /// The `eth` sub-protocol metadata known about the host peer.
    Info(EthInfo),
    /// The string "handshake" if the peer is still completing the handshake for the protocol.
    #[serde(with = "handshake")]
    Handshake,
}

/// Represents a short summary of the `eth` sub-protocol metadata known about a connected peer
///
/// See [geth's `ethPeerInfo`
/// struct](https://github.com/ethereum/go-ethereum/blob/94579932b18931115f28aa7f87f02450bda084c9/eth/peer.go#L26)
/// for how these fields are determined.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct EthInfo {
    /// The negotiated eth version.
    pub version: u64,
}

/// Can contain either snap protocol info or a string "handshake", which geth uses if the peer is
/// still completing the handshake for the protocol.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SnapPeerInfo {
    /// The `snap` sub-protocol metadata known about the host peer.
    Info(SnapInfo),
    /// The string "handshake" if the peer is still completing the handshake for the protocol.
    #[serde(with = "handshake")]
    Handshake,
}

/// Represents a short summary of the `snap` sub-protocol metadata known about a connected peer.
///
/// See [geth's `snapPeerInfo`
/// struct](https://github.com/ethereum/go-ethereum/blob/94579932b18931115f28aa7f87f02450bda084c9/eth/peer.go#L45)
/// for how these fields are determined.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapInfo {
    /// The negotiated snap version.
    pub version: u64,
}

/// Represents a short summary of information known about a connected peer.
///
/// See [geth's `PeerInfo` struct](https://github.com/ethereum/go-ethereum/blob/94579932b18931115f28aa7f87f02450bda084c9/p2p/peer.go#L495) for the source of each field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerInfo {
    /// The peer's ENR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enr: Option<String>,
    /// The peer's enode URL.
    pub enode: String,
    /// The peer's enode ID.
    pub id: String,
    /// The peer's name.
    pub name: String,
    /// The peer's capabilities.
    pub caps: Vec<String>,
    /// Networking information about the peer.
    pub network: PeerNetworkInfo,
    /// The protocols that the peer supports, with protocol metadata.
    pub protocols: PeerProtocolInfo,
}

/// Represents networking related information about the peer, including details about whether or
/// not it is inbound, trusted, or static.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerNetworkInfo {
    /// The local endpoint of the TCP connection.
    pub local_address: SocketAddr,
    /// The remote endpoint of the TCP connection.
    pub remote_address: SocketAddr,
    /// Whether or not the peer is inbound.
    pub inbound: bool,
    /// Whether or not the peer is trusted.
    pub trusted: bool,
    /// Whether or not the peer is a static peer.
    #[serde(rename = "static")]
    pub static_node: bool,
}

/// The type of a peer event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PeerEventType {
    /// A peer was added to the server.
    Add,
    /// A peer was dropped from the server.
    Drop,
    /// A message was successfully sent to the peer.
    MsgSend,
    /// A message was successfully received by the peer.
    MsgRecv,
}

/// An event emitted when peers are either added or dropped from a p2p server or when a message is
/// sent or received on a peer connection.
///
/// See [geth's `PeerEvent` struct](https://github.com/ethereum/go-ethereum/blob/94579932b18931115f28aa7f87f02450bda084c9/p2p/peer.go#L94-L103) for the source of each field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerEvent {
    /// The type of the event.
    #[serde(rename = "type")]
    pub kind: PeerEventType,
    /// The peer's enode ID.
    pub peer: String,
    /// An error occurred on the peer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The protocol of the peer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// The message code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_code: Option<u64>,
    /// The message size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_size: Option<u32>,
    /// The local endpoint of the TCP connection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address: Option<SocketAddr>,
    /// The remote endpoint of the TCP connection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<SocketAddr>,
}

mod handshake {
    use super::*;

    pub(crate) fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<(), D::Error> {
        let s = String::deserialize(deserializer)?;
        if s == "handshake" {
            Ok(())
        } else {
            Err(serde::de::Error::custom(
                "expected \"handshake\" if protocol info did not appear in the response",
            ))
        }
    }

    pub(crate) fn serialize<S: serde::Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str("handshake")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    #[test]
    fn deserialize_peer_info() {
        let response = r#"{
            "enode":"enode://bb37b7302f79e47c1226d6e3ccf0ef6d51146019efdcc1f6e861fd1c1a78d5e84e486225a6a8a503b93d5c50125ee980835c92bde7f7d12f074c16f4e439a578@127.0.0.1:60872",
            "id":"ca23c04b7e796da5d6a5f04a62b81c88d41b1341537db85a2b6443e838d8339b",
            "name":"Geth/v1.10.19-stable/darwin-arm64/go1.18.3",
            "caps":["eth/66","eth/67","snap/1"],
            "network":{
                "localAddress":"127.0.0.1:30304",
                "remoteAddress":"127.0.0.1:60872",
                "inbound":true,
                "trusted":false,
                "static":false
            },
            "protocols":{
                "eth":{
                    "version":67,
                    "difficulty":0,
                    "head":"0xb04009ddf4b0763f42778e7d5937e49bebf1e11b2d26c9dac6cefb5f84b6f8ea"
                },
                "snap":{"version":1}
            }
        }"#;
        let peer_info: PeerInfo = serde_json::from_str(response).unwrap();

        assert_eq!(peer_info.enode, "enode://bb37b7302f79e47c1226d6e3ccf0ef6d51146019efdcc1f6e861fd1c1a78d5e84e486225a6a8a503b93d5c50125ee980835c92bde7f7d12f074c16f4e439a578@127.0.0.1:60872");
    }

    #[test]
    fn deserialize_peer_info_handshake() {
        let response = r#"{
            "enode": "enode://a997fde0023537ad01e536ebf2eeeb4b4b3d5286707586727b704f32e8e2b4959e08b6db5b27eb6b7e9f6efcbb53657f4e2bd16900aa77a89426dc3382c29ce0@[::1]:60948",
            "id": "df6f8bc331005962c2ef1f5236486a753bc6b2ddb5ef04370757999d1ca832d4",
            "name": "Geth/v1.10.26-stable-e5eb32ac/linux-amd64/go1.18.5",
            "caps": ["eth/66","eth/67","snap/1"],
            "network":{
                "localAddress":"[::1]:30304",
                "remoteAddress":"[::1]:60948",
                "inbound":true,
                "trusted":false,
                "static":false
            },
            "protocols":{
                "eth":"handshake",
                "snap":"handshake"
            }
        }"#;

        let info: PeerInfo = serde_json::from_str(response).unwrap();
        assert_eq!(info.protocols.eth, Some(EthPeerInfo::Handshake));
        assert_eq!(info.protocols.snap, Some(SnapPeerInfo::Handshake));
    }

    #[test]
    fn deserialize_peer_info_newer() {
        let response = r#"{
            "enode": "enode://f769f8cf850dd9f88a13c81ff3e70c3400cf93511c676c6d50f0e359beb43c28388931f64f56ab4110ccced37fb08163b6966fe42b6e15ec647fa8087914463d@127.0.0.1:45591?discport=0",
            "id": "daa738efebf7e349b9f5b1a91d782e7355060bb15af8570e23463729d0632deb",
            "name": "Geth/v1.13.14-stable-2bd6bd01/linux-amd64/go1.21.6",
            "caps": ["eth/68", "snap/1"],
            "network": {
                "localAddress": "127.0.0.1:33236",
                "remoteAddress": "127.0.0.1:45591",
                "inbound": false,
                "trusted": false,
                "static": true
            },
            "protocols": { "eth": { "version": 68 }, "snap": { "version": 1 } }
        }"#;

        let info: PeerInfo = serde_json::from_str(response).unwrap();
        assert_eq!(info.protocols.eth, Some(EthPeerInfo::Info(EthInfo { version: 68 })));
        assert_eq!(info.protocols.snap, Some(SnapPeerInfo::Info(SnapInfo { version: 1 })));
    }

    #[test]
    fn deserialize_node_info() {
        // this response also has an enr
        let response = r#"{
            "id":"6e2fe698f3064cd99410926ce16734e35e3cc947d4354461d2594f2d2dd9f7b6",
            "name":"Geth/v1.10.19-stable/darwin-arm64/go1.18.3",
            "enode":"enode://d7dfaea49c7ef37701e668652bcf1bc63d3abb2ae97593374a949e175e4ff128730a2f35199f3462a56298b981dfc395a5abebd2d6f0284ffe5bdc3d8e258b86@127.0.0.1:30304?discport=0",
            "enr":"enr:-Jy4QIvS0dKBLjTTV_RojS8hjriwWsJNHRVyOh4Pk4aUXc5SZjKRVIOeYc7BqzEmbCjLdIY4Ln7x5ZPf-2SsBAc2_zqGAYSwY1zog2V0aMfGhNegsXuAgmlkgnY0gmlwhBiT_DiJc2VjcDI1NmsxoQLX366knH7zdwHmaGUrzxvGPTq7Kul1kzdKlJ4XXk_xKIRzbmFwwIN0Y3CCdmA",
            "ip":"127.0.0.1",
            "ports":{
                "discovery":0,
                "listener":30304
            },
            "listenAddr":"[::]:30304",
            "protocols":{
                "eth":{
                    "network":1337,
                    "difficulty":0,
                    "genesis":"0xb04009ddf4b0763f42778e7d5937e49bebf1e11b2d26c9dac6cefb5f84b6f8ea",
                    "config":{
                        "chainId":0,
                        "eip150Hash":"0x0000000000000000000000000000000000000000000000000000000000000000"
                    },
                    "head":"0xb04009ddf4b0763f42778e7d5937e49bebf1e11b2d26c9dac6cefb5f84b6f8ea"
                },
                "snap":{}
            }
        }"#;

        let _: NodeInfo = serde_json::from_str(response).unwrap();
    }

    #[test]
    fn deserialize_node_info_post_merge() {
        // this response also has an enr
        let response = r#"{
            "id":"6e2fe698f3064cd99410926ce16734e35e3cc947d4354461d2594f2d2dd9f7b6",
            "name":"Geth/v1.10.19-stable/darwin-arm64/go1.18.3",
            "enode":"enode://d7dfaea49c7ef37701e668652bcf1bc63d3abb2ae97593374a949e175e4ff128730a2f35199f3462a56298b981dfc395a5abebd2d6f0284ffe5bdc3d8e258b86@127.0.0.1:30304?discport=0",
            "enr":"enr:-Jy4QIvS0dKBLjTTV_RojS8hjriwWsJNHRVyOh4Pk4aUXc5SZjKRVIOeYc7BqzEmbCjLdIY4Ln7x5ZPf-2SsBAc2_zqGAYSwY1zog2V0aMfGhNegsXuAgmlkgnY0gmlwhBiT_DiJc2VjcDI1NmsxoQLX366knH7zdwHmaGUrzxvGPTq7Kul1kzdKlJ4XXk_xKIRzbmFwwIN0Y3CCdmA",
            "ip":"127.0.0.1",
            "ports":{
                "discovery":0,
                "listener":30304
            },
            "listenAddr":"[::]:30304",
            "protocols":{
                "eth":{
                    "network":1337,
                    "difficulty":0,
                    "genesis":"0xb04009ddf4b0763f42778e7d5937e49bebf1e11b2d26c9dac6cefb5f84b6f8ea",
                    "config":{
                        "chainId":0,
                        "eip150Hash":"0x0000000000000000000000000000000000000000000000000000000000000000",
                        "terminalTotalDifficulty": "0xC70D808A128D7380000",
                        "terminalTotalDifficultyPassed":true,
                        "ethash":{}
                    },
                    "head":"0xb04009ddf4b0763f42778e7d5937e49bebf1e11b2d26c9dac6cefb5f84b6f8ea"
                },
                "snap":{}
            }
        }"#;

        let _: NodeInfo = serde_json::from_str(response).unwrap();
    }

    #[test]
    fn deserialize_node_info_mainnet_full() {
        let actual_response = r#"{
            "id": "74477ca052fcf55ee9eafb369fafdb3e91ad7b64fbd7ae15a4985bfdc43696f2",
            "name": "Geth/v1.10.26-stable/darwin-arm64/go1.19.3",
            "enode": "enode://962184c6f2a19e064e2ddf0d5c5a788c8c5ed3a4909b7f75fb4dad967392ff542772bcc498cd7f15e13eecbde830265f379779c6da1f71fb8fe1a4734dfc0a1e@127.0.0.1:13337?discport=0",
            "enr": "enr:-J-4QFttJyL3f2-B2TQmBZNFxex99TSBv1YtB_8jqUbXWkf6LOREKQAPW2bIn8kJ8QvHbWxCQNFzTX6sehjbrz1ZkSuGAYSyQ0_rg2V0aMrJhPxk7ASDEYwwgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQKWIYTG8qGeBk4t3w1cWniMjF7TpJCbf3X7Ta2Wc5L_VIRzbmFwwIN0Y3CCNBk",
            "ip": "127.0.0.1",
            "ports": {
                "discovery": 0,
                "listener": 13337
            },
            "listenAddr": "[::]:13337",
            "protocols": {
                "eth": {
                    "network": 1337,
                    "difficulty": 17179869184,
                    "genesis": "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3",
                    "config": {
                        "chainId": 1,
                        "homesteadBlock": 1150000,
                        "daoForkBlock": 1920000,
                        "daoForkSupport": true,
                        "eip150Block": 2463000,
                        "eip150Hash": "0x2086799aeebeae135c246c65021c82b4e15a2c451340993aacfd2751886514f0",
                        "eip155Block": 2675000,
                        "eip158Block": 2675000,
                        "byzantiumBlock": 4370000,
                        "constantinopleBlock": 7280000,
                        "petersburgBlock": 7280000,
                        "istanbulBlock": 9069000,
                        "muirGlacierBlock": 9200000,
                        "berlinBlock": 12244000,
                        "londonBlock": 12965000,
                        "arrowGlacierBlock": 13773000,
                        "grayGlacierBlock": 15050000,
                        "terminalTotalDifficulty": "0xC70D808A128D7380000",
                        "terminalTotalDifficultyPassed": true,
                        "ethash": {}
                    },
                    "head": "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3"
                },
                "snap": {}
            }
        }"#;

        let _: NodeInfo = serde_json::from_str(actual_response).unwrap();
    }
}
