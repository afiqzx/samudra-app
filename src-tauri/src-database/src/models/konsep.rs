//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "konsep")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tarikh_masuk: DateTimeUtc,
    pub lemma_id: i32,
    pub golongan_id: Option<String>,
    pub keterangan: Option<String>,
    pub tertib: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::golongan_kata::Entity",
        from = "Column::GolonganId",
        to = "super::golongan_kata::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    GolonganKata,
    #[sea_orm(
        belongs_to = "super::lemma::Entity",
        from = "Column::LemmaId",
        to = "super::lemma::Column::Id",
        on_update = "NoAction",
        on_delete = "SetDefault"
    )]
    Lemma,
}

impl Related<super::golongan_kata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GolonganKata.def()
    }
}

impl Related<super::lemma::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lemma.def()
    }
}

impl Related<super::cakupan::Entity> for Entity {
    fn to() -> RelationDef {
        super::cakupan_x_konsep::Relation::Cakupan.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::cakupan_x_konsep::Relation::Konsep.def().rev())
    }
}

impl Related<super::kata_asing::Entity> for Entity {
    fn to() -> RelationDef {
        super::kata_asing_x_konsep::Relation::KataAsing.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::kata_asing_x_konsep::Relation::Konsep.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}