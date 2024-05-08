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

pub fn save_blockchain_to_csv(
    blockchain: &Vec<block::Block>,
    file_name: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_path(file_name)?;
    //    wtr.write_record(&[
    //        "builder_id",
    //        "proposer_id",
    //        "gas_captured",
    //        "mev_captured",
    //        "block_bid",
    //        "index",
    //    ])?;
    for b in blockchain.iter() {
        wtr.serialize(b)?;
    }
    wtr.flush()?;

    Ok(())
}
