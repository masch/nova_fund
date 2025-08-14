use soroban_sdk::{Address, Env};

use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, set_campaign},
        types::error::Error,
    },
};

pub fn withdraw(env: &Env, campaign_id: Address) -> Result<(), Error> {
    // creator.require_auth();

    let mut campaign = get_campaign(env, campaign_id.clone())?;

    let total_raised = campaign.total_raised;
    if campaign.total_raised != campaign.goal {
        return Err(Error::CampaignGoalNotReached);
    }

    token_transfer(
        &env,
        &env.current_contract_address(),
        &campaign.beneficiary,
        &campaign.total_raised,
    )?;

    campaign.executed = true;
    set_campaign(env, &campaign.beneficiary, &campaign);

    //remove_campaign(env, campaign_id);
    events::campaign::withdraw(&env, campaign_id, total_raised);

    Ok(())
}
