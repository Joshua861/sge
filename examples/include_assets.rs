use sge::prelude::*;

// can parse JSON, TOML, and Ron
include_assets!("assets/data", ASSETS);

fn main() {
    println!("{}", ASSETS.test.messages.hello);
}
