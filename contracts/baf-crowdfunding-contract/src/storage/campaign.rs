use soroban_sdk::{Env, Vec};

use crate::storage::{structs::campaign::Campaign, types::error::Error};

use super::types::storage::DataKey;

// pub(crate) fn has_campaign(env: &Env, creator: &Address) -> bool {
//     let key = DataKey::Campaigns(creator.clone());

//     env.storage().instance().has(&key)
// }

pub(crate) fn new_campaign(env: &Env, campaign: &Campaign) {
    let storage = env.storage().persistent();
    let mut current_campaign = storage
        .get::<DataKey, Vec<Campaign>>(&DataKey::Campaigns)
        .unwrap_or_else(|| Vec::new(&env)); // If key doesn't exist, start with an empty Vec

    current_campaign.push_back(campaign.clone());
    storage.set(&DataKey::Campaigns, &current_campaign);
}

pub(crate) fn set_campaign(env: &Env, campaign: Campaign) {
    let storage = env.storage().persistent();
    let mut current_campaign = storage
        .get::<DataKey, Vec<Campaign>>(&DataKey::Campaigns)
        .unwrap_or_else(|| Vec::new(&env)); // If key doesn't exist, start with an empty Vec

    current_campaign.set(campaign.id, campaign);
    storage.set(&DataKey::Campaigns, &current_campaign);
}

pub(crate) fn get_campaign(env: &Env, id: u32) -> Result<Campaign, Error> {
    let storage = env.storage().persistent();

    // 1. Get the vector of campaigns. This returns an Option<Vec<Campaign>>.
    let campaigns_vec = storage
        .get::<DataKey, Vec<Campaign>>(&DataKey::Campaigns)
        // 2. Convert the Option into a Result. If it's None, it becomes Err.
        // The `?` operator will automatically return Err(Error::CampaignNotFound) from the function if it's an error.
        .ok_or(Error::CampaignNotFound)?;

    // 3. If we are here, `campaigns_vec` is a valid Vec<Campaign>.
    // Now, get the campaign at the specified index `id`. This returns an Option<Campaign>.
    campaigns_vec
        .get(id)
        // 4. Convert this final Option into a Result to match the function's return type.
        .ok_or(Error::CampaignNotFound)
}

//  pub(crate) fn remove_campaign(env: &Env, campaign_id: u32) {
//     let storage = env.storage().persistent();
//     let mut current_campaign = storage
//         .get::<DataKey, Vec<Campaign>>(&DataKey::Campaigns)
//         .unwrap_or_else(|| Vec::new(&env)); // If key doesn't exist, start with an empty Vec

//     current_campaign.remove(campaign_id);
//     storage.set(&DataKey::Campaigns, &current_campaign);
//  }

pub(crate) fn get_and_increment_next_id(env: &Env) -> u32 {
    let current_id: u32 = env.storage().instance().get(&DataKey::NextID).unwrap_or(0);

    env.storage()
        .instance()
        .set(&DataKey::NextID, &(current_id + 1));

    current_id
}

// pub(crate) fn get_next_id(env: &Env) -> u32 {
//     env.storage().instance().get(&DataKey::NextID).unwrap_or(0)
// }
