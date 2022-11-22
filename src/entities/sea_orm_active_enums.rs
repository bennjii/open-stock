//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use std::fmt::Display;

use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "transaction_type")]
pub enum TransactionType {
    #[sea_orm(string_value = "in")]
    In,
    #[sea_orm(string_value = "out")]
    Out,
}