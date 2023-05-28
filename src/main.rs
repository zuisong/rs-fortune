use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use rand::Rng;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// The fortune cookie file path
    #[arg(env = "FORTUNE_FILE")]
    file: String,
    // If provided, outputs the completion file for given shell
    #[arg(long = "completion", value_enum)]
    completion: Option<Shell>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn main() {
    let args = Args::parse();

    if let Some(shell) = args.completion {
        print_completions(shell, &mut Args::command());
        return;
    }

    let fortunes = match Fortunes::from_file(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            process::exit(-1)
        }
    };

    fortunes.choose_one();
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

#[cfg(test)]
mod tests;
