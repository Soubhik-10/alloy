#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(test)]
extern crate self as alloy_contract;

mod eth_call;
pub use eth_call::{CallDecoder, EthCall};

mod storage_slot;
pub use storage_slot::*;

mod error;
pub use error::{Error, Result, TransportErrorExt, TryParseTransportErrorResult};

mod event;
pub use event::{Event, EventPoller};

#[cfg(feature = "pubsub")]
pub use event::subscription::EventSubscription;

mod interface;
pub use interface::*;

mod instance;
pub use instance::*;

mod call;
pub use call::*;

mod multicall;

// Not public API.
// NOTE: please avoid changing the API of this module due to its use in the `sol!` macro.
#[doc(hidden)]
pub mod private {
    pub use alloy_network::{Ethereum, Network};
    pub use alloy_provider::Provider;
}
