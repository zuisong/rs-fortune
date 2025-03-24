use clap::{Command, CommandFactory, Parser};
use clap_complete::{Generator, Shell, generate};
use std::{
    error::Error,
    fs::File,
    io::{self, IsTerminal, Read, stdin, stdout},
    path::Path,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[command(subcommand)]
    command: Option<SubCommand>,

    /// The fortune cookie file path
    #[arg(env = "FORTUNE_FILE")]
    fortune_file: Option<String>,
}
#[derive(Parser, Debug, Clone)]
enum SubCommand {
    /// Generate tab-completion scripts for your shell
    Completions {
        #[arg(long = "shell", short = 's', value_enum)]
        shell: Shell,
    },
}

fn print_completions<G: Generator>(g: G, cmd: &mut Command) {
    generate(g, cmd, cmd.get_name().to_string(), &mut stdout());
}

pub fn read_pipe() -> Option<String> {
    let mut input = String::new();
    if !stdin().is_terminal() {
        stdin().read_to_string(&mut input).ok()?;
    }
    (!input.trim().is_empty()).then_some(input.trim().into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Some(SubCommand::Completions { shell }) => {
            print_completions(shell, &mut Args::command());
        }
        None => {
            if let Some(input) = read_pipe() {
                // read fortune data from pipe
                Fortunes::new(input)?.choose_one();
            } else if let Some(ref path) = args.fortune_file {
                Fortunes::from_file(path)?.choose_one();
            } else {
                Args::command().print_help()?;
            }
        }
    }

    Ok(())
}

struct Fortunes(Vec<String>);

impl Fortunes {
    pub fn new(content: String) -> Result<Fortunes, Box<dyn Error>> {
        let fortunes = content.split("\n%\n").map(|it| it.to_string()).collect();
        Ok(Self(fortunes))
    }

    pub fn from_file(path: &String) -> Result<Fortunes, Box<dyn Error>> {
        let file_path = Path::new(&path);
        if !file_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("The fortune file '{path}' does not exist"),
            )
            .into());
        }
        if file_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("'{path}' is a directory, not a file"),
            )
            .into());
        }
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Self::new(content)
    }

    pub fn choose_one(&self) {
        let Fortunes(fortunes) = &self;
        if fortunes.is_empty() {
            return;
        }

        if let Some(fortune) = fastrand::choice(fortunes) {
            println!("{}", fortune);
        }
    }
}

#[cfg(test)]
mod tests;
