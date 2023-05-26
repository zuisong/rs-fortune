use rand::Rng;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let fortune_file = match parse_args() {
        Ok(s) => s,
        Err(r) => {
            println!("{r}");
            return;
        }
    };
    let path = Path::new(&fortune_file);

    match path {
        _ if path.is_file() => (),
        _ => {
            if !path.exists() {
                println!("The forunte file '{fortune_file}' does not exists");
            }
            if path.is_dir() {
                println!("The forunte file '{fortune_file}' is a directory");
            }
            return;
        }
    };

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut fortunes: Vec<String> = Vec::new();
    let mut fortune: String = String::new();
    for line in reader.lines() {
        match line {
            Ok(p) if p == "%" => {
                if !fortune.trim().is_empty() {
                    fortunes.push(fortune);
                }
                fortune = String::new();
            }
            Ok(line) => {
                fortune.push_str(&line);
                fortune.push('\n');
            }
            Err(err) => {
                println!("{err}");
                return;
            }
        }
    }
    if !fortune.trim().is_empty() {
        fortunes.push(fortune);
    }
    if fortunes.is_empty() {
        return;
    }
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..fortunes.len());
    print!("{}", fortunes[i]);
}

fn parse_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let prog = Path::new(&args[0]).file_name().unwrap().to_str().unwrap();
    let usage = format!(
        "{prog}

Usage:
  {prog} [/path/to/fortune/cookie/file]
  {prog} -h|--help

If the fortune cookie file path is omitted, the contents of environment
variable FORTUNE_FILE will be used. If neither is available, fortune will abort.",
    );

    match args.len() {
        1 => Ok(env::var("FORTUNE_FILE").map_err(|_| usage.clone())?),
        2 => match args[1].as_str() {
            "-h" | "--help" => Err(usage.into()),
            x => Ok(x.to_string()),
        },
        _ => Err(usage.into()),
    }
}
