use clap::{Parser};
use utils::cli::{Cli, Commands};
use utils::config::Config;

mod day1;

fn main() {
    let config = Config::from_env();
    let args = Cli::parse();

    match args.command {
        Commands::Run(wrapper) => {
            let data = wrapper
                .data
                .load_to_string(2023, wrapper.day, &config.session_cookie);
            match wrapper.day {
                1 => day1::main(data),
                _ => panic!("Day not implemented"),
            }
        }
        Commands::Download(wrapper) => {
            wrapper
                .data
                .download(2023, wrapper.day, &config.session_cookie)
        }
    }
}
