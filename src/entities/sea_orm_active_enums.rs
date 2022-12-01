//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "transaction_type")]
pub enum TransactionType {
    /// For **finalized** incoming transactions - all items within suborders of this transaction will be considered as additive towards inventory (i.e. incoming shipment of goods)
    #[sea_orm(string_value = "in")]
    In,
    /// For **finalized** outgoing transactions - all items within suborders of this transaction will be considered as subtractive towards inventory (i.e. sale of good to customer)
    #[sea_orm(string_value = "out")]
    Out,
    /// For **proposed** incoming transactions - all items within suborders of this transaction will be considered as additive towards inventory upon finalization (i.e. quote/purchase order for incoming shipment of goods)
    #[sea_orm(string_value = "pending-in")]
    PendingIn,
    /// For **proposed** outgoing transactions - all items within suborders of this transaction will be considered as subtractive towards inventory upon finalization (i.e. quote for sale of good to customer)
    #[sea_orm(string_value = "pending-out")]
    PendingOut
}