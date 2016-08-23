extern crate clap;
extern crate hyper;
extern crate reroute;

mod server;
mod store;

use clap::{App, Arg, SubCommand};
use server::{ServerOptions, start};

fn main() {
    let cli_matches = App::new("mahe")
        .version("0.0")
        .author("Joe Corcoran <joe@corcoran.io>")
        .subcommand(SubCommand::with_name("server")
            .arg(Arg::with_name("database").long("database").short("d").takes_value(true))
            .arg(Arg::with_name("ip").long("ip").short("i").takes_value(true))
            .arg(Arg::with_name("port").long("port").short("p").takes_value(true))
        ).get_matches();

    if let Some(server_matches) = cli_matches.subcommand_matches("server") {
        let options = ServerOptions {
            ip: server_matches.value_of("ip").unwrap_or("0.0.0.0"),
            port: server_matches.value_of("port").unwrap_or("8989")
        };
        start(server_matches.value_of("database").unwrap_or("default"), options);
    }
}
