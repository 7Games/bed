//! # bed
//!
//! Better ED, or Benjamin's ED.
//!
//! Reimplementation of ed(1) in Rust, because everything needs to be in here.

use std::fs::File;
use std::io::{self, Read};

/// Modes the editor can be in
enum EditorMode {
    /// Able to run commands such as p, w, q, etc
    Command = 0,
    /// Append new text at the current line number
    Insert,
}

/// Current state of the editor
struct Editor {
    /// Lines of the file
    lines: Vec<String>,
    /// The current line (0-indexed)
    line_number: usize,
    /// Current mode the editor is in
    mode: EditorMode,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            lines: vec![],
            line_number: 0,
            mode: EditorMode::Command,
        }
    }
}

impl Editor {
    /// Opens a file and inserts it into [`Self::lines`]
    fn open_file(&mut self, file_path: &str) {
        let mut data_file = File::open(file_path).unwrap();
        let mut file_content = String::new();

        data_file.read_to_string(&mut file_content).unwrap();

        let lines = file_content.split("\n");

        for line in lines {
            self.lines.push(line.to_string());
        }
    }

    /// Inserts a line of text into the `line_number` at [`Self::lines`]
    ///
    /// Creates a new line at line_number (0-indexed, you must do this
    /// before passing), and fills it with `line`.
    fn insert_text(&mut self, line_number: usize, line: &str) {
        if line_number > self.lines.len() {
            println!("?");
            return;
        }

        self.lines.insert(line_number, line.to_string());
    }

    /// Prints the current line
    fn print(&self, display_line_number: bool) {
        if display_line_number {
            print!("{}\t", self.line_number + 1);
        }

        println!("{}", self.lines[self.line_number]);
    }

    /// Print all the lines in range of `from`-`to`
    fn print_range(&self, from: usize, to: usize, display_line_number: bool) {
        if from > to {
            println!("?");
            return;
        }

        for i in from..to {
            if i > self.lines.len() {
                println!("?");
            } else {
                if display_line_number {
                    print!("{}\t", i + 1);
                }
                println!("{}", self.lines[i]);
            }
        }
    }

    /// Moves [`Self::line_number`] to `new_line`
    ///
    /// Remember! [`Self::line_number`] is 0-indexed!
    fn goto_line(&mut self, new_line: usize) {
        if new_line > self.lines.len() {
            println!("?");
            return;
        }

        self.line_number = new_line;
    }
}

/// Returns whether a given string is a number
fn is_number(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }
    true
}

fn main() {
    let mut editor: Editor = Default::default();
    let stdin = io::stdin();
    let input = &mut String::new();

    editor.open_file("./main.rs");

    loop {
        input.clear();
        let _ = stdin.read_line(input);

        match editor.mode {
            EditorMode::Command => {
                match input.as_str().trim() {
                    // Goto next line (enter key technically but it's nothing because it's trimmed)
                    "" => {
                        editor.goto_line(editor.line_number + 1);
                        editor.print(false);
                    }
                    // Print
                    "p" => editor.print(false),
                    ",p" => editor.print_range(0, editor.lines.len(), false),
                    // Print with line numbers
                    "n" => editor.print(true),
                    ",n" => editor.print_range(0, editor.lines.len(), true),
                    // Insert
                    "i" => {
                        editor.line_number += 1;
                        editor.mode = EditorMode::Insert;
                    },
                    "a" => editor.mode = EditorMode::Insert,
                    // Assume number?
                    &_ => {
                        if is_number(input.trim()) {
                            if input.trim() == "0" {
                                println!("?");
                                continue;
                            }
                            editor.goto_line(input.trim().parse::<usize>().unwrap() - 1);
                            editor.print(false);
                        } else {
                            println!("?");
                        }
                    },
                }
            },
            EditorMode::Insert => {
                match input.as_str().trim() {
                    // Exit input mode
                    "." => {
                        editor.mode = EditorMode::Command;
                    }
                    // Insert as is (except trimmed ig)
                    &_ => {
                        // TODO: Not trimmed? Maybe just get rid of \n
                        editor.insert_text(editor.line_number, input.trim());
                        editor.line_number += 1;
                    },
                }
            },
        }
    }
}
