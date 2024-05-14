use crate::blockchain_env::block;
use crate::blockchain_env::transaction;
use crate::entities::builder;
use crate::entities::proposer;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::thread::available_parallelism;

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
                    let mut block = normal_builder.build_block(block_size);
                    let bid = builder::Builder::parse_bid_calculation(
                        (block.gas_captured + block.mev_captured) as i64,
                        blockchain,
                        random_numbers,
                    );
                    block.block_inclusion_bid = Some(bid);
                    block
                }
                builder::BuilderType::MevBuilder(mev_builder) => {
                    let mut block = mev_builder.build_block(block_size);
                    let bid = builder::Builder::parse_bid_calculation(
                        (block.gas_captured + block.mev_captured) as i64,
                        blockchain,
                        random_numbers,
                    );
                    block.block_inclusion_bid = Some(bid);
                    block
                }
            })
            .collect();
        let winning_block: &block::Block = submitted_blocks
            .iter()
            .max_by_key(|b| b.block_inclusion_bid.unwrap() as u32)
            .unwrap();

        let highest_bid = submitted_blocks
            .iter()
            .max_by_key(|b| b.block_inclusion_bid.unwrap() as u32)
            .unwrap()
            .block_inclusion_bid
            .unwrap();
        let highest_bid_blocks = submitted_blocks
            .iter()
            .filter(|b| b.block_inclusion_bid.unwrap() as u32 == highest_bid as u32)
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
