use crate::types::*;
use frame_support::sp_std::prelude::*;

#[derive(Default, Debug)]
pub struct OrderBuilder<AccountId, Moment>
    where 
    AccountId: Default,
    Moment: Default,
{
        id: u128,
        user: AccountId,
        products: Vec<u128>,
        cost: u32,
        order_date: Moment,
}

impl<AccountId, Moment> OrderBuilder<AccountId, Moment> 
where
    AccountId: Default,
    Moment: Default,
{
        pub fn calc_cost(&mut self) -> &Self {
            let total: u32 = 0;
            for n in &self.products {
                total += n;
            }
            &self.cost = &total;
            &self
        }
}

pub struct OrderEventBuilder<Moment> 
where
    Moment: Default 
{
        order_id: u128,
        event_type: OrderEventType,
        location: Location,
        timestamp: Moment,
}

impl<Moment> Default for OrderEventBuilder<Moment> 
where
    Moment: Default
{
        fn Default() -> Self {
            OrderEventBuilder {
                order_id: 0,
                event_type: OrderEventType::OrderPickup,
                location: Location::default(),
                timestamp: Moment::default(),
            }
        }
}

impl<Moment> OrderBuilder<Moment>
where
Moment: Default 
{
    pub fn for_order(&mut self, id: &u128) -> Self {
        self.id = id;
        self
    }
}