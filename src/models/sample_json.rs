use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    name: String,
    email: String,
    message: String,
    
}