use std::thread::AccessError;

use sp_std::prelude::*;
use codec::{Encode, Decode};
use core::fmt;

pub type Decimal = I16F16;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Order<AccountId, Moment> {
    pub id: u128,
    pub products: Vec<u128>,
    pub order_total: u32,
    pub user: AccountId,
    pub order_date: Moment,
    pub order_status: OrderStatus,
    pub tracking_number: Vec<u8>,
    pub carrier: Carrier,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum OrderStatus {
    Received,
    Filled,
    Shipped,
    InTransit,
    Delivered,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum Carrier {
    USPS,
    UPS,
    FedEx,
}

pub enum OrderEventType {
    OrderRegistration,
    OrderPickup,
    OrderScan,
    pubOrderDeliver,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum ShippingOperation {
    Pickup,
    Scan,
    Deliver,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OrderEvent<Moment> {
    pub order_event_type: OrderEventType,
    pub order_id: u128,
    pub timestamp: Moment,
}

impl From<OrderEvent> for OrderEventType {
    fn from(op: ShippingOperation) -> Self {
        match op {
            ShippingOperation::Pickup => OrderEventType::OrderPickup,
            ShippingOperation::Scan => OrderEventType::OrderScan,
            ShippingOperation::Deliver => OrderEventType::OrderDeliver,
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Location {
    pub longitude: Decimal,
    pub latitude: Decimal,
}