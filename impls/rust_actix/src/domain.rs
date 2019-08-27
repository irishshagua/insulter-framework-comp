#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    Explicit,
    Childish,
}

#[derive(Serialize, Deserialize)]
pub struct InsultTemplate {
    pub content: String,
    pub rating: Rating
}
