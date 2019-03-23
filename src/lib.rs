extern crate console;
extern crate indicatif;

use std::process::{self, Command, Output, Stdio};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

enum FinalBehavior {
    Exit,
    ReturnCode,
}

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
    fn default() -> Kritik<'a> {
        let showtime = false;
        let progress_bar = ProgressBar::new_spinner();
        Self {
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
    pub fn set_command(mut self, command: String) -> Self {
        self.command = command;
        self
    }

    pub fn showtime(mut self) -> Self {
        self.showtime = true;
        self
    }

    pub fn set_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn set_running_message(mut self, running_message: &'a str) -> Self {
        self.running_text = running_message;
        self
    }

    pub fn set_success_message(mut self, success_message: &'a str) -> Self {
        self.success_text = success_message;
        self
    }

    pub fn set_failure_message(mut self, failure_message: &'a str) -> Self {
        self.failure_text = failure_message;
        self
    }

    pub fn return_exit_code(mut self) -> Self {
        self.behavior = FinalBehavior::ReturnCode;
        self
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
