use async_trait::async_trait;
use errors::Error;
use mockall::automock;

use crate::{
    form::models::{FormId, FormLabel, FormLabelId},
    types::{
        authorization_guard::AuthorizationGuard,
        authorization_guard_with_context::{Create, Delete, Read, Update},
    },
    user::models::User,
};

#[automock]
#[async_trait]
pub trait FormLabelRepository: Send + Sync + 'static {
    async fn create_label_for_forms(
        &self,
        label: AuthorizationGuard<FormLabel, Create>,
        actor: &User,
    ) -> Result<(), Error>;
    async fn fetch_labels(&self) -> Result<Vec<AuthorizationGuard<FormLabel, Read>>, Error>;
    async fn fetch_label(
        &self,
        id: FormLabelId,
    ) -> Result<Option<AuthorizationGuard<FormLabel, Read>>, Error>;
    async fn delete_label_for_forms(
        &self,
        label: AuthorizationGuard<FormLabel, Delete>,
        actor: &User,
    ) -> Result<(), Error>;
    async fn edit_label_for_forms(
        &self,
        id: FormLabelId,
        label: AuthorizationGuard<FormLabel, Update>,
        actor: &User,
    ) -> Result<(), Error>;
    async fn fetch_labels_by_form_id(
        &self,
        form_id: FormId,
    ) -> Result<Vec<AuthorizationGuard<FormLabel, Read>>, Error>;
    async fn replace_form_labels(
        &self,
        form_id: FormId,
        label_ids: Vec<FormLabelId>,
    ) -> Result<(), Error>;
}
