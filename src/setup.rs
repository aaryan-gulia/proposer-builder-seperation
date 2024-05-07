pub mod setup {

    use crate::blockchain_env::*;
    use crate::entities::*;
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal, NormalError};
    pub fn initiate_builders(
        num_builders: u32,
        builder_characteristic: f64,
    ) -> Vec<builder::Builder> {
        let mut builder_vec: Vec<builder::Builder> = vec![];
        for id in 1..=num_builders {
            builder_vec.push(builder::Builder::new(id, builder_characteristic));
        }
        builder_vec
    }

    pub fn initiate_proposers(num_proposers: u32) -> Vec<proposer::Proposer> {
        let mut proposer_vec: Vec<proposer::Proposer> = vec![];
        for id in 1..=num_proposers {
            proposer_vec.push(proposer::Proposer::new(id));
        }
        proposer_vec
    }

    pub fn initiate_builders_from_normal_dist(
        num_builders: u32,
        mean_characteristic: f64,
        std: f64,
    ) -> Vec<builder::Builder> {
        let mut builder_vec: Vec<builder::Builder> = vec![];
        let mut rng = thread_rng();
        let normal = Normal::new(mean_characteristic, std).unwrap();
        for id in 1..=num_builders {
            builder_vec.push(builder::Builder::new(id, normal.sample(&mut rng)));
        }
        builder_vec
    }
}
