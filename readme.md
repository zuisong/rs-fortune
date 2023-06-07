# rs-fortune

[🇨🇳 中文版 Readme](./readme-cn.md)

`rs-fortune` is a simple Rust command-line tool that prints random fortune entries.

## Installation

- Download and install from the [Release page](https://github.com/zuisong/rs-fortune/releases)
- Install via Cargo:

    ```bash
    cargo install --git https://github.com/zuisong/rs-fortune
    ```

- MacOS users can install via Homebrew:

    ```bash
    brew install zuisong/tap/rs-fortune
    ```

## Usage

```txt
rs-fortune [options] <fortune file>
Options:

--help - Print help information
--completion <shell> - Generate shell completion script for <shell>, where <shell> can be bash, zsh, fish, etc.
```

`<fortune file>` is a text file containing fortune entries, separated by `%` on a line of its own.

`rs-fortune` also supports reading `<fortune file>` path from the `FORTUNE_FILE` environment variable:

```bash
FORTUNE_FILE=fortunes.txt rs-fortune
```

If both command-line arguments and environment variables are provided, the command-line arguments take precedence.

For example, if a fortune file named `fortunes.txt` contains:

```txt
Fortune favors the bold.
%
The early bird gets the worm.
%
Slow and steady wins the race.
```

You can use it like this:

```bash
rs-fortune fortunes.txt
# May output "Fortune favors the bold."

### You can also read the fortune file path from an environment variable
FORTUNE_FILE=fortunes.txt rs-fortune 

### And it can read fortunes from pipeline
cat fortunes.txt | rs-fortune  

# Print help information
rs-fortune --help  
```

If `<fortune file>` or `FORTUNE_FILE` environment variable is not specified, it will print a default fortune.

This is a simple and practical command-line tool that can be used to print random quotes and sayings. Feedback and suggestions for improvement are welcome!
