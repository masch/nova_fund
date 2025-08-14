use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::{has_admin, set_admin},
        minimum_donation::set_minimum_donation,
        token::set_token,
        types::error::Error,
    },
};

pub fn initialize(
    env: &Env,
    admin: Address,
    token: Address,
    minimum_donation: i128,
) -> Result<(), Error> {
    if has_admin(env) {
        return Err(Error::ContractInitialized);
    }

    set_admin(&env, &admin);
    set_token(&env, &token);
    set_minimum_donation(&env, &minimum_donation);
    events::contract::contract_initialized(&env, &admin, &token);

    Ok(())
}
