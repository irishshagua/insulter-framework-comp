use actix_web::{App, HttpRequest, HttpResponse, Result, http::Method, Responder, Error};
use crate::domain::{InsultTemplate, Rating};

impl Responder for InsultTemplate {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn ok(_req: &HttpRequest) -> impl Responder {
    InsultTemplate {
        content: "You {name}, are a muppet".to_string(),
        rating: Rating::Explicit
    }
}

fn created(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Created().header("Location", "/insults/1").finish())
}

fn updated(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::NoContent().finish())
}

fn insult(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!("Muppet")))
}

pub fn insult_mgmt() -> App {
    App::new()
        .prefix("/insults")
        .resource("", |r| {
            r.method(Method::GET).f(ok);
            r.method(Method::POST).f(created);
        })
        .resource("/{insult_id}", |r| {
            r.method(Method::GET).f(ok);
            r.method(Method::PUT).f(updated);
        })
}

pub fn insulter() -> App {
    App::new()
        .prefix("/insult")
        .resource("{name}", |r| {
            r.method(Method::GET).f(insult);
        })
}