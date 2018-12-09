#[macro_export]
macro_rules! bad_request {
    ($e:expr) => {
        return Err($crate::errors::err_msg($crate::errors::Status::BadRequest, $e));
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err($crate::errors::err_msg($crate::errors::Status::BadRequest, format!($fmt, $($arg)+)));
    };
}

#[macro_export]
macro_rules! not_found {
    ($e:expr) => {
        return Err($crate::errors::err_msg($crate::errors::Status::NotFound, $e));
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err($crate::errors::err_msg($crate::errors::Status::NotFound, format!($fmt, $($arg)+)));
    };
}
