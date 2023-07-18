use crate::http::prelude::HttpError;

#[derive(Debug)]
pub enum HarmonyError {
    HttpError(HttpError),
    
}