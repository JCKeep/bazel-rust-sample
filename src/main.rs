use std::time::Duration;

use bazel_rust_sample::add;
use serde::Serialize;
use tokio::time::sleep;

#[derive(Debug, Serialize)]
struct Foo {
    a: i32,
    b: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world({})!", add(1, 2));

    let foo = Foo { a: 1234, b: true };
    let json = serde_json::to_string_pretty(&foo)?;
    println!("{}", json);

    sleep(Duration::from_secs(2)).await;

    Ok(())
}
