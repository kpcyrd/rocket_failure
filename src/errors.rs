use rocket::http::ContentType;
pub use rocket::http::Status;
use rocket::response::Responder;
use rocket::Request;
use serde_json;
use std::fmt::Display;
use std::io::Cursor;

pub trait ApiResultExt<T, E> {
    fn private_context<D>(self, context: D) -> Result<T, WebError>
    where
        D: Display + Send + Sync + 'static,
        Self: Sized;

    fn public_context<D>(self, context: D) -> Result<T, WebError>
    where
        D: Display + Send + Sync + 'static,
        Self: Sized;

    fn publish_error(self) -> Result<T, WebError>
    where
        Self: Sized;

    fn bad_request(self) -> Result<T, WebError>
    where
        Self: Sized;

    fn forbidden(self) -> Result<T, WebError>
    where
        Self: Sized;

    fn not_found(self) -> Result<T, WebError>
    where
        Self: Sized;
}

impl<T, E> ApiResultExt<T, E> for Result<T, E>
where
    E: WebFail,
{
    fn private_context<D>(self, context: D) -> Result<T, WebError>
    where
        D: Display + Send + Sync + 'static,
        Self: Sized,
    {
        self.map_err(|err| err.fail().with_private_error(context))
    }

    fn public_context<D>(self, context: D) -> Result<T, WebError>
    where
        D: Display + Send + Sync + 'static,
        Self: Sized,
    {
        self.map_err(|err| err.fail().with_public_error(context))
    }

    fn publish_error(self) -> Result<T, WebError>
    where
        Self: Sized,
    {
        self.map_err(|err| err.fail().publish_error())
    }

    fn bad_request(self) -> Result<T, WebError>
    where
        Self: Sized,
    {
        // 400
        self.map_err(|err| err.fail().with_status(Status::BadRequest))
    }

    fn forbidden(self) -> Result<T, WebError>
    where
        Self: Sized,
    {
        // 403
        self.map_err(|err| err.fail().with_status(Status::Forbidden))
    }

    fn not_found(self) -> Result<T, WebError>
    where
        Self: Sized,
    {
        // 404
        self.map_err(|err| err.fail().with_status(Status::NotFound))
    }
}

pub type ApiResult<T> = ::std::result::Result<T, WebError>;

#[derive(Debug)]
pub enum ApiResponse<T> {
    Success(T),
    Error(WebError),
}

impl<T> ApiResponse<T> {
    pub fn into_strict(self) -> (StrictApiResponse<T>, ResponseHints) {
        match self {
            ApiResponse::Success(x) => (StrictApiResponse::Success(x), ResponseHints::ok()),
            ApiResponse::Error(x) => {
                let msg = x.public_error();
                let hints = ResponseHints::err(&x);
                (StrictApiResponse::Error(msg), hints)
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StrictApiResponse<T> {
    #[serde(rename = "success")]
    Success(T),
    #[serde(rename = "error")]
    Error(String),
}

pub struct ResponseHints {
    status: Status,
}

impl ResponseHints {
    pub fn ok() -> ResponseHints {
        ResponseHints {
            status: Status::Ok,
        }
    }

    pub fn err(err: &WebError) -> ResponseHints {
        let status = err.status.unwrap_or(Status::InternalServerError);

        ResponseHints {
            status,
        }
    }
}

impl<'r, T> Responder<'r> for ApiResponse<T>
where
    T: serde::Serialize,
{
    fn respond_to(self, _req: &Request) -> ::std::result::Result<rocket::Response<'r>, Status> {
        let (response, hints) = self.into_strict();
        let body = serde_json::to_string(&response).map_err(|_| Status::InternalServerError)?;

        rocket::Response::build()
            .status(hints.status)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(body))
            .ok()
    }
}

#[derive(Debug)]
pub struct WebError {
    status: Option<Status>,
    private_error: String,
    public_error: Option<String>,
}

impl WebError {
    pub fn new<D: Display>(error: D) -> WebError {
        WebError {
            status: None,
            private_error: error.to_string(),
            public_error: None,
        }
    }

    pub fn with_status(mut self, status: Status) -> WebError {
        self.status = Some(status);
        self
    }

    pub fn with_private_error<D: Display>(mut self, error: D) -> WebError {
        self.private_error = error.to_string();
        self
    }

    pub fn with_public_error<D: Display>(mut self, error: D) -> WebError {
        self.public_error = Some(error.to_string());
        self
    }

    pub fn publish_error(mut self) -> WebError {
        self.public_error = Some(self.private_error.clone());
        self
    }

    pub fn public_error(&self) -> String {
        self.public_error.as_ref()
            .map(|x| x.to_string())
            .unwrap_or_else(|| match self.status {
                Some(status) => status.reason,
                _ => Status::InternalServerError.reason,
            }.to_string())
    }
}

impl<E: Display + Send + Sync + 'static> From<E> for WebError {
    fn from(e: E) -> WebError {
        WebError::new(e)
    }
}

impl<'r> Responder<'r> for WebError {
    fn respond_to(self, _req: &Request) -> ::std::result::Result<rocket::Response<'r>, Status> {
        ApiResponse::Error::<()>(self).respond_to(_req)
    }
}

pub trait WebFail: Send + Sync + 'static {
    fn fail(self) -> WebError
    where
        Self: Sized;
}

impl<E: Display + Send + Sync + 'static> WebFail for E {
    fn fail(self) -> WebError
    where
        Self: Sized,
    {
        WebError::new(self)
    }
}

impl WebFail for WebError {
    fn fail(self) -> WebError
    where
        Self: Sized,
    {
        self
    }
}

pub fn err_msg<D: Display + Sync + Send + 'static>(status: Status, msg: D) -> WebError {
    WebError {
        status: Some(status),
        private_error: msg.to_string(),
        public_error: Some(msg.to_string()),
    }
}
