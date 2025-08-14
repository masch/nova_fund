use soroban_sdk::{Address, Env, Map};

use crate::storage::{structs::campaign::Campaign, types::error::Error};

use super::types::storage::DataKey;

// pub(crate) fn has_campaign(env: &Env, creator: &Address) -> bool {
//     let key = DataKey::Campaigns(creator.clone());

//     env.storage().instance().has(&key)
// }

const CAMPAIGNS: DataKey = DataKey::Campaigns;

pub(crate) fn new_campaign(env: &Env, campaign: Campaign) {
    let mut campaigns: Map<Address, Campaign> = env
        .storage()
        .persistent()
        .get(&CAMPAIGNS)
        .unwrap_or_else(|| Map::new(env));

    if campaigns.contains_key(campaign.beneficiary.clone()) {
        panic!("CAMPAIGN has been already added");
    }

    campaigns.set(campaign.beneficiary.clone(), campaign.clone());
    env.storage().persistent().set(&CAMPAIGNS, &campaigns)
}

pub(crate) fn exist_campaign(env: &Env, address: Address) -> bool {
    let campaigns: Map<Address, Campaign> = env
        .storage()
        .persistent()
        .get(&CAMPAIGNS)
        .unwrap_or_else(|| Map::new(env));

    campaigns.contains_key(address.clone())
}

pub(crate) fn set_campaign(env: &Env, campaign_id: &Address, campaign: &Campaign) {
    let mut campaigns: Map<Address, Campaign> = env
        .storage()
        .persistent()
        .get(&CAMPAIGNS)
        .unwrap_or_else(|| Map::new(env));

    campaigns.set(campaign_id.clone(), campaign.clone());

    // Saves the updated map back to storage
    env.storage().persistent().set(&CAMPAIGNS, &campaigns);
}

pub(crate) fn get_campaign(env: &Env, id: Address) -> Result<Campaign, Error> {
    let campaigns: Map<Address, Campaign> = env
        .storage()
        .persistent()
        .get(&CAMPAIGNS)
        .unwrap_or_else(|| Map::new(env));

    campaigns.get(id.clone()).ok_or(Error::CampaignNotFound)
}

//  pub(crate) fn remove_campaign(env: &Env, campaign_id: u32) {
//     let storage = env.storage().persistent();
//     let mut current_campaign = storage
//         .get::<DataKey, Vec<Campaign>>(&DataKey::Campaigns)
//         .unwrap_or_else(|| Vec::new(&env)); // If key doesn't exist, start with an empty Vec

//     current_campaign.remove(campaign_id);
//     storage.set(&DataKey::Campaigns, &current_campaign);
//  }

// pub(crate) fn get_and_increment_next_id(env: &Env, ong: &Ong) -> u32 {
//     let current_id: u32 = env.storage().instance().get(&DataKey::NextID).unwrap_or(0);

//     env.storage()
//         .instance()
//         .set(&DataKey::NextID, &(current_id + 1));

//     current_id
// }

// pub(crate) fn get_next_id(env: &Env) -> u32 {
//     env.storage().instance().get(&DataKey::NextID).unwrap_or(0)
// }
