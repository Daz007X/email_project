use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}