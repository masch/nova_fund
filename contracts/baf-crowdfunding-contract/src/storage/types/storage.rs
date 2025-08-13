use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Campaigns,
    NextID,
    Contribution(u32, Address), // (campaign_address, contributor)
}