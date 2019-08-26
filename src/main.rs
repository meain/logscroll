use std::io;
use terminal_size::{terminal_size, Height, Width};

fn cycle_lines(mut lines: Vec<String>, count: usize, newline: String) -> Vec<String> {
    lines.push(newline);
    if lines.len() as usize > count {
       lines.drain(0..1);
    }
    lines
}

fn print_logs(lines: &Vec<String>) {
    // CURSOR_UP_ONE = "\x1b[1A"
    // ERASE_LINE = "\x1b[2K"
    for line in lines.iter() {
        print!("\x1b[2K");
        println!("{}", line);
    }
    for _ in 0..lines.len() {
        print!("\x1b[1A");
    }
}

fn main() {
    let stdin = io::stdin();

    let mut has_next = true;
    let mut line = String::new();

    let mut tw = 5 as usize;
    let mut th = 5 as usize;

    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        tw = (w - 2) as usize;
        if (h as usize) < th {
            th = h as usize;
        }
    }

    let mut lines: Vec<String> = Vec::new();
    // let mut lines: Vec<String> = vec!["".to_string();th];

    while has_next {
        match stdin.read_line(&mut line) {
            Ok(bytes) if bytes > 0 => {
                let sub: String = line.trim_end().chars().take(tw).collect();
                lines = cycle_lines(lines.clone(), th, sub.clone());
                print_logs(&lines);
                line.clear();
                has_next = true;
            }
            Ok(_) => {
                has_next = false;
            }
            Err(err) => return eprintln!("Error while reading stream. {}", err),
        }
    }
}
