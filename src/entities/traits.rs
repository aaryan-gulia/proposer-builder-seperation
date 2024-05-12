use crate::blockchain_env::block;
use crate::blockchain_env::transaction;
use crate::entities::builder;
use crate::entities::proposer;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
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
        builders_vec: &mut Vec<builder::BuilderType>,
        block_size: u32,
        blockchain: &Vec<block::Block>,
        random_numbers: &Vec<f64>,
    ) -> block::Block {
        let submitted_blocks: Vec<block::Block> = builders_vec
            .into_iter() // Iterate in parallel using Rayon
            .map(|b| match b {
                builder::BuilderType::NormalBuilder(normal_builder) => {
                    normal_builder.build_block(block_size, blockchain, random_numbers)
                }
                builder::BuilderType::MevBuilder(mev_builder) => {
                    mev_builder.build_block(block_size, blockchain, random_numbers)
                }
            })
            .collect();
        let winning_block: &block::Block = submitted_blocks
            .iter()
            .max_by_key(|b| b.block_inclusion_bid as u32)
            .unwrap();

        let highest_bid = submitted_blocks
            .iter()
            .max_by_key(|b| b.block_inclusion_bid as u32)
            .unwrap()
            .block_inclusion_bid;
        let highest_bid_blocks = submitted_blocks
            .iter()
            .filter(|b| b.block_inclusion_bid as u32 == highest_bid as u32)
            .collect::<Vec<_>>();

        let mut rng = thread_rng();
        let random_highest_bid_block = highest_bid_blocks.choose(&mut rng).unwrap();
        // let chunk_size = 100;
        // builders_vec
        //     .par_chunks_mut(chunk_size)
        //     .for_each(|builder_chunk| {
        //         for builder in builder_chunk {
        //             builder.clean_mempools(&submitted_blocks[0].transactions);
        //         }
        //     });
        builders_vec.into_iter().for_each(|b| match b {
            builder::BuilderType::NormalBuilder(normal_builder) => normal_builder
                .builder
                .clean_mempools(&winning_block.transactions),
            builder::BuilderType::MevBuilder(mev_builder) => {
                mev_builder
                    .builder
                    .clean_mempools(&random_highest_bid_block.transactions);
            }
        });
        (*random_highest_bid_block).clone()
    }
    fn propose_block(&self, p: &proposer::Proposer, proposed_block: &mut block::Block) {
        proposed_block.add_to_chain(p.id);
    }
}
