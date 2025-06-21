use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
    pub email: String,
    pub message: String,
    
}