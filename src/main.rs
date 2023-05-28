use rand::Rng;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let fortune_file_path = match parse_args() {
        Ok(s) => s,
        Err(e) => return println!("{e}"),
    };

    let fortunes = match Fortunes::from_file(&fortune_file_path) {
        Ok(s) => s,
        Err(e) => return println!("{e}"),
    };

    fortunes.choose_one();
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
variable FORTUNE_FILE will be used. If neither is available, fortune will abort.
"
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

struct Fortunes(Vec<String>);

impl Fortunes {
    pub fn from_file(fortune_file: &String) -> Result<Fortunes, Box<dyn Error>> {
        let path = Path::new(&fortune_file);
        if !path.exists() {
            return Err(format!("The forunte file '{fortune_file}' does not exists").into());
        }
        if path.is_dir() {
            return Err(format!("The forunte file '{fortune_file}' is a directory").into());
        }
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
                Err(err) => return Err(err.into()),
            }
        }
        if !fortune.trim().is_empty() {
            fortunes.push(fortune);
        }
        Ok(Fortunes(fortunes))
    }

    pub fn choose_one(&self) {
        let fortunes = &self.0;
        if fortunes.is_empty() {
            return;
        }
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..fortunes.len());
        print!("{}", fortunes[i]);
    }
}
