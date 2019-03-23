//! kritik is both a library and a binary to "nicely" run programs in the command line. It was
//! inspired by `chronic`, which is included in the [moreutils](https://joeyh.name/code/moreutils/)
//! tools, and "runs a command quietly unless it fails".
//!
//! * As a binary, it can be used in scripts to run commands, only showing the output if there is
//! an error
//! * This can also be used as a library if you want to use it from your own code
//!
//! It depends on [indicatif](https://docs.rs/indicatif) for displaying, and currently only targets
//! `bash` on Linux.
//!
//! # Binary Usage
//!
//! Basic usage would be:
//!
//! ```
//! kritik git pull
//! ```
//!
//! It is possible to set a message that will be displayed while the command runs:
//!
//! ```
//! kritik --message "Updating" git pull
//! ```
//!
//! Chained commands can be called with quotes:
//!
//! ```
//! kritik "git fetch -p origin && git merge origin/master"
//! ```
//!
//! The output will be the message and a spinner while the command runs. The standard output /
//! errors will only be shown if the commands exits with a non-zero error code.
//!
//! Available options can be listed with:
//!
//! ```
//! kritik --help
//! ```
//!
//! # Library Usage
//!
//! The library exposes the [`Kritik`](struct.Kritik.html) structure, which offers the same
//! possibilities than the binary, but allows to control whether the program should exit or just
//! manually manage the error codes.
//!
//! ```rust
//! let mut kritik: Kritik = Default::default();
//! kritik.set_command(String::from("ls /tmp && sleep 2"));
//! kritik.set_message(String::from("Lsing tmp"));
//! kritik.set_success_message("Everything went well");
//! kritik.showtime();
//! kritik.return_exit_code();
//! let error_code = kritik.run();
//! ```
extern crate console;
extern crate indicatif;

use std::process::{self, Command, Output, Stdio};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

enum FinalBehavior {
    Exit,
    ReturnCode,
}

/// The structure that holds the configuration that will be run
pub struct Kritik<'a> {
    showtime: bool,
    running_text: &'a str,
    success_text: &'a str,
    failure_text: &'a str,
    message: String,
    command: String,
    progress_bar: ProgressBar,
    template: String,
    behavior: FinalBehavior,
}

impl<'a> Default for Kritik<'a> {
    /// The default configuration
    ///
    /// Default behavior is as follow:
    /// * Text while running will be "RUNNING"
    /// * Text in case of failure will be "FAILURE"
    /// * Text in case of success will be "SUCCESS"
    /// * Message while running will be the same as the command that was run
    /// * In case of error, the process will exit, returning the command's error code
    fn default() -> Kritik<'a> {
        let showtime = false;
        let progress_bar = ProgressBar::new_spinner();
        Kritik {
            showtime,
            running_text: "RUNNING",
            success_text: "SUCCESS",
            failure_text: "FAILURE",
            message: String::from(""),
            command: String::from(""),
            progress_bar,
            template: "".to_string(),
            behavior: FinalBehavior::Exit,
        }
    }
}

impl<'a> Kritik<'a> {
    /// Defines the command that will be ran
    pub fn set_command(&mut self, command: String) {
        self.command = command;
    }

    /// If called, the time since the command started will be displayed next to the message while
    /// it is running
    pub fn showtime(&mut self) {
        self.showtime = true;
    }

    /// Defines the message that will be displayed while the command runs
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    /// The status string for a running command. Ignored if ``showtime`` was called
    pub fn set_running_message(&mut self, running_message: &'a str) {
        self.running_text = running_message;
    }

    /// The status string for a successful command
    pub fn set_success_message(&mut self, success_message: &'a str) {
        self.success_text = success_message;
    }

    /// The status message for a failed command
    pub fn set_failure_message(&mut self, failure_message: &'a str) {
        self.failure_text = failure_message;
    }

    /// If called, the process will returns the command's exit code rather than stopping the
    /// program
    pub fn return_exit_code(&mut self) {
        self.behavior = FinalBehavior::ReturnCode;
    }

    fn build_template(&mut self) {
        if self.message.is_empty() {
            self.message = self.command.clone();
        }
        self.template.push_str("{spinner:.bold.cyan}");
        if self.showtime {
            self.template.push_str(" [{elapsed_precise:.bold}]");
        } else {
            self.template
                .push_str(&format!(" [{}]", style(self.running_text).cyan().bold()));
        }
        self.template.push_str(" {wide_msg}");
    }

    fn build_spinner(&mut self) {
        self.progress_bar.enable_steady_tick(50);
        self.progress_bar
            .set_style(ProgressStyle::default_spinner().template(&self.template));
        self.progress_bar.set_message(&self.message);
    }

    pub fn run(&mut self) -> i32 {
        self.build_template();
        self.build_spinner();
        let result = self.execute_command();
        self.progress_bar.finish_and_clear();
        if result.status.success() {
            self.handle_success(&result);
        } else {
            self.handle_failure(&result);
        };
        let status_code = result.status.code().unwrap();
        match self.behavior {
            FinalBehavior::ReturnCode => status_code,
            FinalBehavior::Exit => process::exit(status_code),
        }
    }

    fn execute_command(&self) -> Output {
        Command::new("bash")
            .arg("-c")
            .arg(&self.command)
            .stdin(Stdio::null())
            .output()
            .unwrap()
    }

    fn handle_success(&self, output: &Output) {
        println!(
            "  [{}] {}",
            style(self.success_text).green().bold(),
            self.message
        );
    }

    fn handle_failure(&self, output: &Output) {
        println!(
            "  [{}] {}",
            style(self.failure_text).red().bold(),
            self.message
        );
        println!(
            "  [{}] {}",
            style("ERROR CODE").red().bold(),
            output.status.code().unwrap()
        );
        let stdout = String::from_utf8(output.stdout.clone()).unwrap();
        if stdout != "" {
            println!("  [{}]", style("STDOUT").white().bold());
            print!("{}", stdout);
        } else {
            println!("  [{}] Empty", style("STDOUT").white().bold());
        }
        let stderr = String::from_utf8(output.stderr.clone()).unwrap();
        if stderr != "" {
            println!("  [{}]", style("STDERR").white().bold());
            print!("{}", stderr);
        } else {
            println!("  [{}] Empty", style("STDERR").white().bold());
        }
    }
}
