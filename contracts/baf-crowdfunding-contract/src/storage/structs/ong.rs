use crate::storage::structs::campaign::Campaign;
use soroban_sdk::{contracttype, Address, Map};

#[derive(Clone)]
#[contracttype]
pub struct Ong {
    pub balance: i128,
    // pub campaigns: Map<Address, Campaign>,
    pub total_campaigns: u32,
}
