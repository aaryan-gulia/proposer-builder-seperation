use crate::blockchain_env::block;
use crate::blockchain_env::transaction;
use crate::entities::traits;
use rand::distributions::{Distribution, Uniform};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct Proposer {
    pub id: u32,
}

impl Proposer {
    pub fn new(id: u32) -> Self {
        Proposer { id }
    }
}

impl traits::Proposer for Proposer {}
