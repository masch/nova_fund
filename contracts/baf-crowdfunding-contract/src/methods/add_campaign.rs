use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::get_admin,  campaign::new_campaign, campaign::get_and_increment_next_id, structs::campaign::Campaign, types::{error::Error}
    },
};

pub fn add_campaign(env: &Env, creator: Address, beneficiary: Address, goal: i128, min_donation: i128) -> Result<(), Error> {
    let current_admin = get_admin(env);
    current_admin.require_auth();

    let id = get_and_increment_next_id(env);

    let campaign = Campaign {
        goal,
        min_donation,
        total_raised: 0,
        supporters: 0,
        id,
        beneficiary,
        executed: false,
    };


    new_campaign(&env, &campaign);
    events::campaign::add_campaign(&env, campaign, &creator);
    Ok(())
}