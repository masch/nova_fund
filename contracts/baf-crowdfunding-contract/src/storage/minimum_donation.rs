use soroban_sdk::Env;

use super::types::storage::DataKey;

pub(crate) fn set_minimum_donation(env: &Env, minimum_donation: &i128) {
    let key = DataKey::MinimumDonation;

    env.storage().instance().set(&key, minimum_donation);
}

pub(crate) fn get_minimum_donation(env: &Env) -> i128 {
    let key = DataKey::MinimumDonation;

    env.storage().instance().get(&key).unwrap()
}
