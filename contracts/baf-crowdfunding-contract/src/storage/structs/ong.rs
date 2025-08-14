use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub struct Ong {
    pub balance: i128,
    // pub campaigns: Map<Address, Campaign>,
    pub total_campaigns: u32,
}
