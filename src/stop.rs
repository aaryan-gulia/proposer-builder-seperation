use crate::blockchain_env::*;
use crate::entities::traits::Proposer;
use crate::entities::{builder, proposer};
use crate::setup::*;
use csv::WriterBuilder;
use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError, Uniform};
use serde::Serialize;
use std::collections::HashSet;
use std::{error::Error, io, process};

pub fn save_continuous_simulation_to_csv(
    blockchain: &Vec<block::Block>,
    file_name: &String,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(file_name)?;

    wtr.write_record(&[
        "builder_id",
        "builder_type",
        "proposer_id",
        "gas_captured",
        "mev_captured",
        "block_bid",
        "block_index",
    ])?;

    for b in blockchain.iter() {
        wtr.serialize(&[
            b.builder_id,
            match b.builder_type.clone().unwrap() {
                builder::BuilderType::NormalBuilder(_) => 1,
                builder::BuilderType::MevBuilder(_) => 0,
            },
            b.proposer_id.unwrap(),
            b.gas_captured as u32,
            b.mev_captured as u32,
            b.block_inclusion_bid
                .expect("PBS simulation blocks must have a bid value!") as u32,
            b.block_index.unwrap(),
        ])?;
    }
    Ok(())
}
