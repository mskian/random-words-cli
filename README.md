# Random Words CLI 📚️

Rust CLI for Fetching a Random Word with Its Definition and Pronunciation.  

> CLI inspired from: <https://github.com/mcnaveen/random-words-cli>  

## Requirements

- Rust: <https://www.rust-lang.org/>
- Random Words API: <https://github.com/sanwebinfo/ts-random-words>  

## Installation

- Download or clone the repo

```sh
git clone https://github.com/mskian/random-words-cli.git
```

- Create a `.env` file in the root of your project with the following content

```sh
RANDOMWORDS_API_URL=https://api.example.com/words
```

- it support global environment variables too

```sh
export RANDOMWORDS_API_URL=https://api.example.com/words
```

- Unset the Global Variable

```sh
unset RANDOMWORDS_API_URL=https://api.example.com/words
```

- Test the CLI

```sh
cargo run
cargo run -- -h
cargo run -- -v
```

- Production build

```sh
cargo build --release
```

- Random Words CLI

```sh
sudo mv target/release/randomwords /usr/local/bin/randomwords
randomwords --help
```

## LICENSE

MIT
