mod cli;
mod config;
mod utils;

fn main() -> Result<(), utils::BoxError> {
    let _cfgpath = match config::create_config(None) {
        Ok((created, path)) => {
            if created {
                println!("created config file at {:?}", path);
            }
            path
        }
        Err(err) => {
            return Err(err);
        }
    };
    cli::parse_cli();
    Ok(())
}
