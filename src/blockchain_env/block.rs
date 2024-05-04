#[derive(Debug)]
pub struct Block {
    pub builder_id: u32,
    pub proposer_id: Option<u32>,
    pub gas_captured: f64,
    pub mev_captured: f64,
    pub block_inclusion_bid: f64,
    block_index: Option<u32>,
}

static mut BLOCK_POSITION_INDEX: u32 = 0;

impl Block {
    pub fn new(
        builder_id: u32,
        gas_captured: f64,
        mev_captured: f64,
        block_inclusion_bid: f64,
    ) -> Self {
        Block {
            builder_id,
            proposer_id: None,
            gas_captured,
            mev_captured,
            block_inclusion_bid,
            block_index: None,
        }
    }

    pub fn add_to_chain(&mut self) {
        unsafe {
            BLOCK_POSITION_INDEX += 1;
            self.block_index = Some(BLOCK_POSITION_INDEX);
        }
    }

    pub fn get_block_index(&mut self) -> Option<u32> {
        self.block_index
    }
}
