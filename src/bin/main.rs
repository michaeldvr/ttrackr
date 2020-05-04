use ttrackr::*;

use dotenv::dotenv;
use log::debug;

fn main() -> Result<(), utils::BoxError> {
    dotenv().ok();
    env_logger::init();
    debug!("test");
    cli::parse_cli()
}
