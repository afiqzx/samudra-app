//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use crate::CheckDuplicateTrait;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "lemma")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tarikh_masuk: DateTimeUtc,
    pub nama: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::konsep::Entity")]
    Konsep,
}

impl Related<super::konsep::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Konsep.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[async_trait]
impl CheckDuplicateTrait<Entity> for ActiveModel {
    async fn check<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        match self::Entity::find()
            .filter(Column::Nama.eq(self.nama.clone().into_value().unwrap()))
            .one(db)
            .await?
        {
            None => Ok(self),
            Some(model) => Ok(model.into()),
        }
    }
}
