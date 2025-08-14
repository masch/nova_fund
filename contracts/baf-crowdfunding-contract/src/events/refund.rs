use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn refund(env: &Env, contributor: &Address, campaign_id: Address, amount: &i128) {
    let topics = (Symbol::new(env, "refund"), contributor);
    let data = (campaign_id, amount);
    env.events().publish(topics, data);
}
