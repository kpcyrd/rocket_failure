# rocket_failure

Semantic error handling for rocket applications.

To enable this crate in your server add this line to your `Cargo.toml`:
```toml
rocket_failure = { version="0.1", features = ["with-rocket"] }
```

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_failure;

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
```

You can run this example with:
```
cargo +nightly run --example fileserver --features=with-rocket
```

With you want to use the `ApiResult<T>` type in your api client to consume the
api, omit the `with-rocket` feature:

```toml
rocket_failure = "0.1"
```

## License

rocket_failure is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
