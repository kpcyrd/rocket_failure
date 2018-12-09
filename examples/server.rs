#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate failure;

use rocket_failure::errors::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/internal/<x>/<y>")]
fn internal(x: String, y: String) -> ApiResult<ApiResponse<String>> {
    let result = if x == y {
        Ok(x)
    } else {
        Err(format_err!("this error is sensitive"))
    };

    let x = result?;

    Ok(ApiResponse::Success(x))
}

#[get("/404/<x>/<y>")]
fn notfound(x: String, y: String) -> ApiResult<ApiResponse<String>> {
    let result = if x == y {
        Ok(x)
    } else {
        Err(format_err!("this error is sensitive"))
    };

    let x = result
        .not_found()?;

    Ok(ApiResponse::Success(x))
}

#[get("/with-msg/internal/<x>/<y>")]
fn internal_with_msg(x: String, y: String) -> ApiResult<ApiResponse<String>> {
    let result = if x == y {
        Ok(x)
    } else {
        Err(format_err!("this error is sensitive"))
    };

    let x = result
        .public_context("hello")?;

    Ok(ApiResponse::Success(x))
}

#[get("/with-msg/404/<x>/<y>")]
fn notfound_with_msg(x: String, y: String) -> ApiResult<ApiResponse<String>> {
    let result = if x == y {
        Ok(x)
    } else {
        Err(format_err!("this error is sensitive"))
    };

    let x = result
        .not_found()
        .public_context("hello")?;

    Ok(ApiResponse::Success(x))
}

#[get("/public-err/internal/<x>/<y>")]
fn internal_public_err(x: String, y: String) -> ApiResult<ApiResponse<String>> {
    let result = if x == y {
        Ok(x)
    } else {
        Err(format_err!("this error is ok to leak"))
    };

    let x = result
        .publish_error()?;

    Ok(ApiResponse::Success(x))
}

#[get("/public-err/404/<x>/<y>")]
fn notfound_public_err(x: String, y: String) -> ApiResult<ApiResponse<String>> {
    let result = if x == y {
        Ok(x)
    } else {
        Err(format_err!("this error is ok to leak"))
    };

    let x = result
        .not_found()
        .publish_error()?;

    Ok(ApiResponse::Success(x))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            internal,
            notfound,
            internal_with_msg,
            notfound_with_msg,
            internal_public_err,
            notfound_public_err,
        ])
        .launch();
}
