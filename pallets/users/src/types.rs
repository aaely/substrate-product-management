use sp_std::prelude::*;
use codec::{Encode, Decode};

pub struct User<AccountId, Moment> {
    pub id: AccountId,
    pub fname: Vec<u8>,
    pub lname: Vec<u8>,
    pub addresses: Vec<HomeAddress>,
    pub phone_numbers: Vec<PhoneNumber>,
    pub created_at: Moment,
}

pub struct HomeAddress {
    pub street: Vec<u8>,
    pub city: Vec<u8>,
    pub state: Vec<u8>,
    pub zip: Vec<u8>,
}

pub struct PhoneNumber {
    pub number_type: Vec<u8>,
    pub number: Vec<u8>,
}