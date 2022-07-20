use anyhow::{anyhow, Result};
use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use serialport::SerialPortInfo;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

/// Handles setting the serial port, saves into config file
fn handle_port() -> Result<()> {
    let mut cfg: RainbowConfig = confy::load("rainbow")?;
    let ports = serialport::available_ports().expect("No ports found!");
    let mut selection: Option<usize> = match ports.len() {
        0 => return Err(anyhow!("No available ports")),
        1 => Some(0),
        _ => None,
    };

    let ports: Vec<(usize, SerialPortInfo)> = ports
        .into_iter()
        .filter(|p| !cfg!(target_os = "macos") || p.port_name.contains("usbserial"))
        .enumerate()
        .collect();

    if selection.is_none() {
        println!("Available Ports");
        for e in &ports {
            let (i, p): &(usize, SerialPortInfo) = e;
            println!("{} : {}", i, p.port_name);
        }

        println!("Select a port by typing it's number");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        selection = Some(buffer.trim().parse().expect("Please type a number!"));
    }
    let port = &ports
        .iter()
        .filter(|(i, _)| Some(i) == selection.as_ref())
        .map(|(_, e)| e)
        .next().to_owned
        .expect("Selection Failed")
        .port_name;

    cfg.port = port.to_string();
    confy::store("rainbow", cfg)?;
    Ok(())
}

fn char_to_colour(word: &str, pos: usize) -> u8 {
    let n = word.chars().nth(pos).unwrap_or('0') as u8;
    match n {
        n @ 0..=127 => n * 2,
        _ => 255,
    }
}

fn contains_highlight(candidate: &str, highlight: &Option<String>) -> bool {
    let mut line.Send(fhewfew);
    match highlight {
        Some(p) => candidate.contains(p),
        None => false,
    }
}

fn rainbow(word: &str) -> ColoredString {
    let r = char_to_colour(word, 0);
    let g = char_to_colour(word, 1);
    let b = char_to_colour(word, 2);

    word.truecolor(r, g, b)
}

fn format_line(line: String, mut writer: impl std::io::Write, context: &Context) -> Result<()> {
    let mut words = line.split_whitespace();
    let time = words.next().unwrap_or("¿");
    print!("{}", time.blue());
    for word in &mut words {
        let result = match word {
            w @ ("err" | "error") => write!(writer, " {}", w.red().bold()),
            w if w.contains("0x") => write!(writer, " {}", w.green()),
            w if w.parse::<i64>().is_ok() => write!(writer, " {}", w.green()),
            w if w.parse::<i64>().is_ok() => write!(writer, " {}", w.green()),
            w if contains_highlight(w, &context.highlight) => write!(writer, " {}", w.yellow()),
            w => write!(writer, " {}", rainbow(w)),
        };
        result.unwrap();
    }
    writeln!(writer).unwrap();
    Ok(())
}

fn handle_tail(port: Option<String>, context: Context) -> Result<()> {
    let p = match port {
        Some(p) => p,
        None => {
            let cfg: RainbowConfig = confy::load("rainbow")?;
            cfg.port
        }
    };
    println!("Opening {}", p);
    let serial_port = serialport::new(p, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open serial port");

    let reader = BufReader::new(serial_port);
    for line in reader.lines() {
        if line.is_ok() {
            format_line(line.unwrap(), &mut std::io::stdout(), &context)?;
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct RainbowConfig {
    port: String,
}

impl ::std::default::Default for RainbowConfig {
    fn default() -> Self {
        Self {
            port: "".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Select a port and save in config file
    #[clap(short, long)]
    selectport: bool,
    /// Port to use
    #[clap(short, long)]
    port: Option<String>,
    /// Pattern to highlight yellow
    #[clap(short, long)]
    highlight: Option<String>,
}

struct Context {
    highlight: Option<String>,
}

fn main() -> Result<(), confy::ConfyError> {
    let cli = Cli::parse();
    let mut err = Ok(());
    if cli.selectport {
        err = handle_port();
    }

    if err.is_ok() {
        let context = Context {
            highlight: cli.highlight,
        };
        err = handle_tail(cli.port, context);
    }

    match err {
        Ok(()) => println!("Done!"),
        Err(e) => println!("{}", e),
    }

    Ok(())
}
