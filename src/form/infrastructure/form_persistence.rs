use crate::database::connection::database_connection;
use crate::form::handlers::domain_for_user_input::raw_form::RawForm;
use crate::form::handlers::domain_for_user_input::raw_form_id::RawFormId;
use sea_orm::DatabaseBackend::MySql;
use sea_orm::{ConnectionTrait, DbErr, Statement, TransactionTrait};
use std::pin::Pin;

/// formを生成する
pub async fn create_form(form: RawForm) {
    let connection = database_connection().await;
    connection
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                txn.execute(Statement::from_sql_and_values(
                    MySql,
                    &*"INSERT INTO seichi_portal.forms (name) VALUES (?)".to_owned(),
                    vec![form.form_name().to_owned().into()],
                ))
            })
        })
        .await?

    // let transaction: Result<(), Error> = connection.transaction(|connection| {
    //     sql_query("INSERT INTO seichi_portal.forms (name) VALUES (?)")
    //         .bind::<VarChar, _>(form.form_name())
    //         .execute(connection)?;
    //
    //     let created_form_id =
    //         sql_query("SELECT LAST_INSERT_ID() AS id").get_result::<RawFormId>(connection)?;
    //
    //     // NOTE: ここのid埋め込みは自動生成の数字なのでそのまま埋め込んで良い
    //     sql_query(format!(
    //         r"CREATE TABLE forms.{} (
    //         id INT AUTO_INCREMENT,
    //         title VARCHAR(100) NOT NULL,
    //         description VARCHAR(300) NOT NULL,
    //         type VARCHAR(10) NOT NULL,
    //         choices TEXT,
    //         PRIMARY KEY(id)
    //     )
    //     ",
    //         created_form_id.id()
    //     ))
    //     .execute(connection)?;
    //
    //     let mut insert_query = form.questions().iter().map(|question| {
    //         let choices = question.choices().clone().map(|choices| choices.join(","));
    //         sql_query(format!(
    //             r"INSERT INTO forms.{} (title, description, type, choices)
    //             VALUES (?, ?, ?, ?)
    //         ",
    //             created_form_id.id()
    //         ))
    //         .bind::<VarChar, _>(question.title())
    //         .bind::<VarChar, _>(question.description())
    //         .bind::<VarChar, _>(question.question_type().to_string())
    //         .bind::<Nullable<Text>, _>(choices)
    //         .execute(connection)
    //         .is_ok()
    //     });
    //
    //     if insert_query.all(|query| query == true) {
    //         Ok(())
    //     } else {
    //         Err(Error::RollbackTransaction)
    //     }
    // });
    //
    // transaction.is_ok()
}

/// formを削除する
pub fn delete_form(_form_id: RawFormId) -> bool {
    todo!()
    // let connection = &mut database_connection();
    // let transaction: Result<(), Error> = connection.transaction(|connection| {
    //     sql_query("DELETE FROM seichi_portal.forms WHERE id = ?")
    //         .bind::<Integer, _>(_form_id.id())
    //         .execute(connection)?;
    //     sql_query(format!("DROP TABLE forms.{}", _form_id.id())).execute(connection)?;
    //
    //     Ok(())
    // });
    //
    // transaction.is_ok()
}
