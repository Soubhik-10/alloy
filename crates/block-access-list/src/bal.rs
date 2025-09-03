//! Contains the `BlockAccessList` struct, which represents a simple list of account changes.

use crate::account_change::AccountChanges;
use alloc::vec::Vec;

/// Vector of account changes.
pub type BlockAccessList = Vec<AccountChanges>;

// #[cfg(test)]
// mod tests {
//     use alloy_primitives::{keccak256, Bytes};
//     use alloy_rlp::{EMPTY_LIST_CODE, EMPTY_STRING_CODE};

//     use crate::BlockAccessList;

//     #[test]
//     fn test_hash() {
//         // let bal = None;
//         // println!("bal default= {:?}", bal);
//         let bal = BlockAccessList::default();
//         // let rlp_encoded = alloy_rlp::encode("");
//         println!("RLP encoded bal default= {:?}", bal);
//         let hash = keccak256(alloy_rlp::encode(bal));

//         println!("hash {:?}", hash);
//     }
// }
