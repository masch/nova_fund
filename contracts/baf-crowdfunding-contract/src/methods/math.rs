use soroban_sdk::Env;

use crate::storage::minimum_donation::get_minimum_donation;
use crate::storage::types::error::Error;

pub(crate) fn checked_addition(amount1: i128, amount2: i128) -> Result<i128, Error> {
    if amount1 < 0 || amount2 < 0 {
        return Err(Error::AmountMustBePositive);
    }
    if amount2 > i128::MAX - amount1 {
        return Err(Error::MathOverflow);
    }
    Ok(amount1 + amount2)
}

pub(crate) fn checked_amount(env: &Env, amount: i128) -> Result<i128, Error> {
    if amount <= 0 {
        return Err(Error::AmountMustBePositive);
    }
    if amount % get_minimum_donation(env) != 0 {
        return Err(Error::IncorrectAmount);
    }
    Ok(amount)
}
