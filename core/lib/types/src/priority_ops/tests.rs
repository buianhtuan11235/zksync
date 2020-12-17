use serde::{Deserialize, Serialize};

use crate::{
    priority_ops::{FullExit, PriorityOp},
    Address, SerialId, ZkSyncPriorityOp,
};

/// Tests the migration of `PriorityOp::eth_hash` from the `Vec<u8>` to `H256` type
mod backward_compatibility {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct OldPriorityOp {
        serial_id: SerialId,
        data: ZkSyncPriorityOp,
        deadline_block: u64,
        eth_hash: Vec<u8>,
        eth_block: u64,
    }

    fn old_value() -> OldPriorityOp {
        let operation = FullExit {
            account_id: 155,
            eth_address: Address::default(),
            token: 1000,
        };
        OldPriorityOp {
            serial_id: 12345,
            data: ZkSyncPriorityOp::FullExit(operation),
            deadline_block: 100,
            eth_hash: vec![2; 32],
            eth_block: 0,
        }
    }

    #[test]
    fn old_deserializes_to_new() {
        let old_value = old_value();
        let serialized = serde_json::to_value(old_value.clone()).unwrap();

        let new_value: PriorityOp = serde_json::from_value(serialized).unwrap();
        assert_eq!(old_value.serial_id, new_value.serial_id);
        assert_eq!(old_value.deadline_block, new_value.deadline_block);
        assert_eq!(old_value.eth_hash, new_value.eth_hash.as_bytes().to_vec());
        assert_eq!(old_value.eth_block, new_value.eth_block);
    }

    #[test]
    fn old_serializes_the_same_as_new() {
        let old_value = old_value();
        let old_serialized = serde_json::to_value(old_value).unwrap();

        let new_value: PriorityOp = serde_json::from_value(old_serialized.clone()).unwrap();
        let new_serialized = serde_json::to_value(new_value).unwrap();
        assert_eq!(old_serialized.to_string(), new_serialized.to_string());
    }

    #[test]
    fn new_roundtrip_unchanged() {
        let old_value = old_value();
        let old_serialized = serde_json::to_value(old_value).unwrap();

        let new_value: PriorityOp = serde_json::from_value(old_serialized).unwrap();
        let new_serialized = serde_json::to_value(new_value.clone()).unwrap();

        let new_value_restored: PriorityOp = serde_json::from_value(new_serialized).unwrap();
        assert_eq!(new_value_restored.serial_id, new_value.serial_id);
        assert_eq!(new_value_restored.deadline_block, new_value.deadline_block);
        assert_eq!(new_value_restored.eth_hash, new_value.eth_hash);
        assert_eq!(new_value_restored.eth_block, new_value.eth_block);
    }

    #[test]
    #[should_panic(expected = "31")]
    /// If the `PriorityOp::eth_hash` size is not 32 bytes, the deserialization cannot happen
    fn bad_format_cannot_be_deserialized() {
        let mut old_value = old_value();
        // remove the last element to shrink its size to 31
        let _ = old_value.eth_hash.pop().unwrap();

        let old_serialized = serde_json::to_value(old_value).unwrap();

        let _new_value: PriorityOp = serde_json::from_value(old_serialized).unwrap();
    }
}
