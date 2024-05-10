use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn hello() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/hello").dispatch();
    assert_eq!(response.into_string(), Some("\"Hello world\"".into()));
}

#[test]
fn index() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn roles() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/roles").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn users() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn settings() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/settings").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn permissions() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/permissions").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
