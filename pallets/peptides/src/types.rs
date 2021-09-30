use sp_std::prelude::*;
use codec::{Encode, Decode};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default)]
pub struct Peptide<AccountId, Moment> {
	pub id: u128,
	pub name: Vec<u8>,
	pub price: u32,
	pub inventory: u32,
	pub created_by: AccountId,
	pub created_at: Moment,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default)]
pub struct PeptideProfile {
	pub product_ref: u128,
	pub chain: Vec<u128>,
	pub production_cost: u32,
	pub production_yield: u32,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default)]
pub struct AminoAcid {
	pub id: u128,
	pub name: Vec<u8>,
	pub cost: u32,
}