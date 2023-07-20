//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "kata_asing_x_konsep")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub konsep_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub kata_asing_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::kata_asing::Entity",
        from = "Column::KataAsingId",
        to = "super::kata_asing::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    KataAsing,
    #[sea_orm(
        belongs_to = "super::konsep::Entity",
        from = "Column::KonsepId",
        to = "super::konsep::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Konsep,
}

impl Related<super::kata_asing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::KataAsing.def()
    }
}

impl Related<super::konsep::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Konsep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}