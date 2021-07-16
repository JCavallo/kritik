extern crate clap;
extern crate kritik;

use clap::{App, AppSettings, Arg, ArgMatches};
use std::io;
use std::process;

use kritik::Kritik;
use std::default::Default;

fn main() {
    let matches = parse_command_line();
    match init_runner(&matches) {
        Ok(mut val) => val.run(),
        Err(_) => {
            println!("Missing command to run");
            process::exit(1)
        }
    };
}

fn init_runner<'a>(matches: &'a ArgMatches) -> Result<Kritik<'a>, io::Error> {
    let showtime = matches.is_present("showtime");
    let command_args: Vec<&'a str> = match matches.values_of("command") {
        Some(v) => v.collect(),
        None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing command")),
    };
    let command_line = command_args.join(" ");

    let mut kritik: Kritik = Default::default();

    kritik.set_command(command_line);

    if matches.is_present("message") {
        kritik.set_message(matches.value_of("message").unwrap().to_string());
    }
    if matches.is_present("running_message") {
        kritik.set_running_message(matches.value_of("running_message").unwrap());
    }
    if matches.is_present("success_message") {
        kritik.set_success_message(matches.value_of("success_message").unwrap());
    }
    if matches.is_present("failure_message") {
        kritik.set_failure_message(matches.value_of("failure_message").unwrap());
    }
    if showtime {
        kritik.showtime();
    }
    Ok(kritik)
}

fn parse_command_line<'a>() -> ArgMatches<'a> {
    App::new("Kritik")
        .version("0.1")
        .author("Jean Cavallo")
        .about("A simple 'chronic' alternative")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("MESSAGE")
                .help(&format!(
                    "{} {}",
                    "The message that will be displayed while the command runs.",
                    "If not set, the command itself will be used instead"
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("showtime")
                .short("s")
                .long("show-time")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("running_message")
                .help("Message that will while running, default RUNNING")
                .takes_value(true)
                .long("running-message"),
        )
        .arg(
            Arg::with_name("success_message")
                .help("Message that will be displayed on completion, default SUCCESS")
                .takes_value(true)
                .long("success-message"),
        )
        .arg(
            Arg::with_name("failure_message")
                .help("Message that will be displayed on failure, default FAILURE")
                .takes_value(true)
                .long("failure-message"),
        )
        .arg(
            Arg::with_name("command")
                .help("command to run")
                .multiple(true)
                .allow_hyphen_values(true),
        )
        .get_matches()
}
