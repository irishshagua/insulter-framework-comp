use crate::domain::{InsultTemplate, Rating};
use actix_web::web::{Json, Path};
use actix_web::{get, post, Responder};

#[get("/insults")]
pub fn retrieve_all_insults() -> impl Responder {
    Json(
        InsultTemplate {
            content: "You {name}, are a muppet".to_string(),
            rating: Rating::Explicit,
        }
    )
}

#[post("/insults")]
pub fn create_new_insult(incoming: Json<InsultTemplate>) -> impl Responder {
    format!("Created new insult: {}", incoming.content)
}

#[get("/insults/{id}")]
pub fn get_specific_insult(id: String) -> impl Responder {
    Json(
        InsultTemplate {
            content: id,
            rating: Rating::Explicit,
        }
    )
}

#[get("/insult/{data}")]
pub fn apply_random_insult_to_name(francis: Path<String>) -> impl Responder {
    format!("Fuck You {}!", francis)
}