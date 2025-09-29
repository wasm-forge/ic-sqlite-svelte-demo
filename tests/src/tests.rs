use ic_test::IcpTest;

use crate::{
    bindings::backend::{NewPerson, Person, ResultVecPerson},
    test_setup,
};

#[tokio::test]
async fn test_add_person() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let person = env
        .backend
        .new_person(NewPerson {
            occupation: "Occupation".to_string(),
            name: "Some Name".to_string(),
            email: "person@sample.com".to_string(),
        })
        .call()
        .await;

    let crate::bindings::backend::ResultPerson::Ok(person) = person else {
        panic!("Failed new_person call!");
    };

    let persons: ResultVecPerson = env
        .backend
        .list_persons(crate::bindings::backend::QueryPersons {
            offset: 0,
            limit: 10,
        })
        .call()
        .await;

    match persons {
        ResultVecPerson::Ok(persons) => {
            assert!(persons.contains(&person), "Inserted person was not found!");
        }
        ResultVecPerson::Err(_) => {
            panic!("Failed call!");
        }
    }
}

#[tokio::test]
async fn test_wrong_email() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let person = env
        .backend
        .new_person(NewPerson {
            occupation: "Occupation".to_string(),
            name: "Some Name".to_string(),
            email: "person@samp@le.com".to_string(),
        })
        .call()
        .await;

    match person {
        crate::bindings::backend::ResultPerson::Ok(_) => {
            panic!("Expected error on wrong email format!");
        }
        crate::bindings::backend::ResultPerson::Err(e) => {
            assert_eq!(e, "E-mail address is not valid!", "received error {e}");
        }
    }
}

#[tokio::test]
async fn test_empty_name_rejected() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let person = env
        .backend
        .new_person(NewPerson {
            occupation: "Occupation".to_string(),
            name: "".to_string(),
            email: "person@sample.com".to_string(),
        })
        .call()
        .await;

    match person {
        crate::bindings::backend::ResultPerson::Ok(_) => {
            panic!("Expected error on wrong email format!");
        }
        crate::bindings::backend::ResultPerson::Err(e) => {
            assert_eq!(e, "Name cannot be empty!", "received error {e}");
        }
    }
}

#[tokio::test]
async fn test_person_update_works() {
    let env = test_setup::setup(IcpTest::new().await).await;

    let person = env
        .backend
        .new_person(NewPerson {
            occupation: "Occupation".to_string(),
            name: "Some name".to_string(),
            email: "person@sample.com".to_string(),
        })
        .call()
        .await;

    let crate::bindings::backend::ResultPerson::Ok(person) = person else {
        panic!("Failed new_person call!");
    };

    env.backend
        .update_person(Person {
            name: "Some other name".to_string(),
            ..person
        })
        .call()
        .await;

    let upd_person = env.backend.get_person(person.id).call().await;

    let crate::bindings::backend::ResultPerson::Ok(upd_person) = upd_person else {
        panic!("Failed get_person call!");
    };

    assert_eq!(person.id, upd_person.id);
    assert_eq!(upd_person.name, "Some other name");
}

#[tokio::test]
async fn test_add_and_delete_person() {
    let env = test_setup::setup(IcpTest::new().await).await;

    // add person
    let person = env
        .backend
        .new_person(NewPerson {
            occupation: "Occupation".to_string(),
            name: "Some Name".to_string(),
            email: "person@sample.com".to_string(),
        })
        .call()
        .await;

    let crate::bindings::backend::ResultPerson::Ok(person) = person else {
        panic!("Failed new_person call!");
    };

    // check the person exists in the database
    let persons: ResultVecPerson = env
        .backend
        .list_persons(crate::bindings::backend::QueryPersons {
            offset: 0,
            limit: 10,
        })
        .call()
        .await;

    match persons {
        ResultVecPerson::Ok(persons) => {
            assert!(persons.contains(&person), "Inserted person was not found!");
        }
        ResultVecPerson::Err(_) => {
            panic!("Failed call!");
        }
    }

    // delete the same person
    let _ = env.backend.delete_person(person.id).call().await;

    // check the person does not exist in the database
    let persons: ResultVecPerson = env
        .backend
        .list_persons(crate::bindings::backend::QueryPersons {
            offset: 0,
            limit: 10,
        })
        .call()
        .await;

    match persons {
        ResultVecPerson::Ok(persons) => {
            assert!(!persons.contains(&person), "Deleted person was found!");
        }
        ResultVecPerson::Err(_) => {
            panic!("Failed call!");
        }
    }
}
