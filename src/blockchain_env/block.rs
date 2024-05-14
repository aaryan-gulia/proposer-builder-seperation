use crate::blockchain_env::transaction;
use crate::blockchain_env::transaction::serialize_as_string;
use crate::entities::builder;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Block {
    pub builder_id: u32,
    pub proposer_id: Option<u32>,
    pub gas_captured: f64,
    pub mev_captured: f64,
    pub block_inclusion_bid: Option<f64>,
    pub block_index: Option<u32>,
    pub transactions: HashSet<transaction::Transaction>,
    pub builder_type: Option<builder::BuilderType>,
}
static mut BLOCK_POSITION_INDEX: u32 = 0;

impl Block {
    pub fn new(
        builder_id: u32,
        gas_captured: f64,
        mev_captured: f64,
        block_inclusion_bid: Option<f64>,
        transaction_set: HashSet<transaction::Transaction>,
        builder_type: builder::BuilderType,
    ) -> Self {
        Block {
            builder_id,
            proposer_id: None,
            gas_captured,
            mev_captured,
            block_inclusion_bid,
            block_index: None,
            transactions: transaction_set,
            builder_type: Some(builder_type),
        }
    }

    pub fn add_to_chain(&mut self, proposer_id: u32) {
        unsafe {
            BLOCK_POSITION_INDEX += 1;
            self.block_index = Some(BLOCK_POSITION_INDEX);
        }
        self.proposer_id = Some(proposer_id);
    }

    pub fn get_block_index(&mut self) -> Option<u32> {
        self.block_index
    }

    pub fn compare_blocks_by_bid(a: &Block, b: &Block) -> std::cmp::Ordering {
        if a.block_inclusion_bid < b.block_inclusion_bid {
            return std::cmp::Ordering::Greater;
        }
        if a.block_inclusion_bid == b.block_inclusion_bid {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }
}
