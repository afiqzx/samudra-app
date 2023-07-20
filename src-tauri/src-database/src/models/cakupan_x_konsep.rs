//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cakupan_x_konsep")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub konsep_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub cakupan_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cakupan::Entity",
        from = "Column::CakupanId",
        to = "super::cakupan::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Cakupan,
    #[sea_orm(
        belongs_to = "super::konsep::Entity",
        from = "Column::KonsepId",
        to = "super::konsep::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Konsep,
}

impl Related<super::cakupan::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cakupan.def()
    }
}

impl Related<super::konsep::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Konsep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}