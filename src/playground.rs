use std::fs;
use std::ops::Not;

use colored::*;
use rustyline::error::ReadlineError;

use crate::prompt::Prompt;
use crate::regex_attempt::RegexAttempt;
use crate::text::Text::{self, *};

struct PlaygroundData {
    pub test_strings: Vec<Text>,
    pub editor: Prompt,
}

impl PlaygroundData {
    fn new() -> Self {
        Self {
            test_strings: vec![],
            editor: Prompt::new(),
        }
    }

    fn print_help() {
        println!("{} receives a word (or a list of word) and adds it to the set of test targets. Note that this mode trims its words.", "#addword".blue());
        println!("{} adds the entire line after the command as a test target. Trailing whitespace will be kept.", "#addline".blue());
        println!(
            "{} reads the indicated file and saves it as a single test target.",
            "#readfile".blue()
        );
        println!("{} clears all loaded test strings. If you just want to clean your terminal, Ctrl+L works.", "#clear".blue());
    }

    fn load_from_file(&mut self, filename: &str) {
        match fs::read_to_string(filename) {
            Ok(contents) => {
                self.test_strings.push(Text::Line(contents));
            }
            Err(err) => {
                eprintln!("Problem reading file: {:?}", err);
            }
        }
    }

    fn parse(&mut self, line: &str) {
        let line = line.trim_start();
        if line.starts_with('#') {
            // Likely a command, such as #help or #add
            let words = line.split(' ');
            match line {
                line if line.starts_with("#help") => {
                    Self::print_help();
                }
                line if line.starts_with("#clear") => {
                    println!("Test strings cleared!");
                    self.test_strings.clear();
                }
                line if line.starts_with("#addword") => {
                    words
                        .skip(1)
                        .for_each(|s| self.test_strings.push(Word(s.into())));
                }
                line if line.starts_with("#addline") => {
                    if line.len() > 9 {
                        self.test_strings.push(Line(String::from(&line[9..])))
                    }
                }
                line if line.starts_with("#readfile") => {
                    if line.len() > 10 {
                        self.load_from_file(&line[10..]);
                    }
                }
                _ => {}
            }
        }

        let _ = RegexAttempt::new(line, &*self.test_strings)
            .map(|attempt| attempt.print_matches())
            .map_err(show_regex_error);
    }
}

fn show_regex_error(err: regex::Error) {
    use regex::Error::*;

    // The regex crate has nicely formatted errors, so
    // it's better to deestructure them and show their
    // error messages rather than just using {:?}
    match err {
        Syntax(err) => eprintln!("{}", err),
        CompiledTooBig(size_limit) => eprintln!(
            "The compiled program exceeded the set size limit ({}).",
            size_limit
        ),
        other => {
            // regex::Error is marked non-exhaustive
            eprintln!("{:?}", other)
        }
    }
}

pub struct PlaygroundManager {}

impl PlaygroundManager {
    /// Prints the introductory playground text
    fn display_intro_text() {
        println!("Welcome to the {} {}. Type in {} to get additional help. \nType in {} {} to add new words as test targets.",
            "Jacarex".green(),
            "Playground".green().bold(),
            "#help".blue(),
            "#addword".blue(),
            "<strings>".blue().bold(),
        )
    }

    /// Starts the Playground loop.
    pub fn start() {
        PlaygroundManager::display_intro_text();
        let mut data = PlaygroundData::new();
        // data.use_arg_values(arg_values);
        loop {
            match data.editor.read_line(">> ") {
                Ok(line) if line.is_empty().not() => data.parse(line.as_str()),
                Ok(_empty_string) => {},
                Err(err) => {
                    // Prints some additional info depending on which error we're getting
                    check_readline_error(err);
                    data.editor.save_history();

                    // Stop the loop
                    return;
                }
            }
        }
    }
}


pub(crate) fn check_readline_error(err: ReadlineError) {
    match err {
        ReadlineError::Interrupted => {
            println!("SIGINT received. Exiting.");
        }
        ReadlineError::Eof => {
            println!("EOF received. Exiting.");
        }
        err => {
            eprintln!("Error: {:?}", err);
        }
    }
}
