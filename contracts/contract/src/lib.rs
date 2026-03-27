#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, String, Vec, Address
};

#[contract]
pub struct Contract;

#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub votes: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Count,
    Proposal(u32),
    Voted(u32, Address),
}

#[contractimpl]
impl Contract {

    // 🆕 Create Proposal
    pub fn create_proposal(env: Env, title: String, description: String) -> u32 {
        let mut count: u32 = env.storage().instance().get(&DataKey::Count).unwrap_or(0);

        count += 1;

        let proposal = Proposal {
            id: count,
            title,
            description,
            votes: 0,
        };

        env.storage().instance().set(&DataKey::Proposal(count), &proposal);
        env.storage().instance().set(&DataKey::Count, &count);

        count
    }

    // 🗳️ Vote
    pub fn vote(env: Env, proposal_id: u32, voter: Address) {
        voter.require_auth();

        let voted_key = DataKey::Voted(proposal_id, voter.clone());

        if env.storage().instance().has(&voted_key) {
            panic!("Already voted");
        }

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found");

        proposal.votes += 1;

        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage().instance().set(&voted_key, &true);
    }

    // 📊 Get Proposal
    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        env.storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Not found")
    }

    // 📋 Get Count
    pub fn get_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Count).unwrap_or(0)
    }

    // 🏆 Get Winner
    pub fn get_winner(env: Env) -> Proposal {
        let count: u32 = env.storage().instance().get(&DataKey::Count).unwrap_or(0);

        if count == 0 {
            panic!("No proposals");
        }

        let mut winner: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(1))
            .unwrap();

        let mut i = 2;
        while i <= count {
            let current: Proposal = env
                .storage()
                .instance()
                .get(&DataKey::Proposal(i))
                .unwrap();

            if current.votes > winner.votes {
                winner = current;
            }

            i += 1;
        }

        winner
    }
}

mod test;