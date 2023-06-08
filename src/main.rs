use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use rand::Rng;
use std::{
    error::Error,
    fs::File,
    io::{self, stdin, stdout, IsTerminal, Read},
    path::Path,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// The fortune cookie file path
    #[arg(env = "FORTUNE_FILE")]
    fortune_file: Option<String>,
    #[arg(long = "completion", value_enum)]
    completion: Option<Shell>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
}

pub fn read_pipe() -> Option<String> {
    let mut buf = String::new();
    if !stdin().is_terminal() {
        stdin().read_to_string(&mut buf).ok()?;
    }
    (!buf.trim().is_empty()).then_some(buf.trim().into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if let Some(shell) = args.completion {
        print_completions(shell, &mut Args::command());
        return Ok(());
    }

    if let Some(buf) = read_pipe() {
        // read fortune data from pipe
        Fortunes::new(buf)?.choose_one();
    } else if let Some(ref fortune_file) = args.fortune_file {
        Fortunes::from_file(fortune_file)?.choose_one();
    } else {
        Args::command().print_help()?;
    }

    Ok(())
}

struct Fortunes(Vec<String>);

impl Fortunes {
    pub fn new(content: String) -> Result<Fortunes, Box<dyn Error>> {
        let fortunes: Vec<String> = content
            .split("\n%\n")
            .into_iter()
            .map(|it| it.to_string())
            .collect();
        Ok(Self(fortunes))
    }

    pub fn from_file(fortune_path: &String) -> Result<Fortunes, Box<dyn Error>> {
        let path = Path::new(&fortune_path);
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("The fortune file '{fortune_path}' does not exist"),
            )
            .into());
        }
        if path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("'{fortune_path}' is a directory, not a file"),
            )
            .into());
        }
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Self::new(buf)
    }

    pub fn choose_one(&self) {
        let fortunes = &self.0;
        if fortunes.is_empty() {
            return;
        }
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..fortunes.len());
        println!("{}", fortunes[i]);
    }
}

#[cfg(test)]
mod tests;
