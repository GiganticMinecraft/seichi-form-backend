//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "form_meta_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::default_answer_titles::Entity")]
    DefaultAnswerTitles,
    #[sea_orm(has_many = "super::form_questions::Entity")]
    FormQuestions,
    #[sea_orm(has_many = "super::form_webhooks::Entity")]
    FormWebhooks,
    #[sea_orm(has_many = "super::response_period::Entity")]
    ResponsePeriod,
}

impl Related<super::default_answer_titles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DefaultAnswerTitles.def()
    }
}

impl Related<super::form_questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormQuestions.def()
    }
}

impl Related<super::form_webhooks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormWebhooks.def()
    }
}

impl Related<super::response_period::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResponsePeriod.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
