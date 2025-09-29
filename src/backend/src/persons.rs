use candid::CandidType;
use ic_rusqlite::with_connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub occupation: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct NewPerson {
    pub name: String,
    pub occupation: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct UpdatePerson {
    pub id: u64,
    pub name: String,
    pub occupation: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct QueryPersons {
    pub limit: u32,
    pub offset: u32,
}

fn validate_email(email: &str) -> Result<(), String> {
    if email.trim().is_empty() {
        return Err("E-mail cannot be empty!".to_string());
    }

    if !email_address::EmailAddress::is_valid(email) {
        return Err("E-mail address is not valid!".to_string());
    }

    Ok(())
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Name cannot be empty!".to_string());
    }

    Ok(())
}

fn validate_occupation(occupation: &str) -> Result<(), String> {
    if occupation.trim().is_empty() {
        return Err("Occupation cannot be empty!".to_string());
    }

    Ok(())
}

pub(crate) fn insert(person: NewPerson) -> Result<Person, String> {
    validate_name(&person.name)?;
    validate_occupation(&person.occupation)?;
    validate_email(&person.email)?;

    with_connection(|conn| {
        let sql = r#"
            INSERT INTO persons (name, occupation, email)
            VALUES (?1, ?2, ?3)
            RETURNING
              id,
              name,
              occupation,
              email
        "#;

        conn.query_row(sql, (person.name, person.occupation, person.email), |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                occupation: row.get(2)?,
                email: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())
    })
}

pub(crate) fn update(person: UpdatePerson) -> Result<Person, String> {
    validate_name(&person.name)?;
    validate_occupation(&person.occupation)?;
    validate_email(&person.email)?;

    with_connection(|conn| {
        let sql = r#"
            UPDATE persons
            SET name = ?2, occupation = ?3, email = ?4
            WHERE id = ?1
            RETURNING
              id,
              name,
              occupation,
              email
        "#;

        conn.query_row(
            sql,
            (person.id, person.name, person.occupation, person.email),
            |row| {
                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    occupation: row.get(2)?,
                    email: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())
    })
}

pub(crate) fn delete(id: u64) -> Result<Person, String> {
    with_connection(|conn| {
        let sql = r#"
                DELETE FROM persons
                WHERE id = ?1
                RETURNING id, name, occupation, email
            "#;

        conn.query_row(sql, (id,), |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                occupation: row.get(2)?,
                email: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())
    })
}

pub(crate) fn get(id: u64) -> Result<Person, String> {
    with_connection(|conn| {
        let sql = r#"
                SELECT
                id,
                name,
                occupation,
                email
                FROM persons
                WHERE id = ?1
            "#;

        conn.query_row(sql, (id,), |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                occupation: row.get(2)?,
                email: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())
    })
}

pub(crate) fn select(params: QueryPersons) -> Result<Vec<Person>, String> {
    with_connection(|conn| {
        let sql = r#"
            SELECT
              id,
              name,
              occupation,
              email
            FROM persons
            LIMIT ?1 OFFSET ?2
        "#;

        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map((params.limit as u64, params.offset as u64), |row| {
                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    occupation: row.get(2)?,
                    email: row.get(3)?,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<ic_rusqlite::Result<Vec<_>>>()
            .map_err(|e| e.to_string())
    })
}
