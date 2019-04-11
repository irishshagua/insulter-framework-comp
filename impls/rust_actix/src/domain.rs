#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    Explicit,
    Childish,
}

#[derive(Serialize)]
pub struct InsultTemplate {
    pub content: String,
    pub rating: Rating
}
