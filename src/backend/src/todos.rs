use candid::CandidType;
use ic_rusqlite::with_connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct Todo {
    pub id: u32,
    pub person_id: u32,
    pub text: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct NewTodo {
    pub person_id: u32,
    pub text: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct UpdateTodo {
    pub id: u32,
    pub text: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct SelectTodo {
    pub person_id: u32,
}

fn validate_todo(todo: &str) -> Result<(), String> {
    if todo.trim().is_empty() {
        return Err("To-do cannot be empty!".to_string());
    }

    Ok(())
}

pub(crate) fn insert(todo: NewTodo) -> Result<Todo, String> {
    validate_todo(&todo.text)?;

    with_connection(|conn| {
        let sql = r#"
            INSERT INTO todos (person_id, text, done)
            VALUES (?1, ?2, ?3)
            RETURNING
              id,
              person_id,
              text,
              done
        "#;

        conn.query_row(sql, (todo.person_id, todo.text, todo.done), |row| {
            Ok(Todo {
                id: row.get(0)?,
                person_id: row.get(1)?,
                text: row.get(2)?,
                done: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())
    })
}

pub(crate) fn update(todo: UpdateTodo) -> Result<Todo, String> {
    validate_todo(&todo.text)?;

    with_connection(|conn| {
        let sql = r#"
            UPDATE todos
            SET text = ?2, done = ?3
            WHERE id = ?1
            RETURNING
              id,
              person_id,
              text,
              done
        "#;

        conn.query_row(sql, (todo.id, todo.text, todo.done), |row| {
            Ok(Todo {
                id: row.get(0)?,
                person_id: row.get(1)?,
                text: row.get(2)?,
                done: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())
    })
}

pub(crate) fn delete(id: u32) -> Result<Todo, String> {
    with_connection(|conn| {
        let sql = r#"
                DELETE FROM todos
                WHERE id = ?1
                RETURNING
                id,
                person_id,
                text,
                done
            "#;

        conn.query_row(sql, (id,), |row| {
            Ok(Todo {
                id: row.get(0)?,
                person_id: row.get(1)?,
                text: row.get(2)?,
                done: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())
    })
}

pub(crate) fn select(params: SelectTodo) -> Result<Vec<Todo>, String> {
    with_connection(|conn| {
        let sql = r#"
            SELECT
              id,
              person_id,
              text,
              done
            FROM todos
            WHERE person_id = ?1
        "#;

        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map((params.person_id,), |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    person_id: row.get(1)?,
                    text: row.get(2)?,
                    done: row.get(3)?,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<ic_rusqlite::Result<Vec<_>>>()
            .map_err(|e| e.to_string())
    })
}
