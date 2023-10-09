//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(Some(16)))")]
    pub uuid: Vec<u8>,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::answers::Entity")]
    Answers,
}

impl Related<super::answers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Answers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
