//use super::types::storage::DataKey;

// const CONTRIBUTORS: DataKey = DataKey::Contributors;

// pub(crate) fn has_contribution(env: &Env, contributor: Address, contributor: &Address) {
//     let contributors: Map<Address, i128> = env
//         .storage()
//         .persistent()
//         .get(&CONTRIBUTORS)
//         .unwrap_or_else(|| Map::new(env));

//     contributors.contains_key(contributor.clone())

//     // let key = DataKey::Contribution(campaign_id, contributor.clone());

//     // env.storage().instance().has(&key)
// }

// pub(crate) fn add_contribution(env: &Env, contributor: &Address, amount: i128) {
//     let mut contributors: Map<Address, i128> = env
//         .storage()
//         .persistent()
//         .get(&CONTRIBUTORS)
//         .unwrap_or_else(|| Map::new(env));

//     // 1. Get the current contributed amount for the address, defaulting to 0 if not present.
//     let current_amount = contributors.get(contributor.clone()).unwrap_or(0);

//     // 2. Calculate the new total amount.
//     let new_total_amount = current_amount + amount;

//     // 3. Update the map with the new total amount for the contributor.
//     contributors.set(contributor.clone(), new_total_amount);

//     // 4. Save the updated map back to storage.
//     env.storage().persistent().set(&CONTRIBUTORS, &contributors);
// }
/*
pub(crate) fn get_contribution(env: &Env, campaign_id: u32, contributor: &Address) -> i128 {
    // let key = DataKey::Contribution(campaign_id, contributor.clone());

    // env.storage().instance().get(&key).unwrap()
    2
}

pub(crate) fn remove_contribution(env: &Env, campaign_id: u32, contributor: &Address) {
    // let key = DataKey::Contribution(campaign_id, contributor.clone());

    // env.storage().instance().remove(&key);
}
 */
