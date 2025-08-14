use soroban_sdk::{Address, Env};

use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, set_campaign},
        types::error::Error,
    },
};

pub fn refund(env: &Env, contributor: Address, campaign_id: Address) -> Result<(), Error> {
    contributor.require_auth();

    let mut campaign = get_campaign(env, campaign_id.clone())?;

    if campaign.executed {
        return Err(Error::CampaignAlreadyExecuted);
    }

    let current_contribution = campaign.contributors.get(contributor.clone()).unwrap_or(0);
    if current_contribution == 0 {
        return Err(Error::ContributionNotFound);
    }

    //let amount = get_contribution(env, campaign_id, &contributor);
    token_transfer(
        &env,
        &env.current_contract_address(),
        &contributor,
        &current_contribution,
    )?;

    campaign.total_raised -= current_contribution;
    campaign.supporters -= 1;

    //remove_contribution(env, campaign_id, &contributor);
    set_campaign(env, &campaign_id, &campaign);
    events::refund::refund(&env, &contributor, campaign_id, &current_contribution);

    Ok(())
}
