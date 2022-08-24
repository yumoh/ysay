mod cli;
mod error;
mod play;
mod tts;

use structopt::StructOpt;

fn main() {
    env_logger::init();

    let cli_args = cli::SayCli::from_args();
    let result = cli::handle_say(&cli_args);
    if let Err(err) = result {
        log::error!("{:?}", err);
    }
}
