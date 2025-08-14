use soroban_sdk::{Address, Env};

use crate::storage::{admin::get_admin, ong::new_ong, types::error::Error};

pub fn add_ong(env: &Env, ong: Address) -> Result<(), Error> {
    let current_admin = get_admin(env);
    current_admin.require_auth();

    //let ong = Ong {};

    new_ong(&env, ong);
    //events::campaign::add_campaign(&env, campaign, &creator);
    Ok(())
}
