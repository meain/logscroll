use atty::Stream;
use std::env;
use std::io;
use std::io::Write;
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
    let mut writer = io::stdout();

    let mut has_next = true;
    let mut line = String::new();

    let mut tw = 5 as usize;
    let mut th = 5 as usize;

    if !atty::is(Stream::Stdout) {
        while has_next {
            match stdin.read_line(&mut line) {
                Ok(bytes) if bytes > 0 => {
                    match write!(writer, "{}", line) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                    line.clear();
                    has_next = true;
                }
                Ok(_) => {
                    has_next = false;
                }
                Err(err) => return eprintln!("Error while reading stream. {}", err),
            }
        }
        return;
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        th = args[1].parse().unwrap();
        if th < 1 {
            th = 1;
        }
    }
    if args.len() > 2 {
        tw = args[2].parse().unwrap();
        if tw < 1 {
            tw = 1;
        }
    }

    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        if args.len() < 3 || ((w - 2) as usize) < tw {
            tw = (w - 2) as usize;
        }
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
