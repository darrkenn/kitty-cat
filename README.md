# kitty-cat

kitty-cat is a command line tool that fetches a cat from [CATAAS](https://cataas.com/) and displays it in your terminal. Built for **Kitty** terminal.

## Requirements

- Kitty terminal
- Rust

## Installation

```bash
git clone https://github.com/darrkenn/kitty-cat
cd kitty-cat
cargo build --release
./target/release/kitty-cat -c
```

## Usage

```bash
kitty-cat #Retrives and displays image
kitty-cat -t  #Retrieves all usable tags.
kitty-cat -s  #Retrieves image but doesnt display it.
kitty-cat -c  #Sets up folders/files.
```

## Configuration

The config file is located at ~/.config/share/kitty-cat/config.toml.

### Example config.toml

```toml
offline = false #Required: Dont retrieve images, instead look in cache.
cache = false #Required: Cache images.
image_type = "square" #Optional: xsmall, small, medium, square.
tags = ["orange"] #Optional: List of tags (Run kitty-cat -t to view avaiable tags).
alignment = "left" #Optional: Left, Center, Right.

[says] #Optional
sentence = "h"
size = 32
color = "blue"


[dimensions] #Optional
height = 200 #In pixels.
width = 200 #In pixels.

[filter] #Optional
kind = "custom" #mono, negate ,custom
brightness = 1
lightness = 1
saturation = 1

[filter.rgb] #Optional
r = 244
g = 1
b = 150
```
