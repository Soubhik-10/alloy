//! Contains the `BlockAccessList` struct, which represents a simple list of account changes.

use crate::account_change::AccountChanges;
use alloc::vec::Vec;

/// Vector of account changes.
pub type BlockAccessList = Vec<AccountChanges>;

// #[cfg(test)]
// mod tests {
//     use alloy_primitives::keccak256;

//     use crate::BlockAccessList;

//     #[test]
//     fn test_hash() {
//         let bal = BlockAccessList::default();
//         println!("bal default= {:?}", bal);
//         let rlp_encoded = alloy_rlp::encode(bal);
//         println!("RLP encoded bal default= {:?}", rlp_encoded);
//         let hash = keccak256(rlp_encoded);
//         println!("hash {:?}", hash);
//     }
// }
