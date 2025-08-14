use soroban_sdk::{Address, Env, Map};

use crate::{
    events,
    storage::{
        campaign::exist_campaign, campaign::new_campaign, ong::exist_ong,
        ong::get_ong_by_address_and_increment_ong_campaigns, structs::campaign::Campaign,
        types::error::Error,
    },
};

pub fn add_campaign(
    env: &Env,
    creator: Address,
    beneficiary: Address,
    goal: i128,
    min_donation: i128,
) -> Result<(), Error> {
    creator.require_auth(); // TODO: Chequear self para validar wihtelist

    if !exist_ong(env, creator.clone()) {
        return Err(Error::OngCannotCreateCampaign);
    }

    if exist_campaign(env, beneficiary.clone()) {
        return Err(Error::CampaignAlreadyExists);
    }

    get_ong_by_address_and_increment_ong_campaigns(env, creator.clone())?;

    let campaign: Campaign = Campaign {
        goal,
        min_donation,
        total_raised: 0,
        supporters: 0,
        beneficiary,
        executed: false,
        contributors: Map::new(env),
    };

    new_campaign(&env, campaign.clone());
    events::campaign::add_campaign(&env, campaign, &creator);
    Ok(())
}
