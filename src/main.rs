use clap::{Command, CommandFactory, Parser};
use clap_complete::{Generator, Shell, generate};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, IsTerminal, stdin, stdout},
    mem::take,
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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Some(SubCommand::Completions { shell }) => {
            print_completions(shell, &mut Args::command());
        }
        None => {
            if let Some(path) = args.fortune_file {
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
                let file = File::open(file_path)?;
                if let Some(fortune) = Fortunes::sample(BufReader::new(file))? {
                    println!("{}", fortune);
                }
            } else if !stdin().is_terminal() {
                if let Some(fortune) = Fortunes::sample(stdin().lock())? {
                    println!("{}", fortune);
                }
            } else {
                Args::command().print_help()?;
            }
        }
    }

    Ok(())
}

struct Fortunes;

impl Fortunes {
    pub fn sample(reader: impl BufRead) -> Result<Option<String>, Box<dyn Error>> {
        let mut selected: Option<String> = None;
        let mut current_fortune = String::new();
        let mut count = 0;

        for line_res in reader.lines() {
            let line = line_res?;
            if line.trim() == "%" {
                Self::consider_fortune(&mut selected, &mut count, &mut current_fortune);
            } else {
                if !current_fortune.is_empty() {
                    current_fortune.push('\n');
                }
                current_fortune.push_str(&line);
            }
        }

        // Handle the last fortune if the file doesn't end with %
        Self::consider_fortune(&mut selected, &mut count, &mut current_fortune);

        Ok(selected)
    }

    fn consider_fortune(selected: &mut Option<String>, count: &mut usize, current: &mut String) {
        if current.trim().is_empty() {
            current.clear();
            return;
        }

        *count += 1;
        // Use Reservoir Sampling to pick one fortune with uniform probability 1/n.
        if fastrand::usize(..*count) == 0 {
            *selected = Some(take(current));
        } else {
            current.clear();
        }
    }
}
