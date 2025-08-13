use soroban_sdk::{Env};

use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, remove_campaign},
        types::error::Error
    }
};

pub fn withdraw(env: &Env, campaign_id: u32) -> Result<(), Error> {
    // creator.require_auth();

    let campaign = get_campaign(env, campaign_id)?;

    if campaign.total_raised != campaign.goal {
        return Err(Error::CampaignGoalNotReached);
    }

    token_transfer(
        &env,
        &env.current_contract_address(),
        &campaign.beneficiary,
        &campaign.total_raised
    )?;

    remove_campaign(env, campaign_id);
    events::campaign::withdraw(&env, campaign_id, campaign.total_raised);
    
    Ok(())
}