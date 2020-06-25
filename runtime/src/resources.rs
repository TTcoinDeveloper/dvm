use libra::move_core_types::{
    identifier::Identifier,
    language_storage::{StructTag, CORE_CODE_ADDRESS},
};
use serde_derive::{Deserialize, Serialize};

/// Height of the current block.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct BlockMetadata {
    pub height: u64,
}

/// A singleton resource holding the current Unix time in seconds.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct CurrentTimestamp {
    pub seconds: u64,
}

/// Returns block metadata struct tag.
pub fn block_metadata() -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        name: Identifier::new("BlockMetadata").expect("Valid module name."),
        module: Identifier::new("Block").expect("Valid module name."),
        type_params: vec![],
    }
}

/// Returns time metadata struct tag.
pub fn time_metadata() -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        name: Identifier::new("CurrentTimestamp").expect("Valid module name."),
        module: Identifier::new("Time").expect("Valid module name."),
        type_params: vec![],
    }
}