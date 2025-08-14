use soroban_sdk::{Address, Env, Map};

use crate::storage::{structs::ong::Ong, types::error::Error};

use super::types::storage::DataKey;

const ONGS: DataKey = DataKey::Ongs;

pub(crate) fn new_ong(env: &Env, address: Address) {
    let mut ongs: Map<Address, Ong> = env
        .storage()
        .persistent()
        .get(&ONGS)
        .unwrap_or_else(|| Map::new(env));

    if ongs.contains_key(address.clone()) {
        panic!("ONG has already added");
    }

    let ong = Ong {
        balance: 0,
        total_campaigns: 0,
    };

    ongs.set(address.clone(), ong);
    env.storage().persistent().set(&ONGS, &ongs)
}

pub(crate) fn exist_ong(env: &Env, address: Address) -> bool {
    let ongs: Map<Address, u32> = env
        .storage()
        .persistent()
        .get(&ONGS)
        .unwrap_or_else(|| Map::new(env));

    ongs.contains_key(address.clone())
}

// pub(crate) fn get_ong_by_address(env: &Env, address: Address) -> Result<Ong, Error> {
//     let ongs: Map<Address, Ong> = env
//         .storage()
//         .persistent()
//         .get(&ONGS)
//         .unwrap_or_else(|| Map::new(env));

//     ongs.get(address.clone()).ok_or(Error::OngNotFound)
// }

pub(crate) fn get_ong_by_address_and_increment_ong_campaigns(
    env: &Env,
    address: Address,
) -> Result<Ong, Error> {
    // 1. Get the map of all ONGs from storage
    let mut ongs: Map<Address, Ong> = env
        .storage()
        .persistent()
        .get(&ONGS)
        .unwrap_or_else(|| Map::new(env));

    // 2. Get the specific ONG by its address, returning an error if not found.
    // The `?` operator handles the error case for you.
    let mut ong = ongs.get(address.clone()).ok_or(Error::OngNotFound)?;

    // 3. Modify the struct. Here we increment the total_campaigns counter.
    ong.total_campaigns += 1;

    // 4. Put the modified struct back into the map, using the same key.
    ongs.set(address.clone(), ong.clone());

    // 5. Save the entire updated map back to persistent storage.
    env.storage().persistent().set(&ONGS, &ongs);

    // 6. Return the updated Ong struct
    Ok(ong)
}

pub(crate) fn add_increment_ong_balance(
    env: &Env,
    address: &Address,
    amount: i128,
) -> Result<Ong, Error> {
    // 1. Get the map of all ONGs from storage
    let mut ongs: Map<Address, Ong> = env
        .storage()
        .persistent()
        .get(&ONGS)
        .unwrap_or_else(|| Map::new(env));

    // 2. Get the specific ONG by its address, returning an error if not found.
    // The `?` operator handles the error case for you.
    let mut ong = ongs.get(address.clone()).ok_or(Error::OngNotFound)?;

    // 3. Modify the struct. Here we increment the total_campaigns counter.
    ong.balance += amount;

    // 4. Put the modified struct back into the map, using the same key.
    ongs.set(address.clone(), ong.clone());

    // 5. Save the entire updated map back to persistent storage.
    env.storage().persistent().set(&ONGS, &ongs);

    // 6. Return the updated Ong struct
    Ok(ong)
}
