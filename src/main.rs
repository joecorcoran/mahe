#[macro_use]
extern crate clap;
extern crate hyper;

mod server;

fn main() {
    let cli_config = load_yaml!("cli.yml");
    let cli_matches = clap::App::from_yaml(cli_config).get_matches();

    if cli_matches.is_present("server") {
        server::start();
    }
}
