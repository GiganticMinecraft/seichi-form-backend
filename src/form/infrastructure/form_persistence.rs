use crate::database::connection::database_connection;
use crate::form::handlers::domain_for_user_input::raw_form::RawForm;
use crate::form::handlers::domain_for_user_input::raw_form_id::RawFormId;
use diesel::sql_types::{Integer, VarChar};
use diesel::{sql_query, MysqlConnection, QueryResult, RunQueryDsl};

/// formを生成する
pub fn create_form(_form: RawForm) -> QueryResult<usize> {
    let connection: &mut MysqlConnection = &mut database_connection();
    let is_form_inserted = sql_query("INSERT INTO seichi_portal.forms (name) VALUES (?)")
        .bind::<VarChar, _>(_form.form_name())
        .execute(connection)
        .is_ok();

    let created_form_id = sql_query("SELECT id FROM seichi_portal.forms WHERE name = ?")
        .bind::<VarChar, _>(_form.form_name())
        .get_result::<RawFormId>(connection)
        .unwrap();

    // NOTE: ここのid埋め込みは自動生成の数字なのでそのまま埋め込んで良い
    sql_query(format!(
        r"CREATE TABLE forms.{} (
            id INT AUTO_INCREMENT,
            title VARCHAR(100),
            description VARCHAR(300),
            type VARCHAR(10),
            choices TEXT,
            PRIMARY KEY(id)
        )
        ",
        created_form_id.id()
    ))
    .execute(connection)
}

/// formを削除する
pub fn delete_form(_form_id: RawFormId) -> QueryResult<usize> {
    let mut connection = database_connection();
    sql_query("DELETE FROM seichi_portal.forms WHERE id = ?")
        .bind::<Integer, _>(_form_id.id())
        .execute(&mut connection)
}
