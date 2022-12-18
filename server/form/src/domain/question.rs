use crate::domain::QuestionType;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Question {
    title: String,
    description: String,
    question_type: QuestionType,
    choices: Option<Vec<String>>,
}