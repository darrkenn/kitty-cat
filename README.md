# kitty-cat

kitty-cat is a command line tool that fetches a cat from [CATAAS](https://cataas.com/) and displays it in your terminal. Built for **Kitty** terminal.

## Requirements

- Kitty terminal
- Rust

## Installation

````
```bash
git clone https://github.com/darrkenn/kitty-cat
cd kitty-cat
cargo build --release
./target/release/kitty-cat -c
```

## Usage
```bash
kitty-cat -t  #Retrieves all usable tags.
kitty-cat -s  #Retrieves image but doesnt display it.
kitty-cat -c  #Sets up folders/files.
```
````
