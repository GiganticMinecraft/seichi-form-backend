use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    form::{
        answer::models::{AnswerId, AnswerLabel, FormAnswerContent},
        comment::models::CommentId,
        models::{Form, FormLabel},
    },
    user::models::User,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Comment {
    pub answer_id: AnswerId,
    pub id: CommentId,
    pub content: String,
    pub commented_by: Uuid,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct CrossSearchResult {
    pub forms: Vec<Form>,
    pub users: Vec<User>,
    pub answers: Vec<FormAnswerContent>,
    pub label_for_forms: Vec<FormLabel>,
    pub label_for_answers: Vec<AnswerLabel>,
    pub comments: Vec<Comment>,
}
