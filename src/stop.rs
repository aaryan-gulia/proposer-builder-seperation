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
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_path(file_name)?;
    let mut record = vec![
        "builder_id",
        "builder_type",
        "proposer_id",
        "gas_captured",
        "mev_captured",
        "block_bid",
        "block_index",
    ];

    for _ in 0..blockchain.get(0).unwrap().transactions.len() {
        record.push("transaction_id");
        record.push("gas");
        record.push("mev");
        record.push("transaction_type");
        record.push("block_created");
    }

    wtr.write_record(&record)?;
    for b in blockchain.iter() {
        let mut record = vec![
            b.builder_id.to_string(),
            match b.builder_type.clone().unwrap() {
                builder::BuilderType::NormalBuilder(_) => 1.to_string(),
                builder::BuilderType::MevBuilder(_) => 0.to_string(),
            },
            b.proposer_id.unwrap().to_string(),
            b.gas_captured.to_string(),
            b.mev_captured.to_string(),
            b.block_inclusion_bid
                .expect("PBS simulation blocks must have a bid value!")
                .to_string(),
            b.block_index.unwrap().to_string(),
        ];
        for t in b.transactions.iter() {
            record.push(t.get_transaction_id().unwrap().to_string());
            record.push(t.gas_amount.to_string());
            record.push(t.max_mev_amount.to_string());
            record.push(t.transaction_type.to_string());
            record.push(t.block_created.to_string());
        }
        wtr.write_record(&record)?;
    }
    Ok(())
}

pub fn save_pos_to_csv(
    blockchain: &Vec<block::Block>,
    file_name: &String,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(file_name)?;
    wtr.write_record(&["builder_id", "builder_type", "gas_captured", "mev_captured"])?;

    for b in blockchain.iter() {
        wtr.serialize(&[
            b.builder_id,
            match b.builder_type.clone().unwrap() {
                builder::BuilderType::NormalBuilder(_) => 1,
                builder::BuilderType::MevBuilder(_) => 0,
            },
            b.gas_captured as u32,
            b.mev_captured as u32,
        ])?;
    }
    Ok(())
}
