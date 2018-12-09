#[macro_use]
extern crate serde_derive;

use std::fmt;

#[cfg(feature = "with-rocket")]
pub mod errors;
#[cfg(feature = "with-rocket")]
pub mod macros;

#[derive(Debug, Serialize, Deserialize)]
pub enum StrictApiResponse<T> {
    #[serde(rename = "success")]
    Success(T),
    #[serde(rename = "error")]
    Error(String),
}

impl<T> StrictApiResponse<T> {
    pub fn success(self) -> Result<T, ApiError> {
        match self {
            StrictApiResponse::Success(x) => Ok(x),
            StrictApiResponse::Error(err) => Err(ApiError(err)),
        }
    }
}

#[derive(Debug)]
pub struct ApiError(String);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "api error: {:?}", self.0)
    }
}

impl std::error::Error for ApiError {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
