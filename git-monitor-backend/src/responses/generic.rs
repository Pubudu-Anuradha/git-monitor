use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub success: bool,
    pub message: String,
}
