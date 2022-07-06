use actix_web::Responder;
use askama::Template;

#[derive(Template)]
#[template(path = "home/index.html")]
struct HandlerTemplate<'a> {
    name: &'a str,
}

pub async fn index() -> impl Responder {
    HandlerTemplate { name: "World" }
}
