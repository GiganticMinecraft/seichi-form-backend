//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "form_questions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub question_id: i32,
    pub form_id: i32,
    pub title: String,
    pub description: String,
    pub answer_type: String,
    pub choices: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::answer_types::Entity",
        from = "Column::AnswerType",
        to = "super::answer_types::Column::AnswerType",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AnswerTypes,
    #[sea_orm(
        belongs_to = "super::forms::Entity",
        from = "Column::FormId",
        to = "super::forms::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Forms,
}

impl Related<super::answer_types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnswerTypes.def()
    }
}

impl Related<super::forms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Forms.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
