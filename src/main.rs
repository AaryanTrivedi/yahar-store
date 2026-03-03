mod server;
mod store;

use server::run;

fn main() -> std::io::Result<()> {
    run("0.0.0.0:9162")
}
