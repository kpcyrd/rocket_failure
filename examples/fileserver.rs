#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_failure;

use rocket_failure::errors::*;
use std::fs;

#[get("/<file>")]
fn index(file: String) -> ApiResult<Vec<u8>> {
    if !file.chars().all(|c| char::is_alphanumeric(c) || c == '-' || c == '.') {
        bad_request!("file contains forbidden characters")
    }

    // if this returns an Err(_), return a standard 404
    let content = fs::read(&file)
        .not_found()?;

    // detailed errors are hidden by default
    // we can publish the actual error if we want to
    /*
    let content = fs::read(&file)
        .not_found()
        .publish_error()?;
    */

    // or we can set a public error while preserving the actual error
    /*
    let content = fs::read(&file)
        .not_found()
        .public_context("That didn't work")?;
    */

    Ok(content)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
