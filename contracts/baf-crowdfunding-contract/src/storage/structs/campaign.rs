use soroban_sdk::{contracttype, Address, Map};

#[derive(Clone)]
#[contracttype]
pub struct Campaign {
    pub goal: i128,
    pub beneficiary: Address,
    pub min_donation: i128,
    pub total_raised: i128,
    pub supporters: u32,
    pub ong: Address,
    pub executed: bool,
    pub contributors: Map<Address, i128>,
}
