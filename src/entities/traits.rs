use crate::blockchain_env::block;
use crate::blockchain_env::transaction;
use crate::entities::builder;
use crate::entities::proposer;
use rand::Rng;
use rayon::prelude::*;
use std::thread::available_parallelism;

pub trait Entity {
    fn get_characteristic(&self) -> f64;

    fn access_mempool(self: &mut Self) -> Box<&mut Vec<&Box<transaction::Transaction>>>;
}
pub trait Builder: Entity {
    fn collect_transactions(&mut self, all_transactions: &Vec<Box<transaction::Transaction>>) {
        let mut rng = rand::thread_rng();
        for t in all_transactions.iter() {
            if self.get_characteristic() * 100.0 > rng.gen_range(0.0..100.0) {
                self.access_mempool().push(t);
            }
        }
    }
}

pub trait Proposer {
    fn run_auction(
        &self,
        builders_vec: &mut Vec<builder::Builder>,
        block_size: u32,
        blockchain: &Vec<block::Block>,
        random_numbers: &Vec<f64>,
    ) -> block::Block {
        let mut submitted_blocks: Vec<block::Block> = builders_vec
            .into_iter() // Iterate in parallel using Rayon
            .map(|b| b.build_block(block_size, blockchain, random_numbers))
            .collect();
        submitted_blocks.sort_unstable_by(block::Block::compare_blocks_by_bid);

        // let chunk_size = 100;
        // builders_vec
        //     .par_chunks_mut(chunk_size)
        //     .for_each(|builder_chunk| {
        //         for builder in builder_chunk {
        //             builder.clean_mempools(&submitted_blocks[0].transactions);
        //         }
        //     });
        builders_vec
            .into_iter()
            .for_each(|b| b.clean_mempools(&submitted_blocks[0].transactions));
        submitted_blocks[0].clone()
    }
    fn propose_block(&self, p: &proposer::Proposer, proposed_block: &mut block::Block) {
        proposed_block.add_to_chain(p.id);
    }
}
