use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::{
    methods::{
        add_campaign::add_campaign, add_ong::add_ong, contribute::contribute,
        get_campaign::get_campaign, initialize::initialize, refund::refund, withdraw::withdraw,
    },
    storage::{structs::campaign::Campaign, types::error::Error},
};

#[contract]
pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {
    pub fn __constructor(
        env: Env,
        admin: Address,
        token: Address,
        minimum_donation: i128,
    ) -> Result<(), Error> {
        initialize(&env, admin, token, minimum_donation)
    }

    pub fn create_ong(env: Env, ong: Address) -> Result<(), Error> {
        add_ong(&env, ong)
    }

    pub fn create_campaign(
        env: Env,
        creator: Address,
        beneficiary: Address,
        goal: i128,
        min_donation: i128,
    ) -> Result<(), Error> {
        add_campaign(&env, creator, beneficiary, goal, min_donation)
    }

    pub fn get_campaign(env: Env, campaign_id: Address) -> Result<Campaign, Error> {
        get_campaign(&env, campaign_id)
    }

    pub fn contribute(
        env: Env,
        contributor: Address,
        campaign_id: Address,
        amount: i128,
    ) -> Result<(), Error> {
        contribute(&env, contributor, campaign_id, amount)
    }

    pub fn withdraw(env: Env, campaign_id: Address) -> Result<(), Error> {
        withdraw(&env, campaign_id)
    }

    pub fn refund(env: Env, contributor: Address, campaign_id: Address) -> Result<(), Error> {
        refund(&env, contributor, campaign_id)
    }
}
