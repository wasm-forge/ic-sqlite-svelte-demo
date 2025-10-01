use ic_cdk::export_candid;
use ic_rusqlite::with_connection;

use crate::{
    persons::{NewPerson, Person, QueryPersons, UpdatePerson},
    todos::{NewTodo, SelectTodo, Todo, UpdateTodo},
};

pub mod persons;
pub mod todos;

fn create_db() {
    with_connection(|conn| {
        conn.execute(
            "
            DROP TABLE IF EXISTS todos;",
            (),
        )
        .expect("Failed to prepare demo tables");

        conn.execute(
            "
            DROP TABLE IF EXISTS persons;",
            (),
        )
        .expect("Failed to prepare demo tables");

        // Email validation at the SQL layer
        // conn.execute(
        //     "
        //     CREATE TABLE IF NOT EXISTS persons (
        //     id INTEGER PRIMARY KEY AUTOINCREMENT,
        //     name TEXT,
        //     email TEXT,
        //     occupation TEXT,
        //     CHECK (
        //         LENGTH(email) - LENGTH(REPLACE(email, '@', '')) = 1
        //         AND
        //         email LIKE '%_@__%.__%'
        //         AND
        //         LOWER(email) GLOB '[a-z0-9@._-]*'
        //     ));",
        //     (),
        // )
        // .expect("Failed to create demo table");

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS persons (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            email TEXT, 
            occupation TEXT);",
            (),
        )
        .expect("Failed to create demo table");

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS todos (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            person_id INTEGER NOT NULL,
            text      TEXT NOT NULL,
            done      BOOLEAN DEFAULT 0,
            FOREIGN KEY (person_id)
                REFERENCES persons(id)
                ON DELETE CASCADE
            );",
            (),
        )
        .expect("Failed to create demo table");
    });
}

#[ic_cdk::update]
pub fn add_demo_data() {
    let persons = [
        (
            "Olivia Bennett",
            "Creative Artist",
            "olivia.bennett92@example.com",
            vec![
                "Brush teeth, but pretend you’re painting a masterpiece 🪥",
                "Make coffee look Instagram-worthy ☕",
                "Sketch a random object nearby ✏️",
                "Dance break to your favorite song 💃",
            ],
        ),
        (
            "Marcus Reynolds",
            "Busy Coder",
            "marcus.reynolds87@example.com",
            vec![
                "Boot up brain with caffeine ☕",
                "Brush teeth while debugging life 🪥",
                "Write “Hello World” in at least 3 languages 💻",
                "Take a stretch break (pretend you’re updating firmware) 🤖",
                "Rename your WiFi to something funny 📶",
                "High-five your keyboard ⌨️",
            ],
        ),
        (
            "Priya Sharma",
            "Fitness Fan",
            "priya.sharma21@example.com",
            vec![
                "Jump out of bed like it’s a burpee 🛏️➡️💪",
                "Brush teeth while doing squats 🪥🏋️",
                "Run to the fridge (for water, not snacks!) 🏃‍♂️",
                "Flex in the mirror and wink at yourself 💪😉",
                "Do 5 pushups before opening your phone 📱",
                "End the day with a victory dance 🕺",
            ],
        ),
        (
            "Ethan Caldwell",
            "Bookworm",
            "e.caldwell56@example.com",
            vec![
                "Wake up and read one random page 📖",
                "Brush teeth while reciting poetry 🪥🎭",
                "Smell a book like it’s fine wine 🍷📚",
                "Write down a word you love ✍️",
                "Pretend your pet/plant is listening to your audiobook 🐱🌱",
                "Fall asleep hugging a novel 💕",
            ],
        ),
    ];

    for (pname, poccupation, pemail, todos) in persons {
        let person = persons::insert(NewPerson {
            name: pname.to_string(),
            email: pemail.to_string(),
            occupation: poccupation.to_string(),
        })
        .expect("Failed to insert demo data");

        for todo in todos {
            todos::insert(NewTodo {
                person_id: person.id,
                text: todo.to_string(),
                done: todo.len() % 2 == 1,
            })
            .expect("Failed to insert demo data");
        }
    }
}

#[ic_cdk::init]
fn init() {
    create_db();
    add_demo_data();
}

#[ic_cdk::update]
fn reset_base() {
    create_db();
}

#[ic_cdk::update]
fn new_person(person: NewPerson) -> Result<Person, String> {
    persons::insert(person)
}

#[ic_cdk::update]
fn update_person(person: UpdatePerson) -> Result<Person, String> {
    persons::update(person)
}

#[ic_cdk::update]
fn delete_person(id: u32) -> Result<Person, String> {
    persons::delete(id)
}

#[ic_cdk::query]
fn get_person(id: u32) -> Result<Person, String> {
    persons::get(id)
}

#[ic_cdk::query]
fn list_persons(param: QueryPersons) -> Result<Vec<Person>, String> {
    persons::select(param)
}

#[ic_cdk::update]
fn dummy_update() {
    //
}

#[ic_cdk::query]
fn get_persons() -> Result<Vec<Person>, String> {
    persons::select(QueryPersons {
        limit: 1000,
        offset: 0,
    })
}

#[ic_cdk::update]
fn new_todo(todo: NewTodo) -> Result<Todo, String> {
    todos::insert(todo)
}

#[ic_cdk::update]
fn update_todo(todo: UpdateTodo) -> Result<Todo, String> {
    todos::update(todo)
}

#[ic_cdk::update]
fn delete_todo(id: u32) -> Result<Todo, String> {
    todos::delete(id)
}

#[ic_cdk::query]
fn list_todos(param: SelectTodo) -> Result<Vec<Todo>, String> {
    todos::select(param)
}

export_candid!();
