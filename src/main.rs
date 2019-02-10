extern crate console;
extern crate indicatif;
extern crate clap;

use std::thread;
use std::time::Duration;

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use console::style;


fn main() {
    let matches = App::new("Kritik")
        .version("0.1")
        .author("Me")
        .about("Perfection")
        .arg(Arg::with_name("message")
             .short("m")
             .long("message")
             .value_name("MESSAGE")
             .help("The message that will be displayed to identify the command")
             .takes_value(true)
             .required(true)
             )
        .arg(Arg::with_name("showtime")
             .short("s")
             .long("show-time")
             .takes_value(false)
             )
        .get_matches();

    let showtime = matches.is_present("showtime");
    let message = matches.value_of("message").unwrap();
    let mut template = "".to_string();
    template.push_str("{spinner:.bold.cyan}");

    if showtime {
        template.push_str(" [{elapsed_precise:.bold}]");
    } else {
        template.push_str(
            &format!(" [{}]", style("RUNNING").cyan().bold()));
    }

    template.push_str(" {wide_msg}");

    let progress_bar = ProgressBar::new_spinner();
    progress_bar.enable_steady_tick(50);
    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template(&template),
        );

    progress_bar.set_message(&message);
    for _ in 0..256 {
        thread::sleep(Duration::from_millis(5));
    }

    progress_bar.finish_and_clear();
    println!("  [{}] {}", style("SUCCESS").green().bold(), message);
}
