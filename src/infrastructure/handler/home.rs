use actix_session::Session;
use actix_web::Responder;
use askama::Template;

#[derive(Template)]
#[template(path = "home/index.html")]
struct HandlerTemplate<'a> {
    name: &'a str,
    counter: i32,
}

pub async fn index(session: Session) -> impl Responder {
    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter").unwrap() {
        counter = count + 1;
        session.insert("counter", counter).unwrap();
    } else {
        session.insert("counter", counter).unwrap();
    }

    HandlerTemplate {
        name: "World",
        counter,
    }
}
