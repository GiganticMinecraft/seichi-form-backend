//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "form_choices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub question_id: i32,
    pub choice: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::form_questions::Entity",
        from = "Column::QuestionId",
        to = "super::form_questions::Column::QuestionId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    FormQuestions,
}

impl Related<super::form_questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormQuestions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
