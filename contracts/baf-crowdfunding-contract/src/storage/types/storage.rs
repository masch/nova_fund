use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Campaigns,
    Contributors,
    Ongs,
    MinimumDonation,
}
