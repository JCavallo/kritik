extern crate clap;
extern crate kritik;

use clap::{App, AppSettings, Arg, ArgMatches};

use kritik::Kritik;


fn main() {
    let matches = parse_command_line();
    init_runner(&matches)
        .return_exit_code()
        .run();
    Kritik::new()
        .set_message("Hi!")
        .set_command(String::from("sleep 2 && git status"))
        .return_exit_code()
        .run();
    Kritik::new()
        .set_message("Having fun...")
        .showtime()
        .set_command(String::from("sleep 3 && azdqsdazdqs"))
        .return_exit_code()
        .run();
    Kritik::new()
        .set_command(String::from("sleep 2 && git status"))
        .return_exit_code()
        .run();
    Kritik::new()
        .set_message("Final test...")
        .set_command(String::from("sleep 2 && azdqsdazzz"))
        .run();
}

fn init_runner<'a>(matches: &'a ArgMatches) -> Kritik <'a> {
    let showtime = matches.is_present("showtime");
    let message = matches.value_of("message").unwrap();
    let command_args: Vec<&'a str> = matches.values_of("command")
        .unwrap().collect();
    let command_line = command_args.join(" ");

    let mut kritik = Kritik::new()
        .set_command(command_line)
        .set_message(message);

    if showtime {
        kritik = kritik.showtime();
    }
    kritik
}

fn parse_command_line<'a>() -> ArgMatches <'a> {
    App::new("Kritik")
        .version("0.1")
        .author("Me")
        .about("Perfection")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("message")
             .short("m")
             .long("message")
             .value_name("MESSAGE")
             .help("The message that will be displayed while the command runs")
             .takes_value(true)
             )
        .arg(Arg::with_name("showtime")
             .short("s")
             .long("show-time")
             .takes_value(false)
             )
        .arg(Arg::with_name("command")
             .help("command to run")
             .multiple(true)
             .allow_hyphen_values(true)
             )
        .get_matches()
}

