use async_trait::async_trait;
use errors::Error;
use mockall::automock;

use crate::{
    form::{
        answer::models::AnswerId,
        comment::{
            models::{Comment, CommentId},
            service::CommentAuthorizationContext,
        },
    },
    types::authorization_guard_with_context::{
        AuthorizationGuardWithContext, Create, Delete, Read,
    },
    user::models::User,
};

#[automock]
#[async_trait]
pub trait CommentRepository: Send + Sync + 'static {
    async fn get_comments(
        &self,
        answer_id: AnswerId,
    ) -> Result<
        Vec<AuthorizationGuardWithContext<Comment, Read, CommentAuthorizationContext<Read>>>,
        Error,
    >;
    async fn get_comment(
        &self,
        comment_id: CommentId,
    ) -> Result<
        Option<AuthorizationGuardWithContext<Comment, Read, CommentAuthorizationContext<Read>>>,
        Error,
    >;
    async fn post_comment(
        &self,
        answer_id: AnswerId,
        context: &CommentAuthorizationContext<Read>,
        actor: &User,
        comment: AuthorizationGuardWithContext<Comment, Create, CommentAuthorizationContext<Read>>,
    ) -> Result<(), Error>;
    async fn delete_comment(
        &self,
        context: CommentAuthorizationContext<Read>,
        actor: &User,
        comment: AuthorizationGuardWithContext<Comment, Delete, CommentAuthorizationContext<Read>>,
    ) -> Result<(), Error>;
}