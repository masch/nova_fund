use soroban_sdk::{Address, Env, Symbol};

use crate::storage::structs::campaign::Campaign;

pub(crate) fn add_campaign(env: &Env, campaign: Campaign, creator: &Address) {
    let topics = (Symbol::new(env, "add_campaign"),  creator);
    env.events().publish(topics, campaign);
}

pub (crate) fn withdraw(env: &Env, campaign_id: u32, total_raised: i128) {
    let topics = (Symbol::new(env, "withdraw"), campaign_id);
    env.events().publish(topics, &total_raised);
}