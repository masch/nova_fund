use crate::{
    events,
    methods::math::{checked_addition, checked_amount},
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, set_campaign},
        ong::add_increment_ong_balance,
        types::error::Error,
    },
};
use soroban_sdk::{Address, Env};

pub fn contribute(
    env: &Env,
    contributor: Address,
    campaign_id: Address,
    amount: i128,
) -> Result<(), Error> {
    contributor.require_auth();

    let amount_validated = checked_amount(env, amount)?;

    let mut campaign = get_campaign(env, campaign_id.clone())?;
    if campaign.min_donation > amount_validated {
        return Err(Error::ContributionBelowMinimum);
    }

    let new_contribution_amount = checked_addition(campaign.total_raised, amount_validated)?;
    if new_contribution_amount > campaign.goal {
        return Err(Error::CampaignGoalExceeded);
    }

    token_transfer(
        &env,
        &contributor.clone(),
        &env.current_contract_address(),
        &amount_validated,
    )?;

    // Update campaign state
    campaign.total_raised = new_contribution_amount;

    // Update contributor's total contribution
    let current_contribution = campaign.contributors.get(contributor.clone()).unwrap_or(0);
    if current_contribution == 0 {
        // If it's a new contributor, increment supporters count
        campaign.supporters = checked_addition(campaign.supporters.into(), 1)?
            .try_into()
            .unwrap();
    }
    let new_contribution = checked_addition(current_contribution, amount_validated)?;
    campaign
        .contributors
        .set(contributor.clone(), new_contribution);

    // Save the updated campaign
    set_campaign(env, &campaign_id, &campaign);

    add_increment_ong_balance(&env, &campaign.ong, amount_validated)?;

    events::contribute::add_contribute(&env, &contributor, &campaign_id, &amount);

    Ok(())
}
