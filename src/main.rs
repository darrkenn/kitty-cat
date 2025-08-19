use std::{
    env::home_dir,
    error::Error,
    fs::{self, File},
    io::copy,
    path::PathBuf,
    process::Command,
};

use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct Dimensions {
    height: u32,
    width: u32,
}

#[derive(Deserialize)]
struct Says {
    sentence: String,
    size: Option<u8>,
    color: Option<String>,
}

#[derive(Deserialize)]
enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Deserialize)]
enum Filter {
    Blur,
    Mono,
    Negate,
    Custom,
}

#[derive(Deserialize)]
struct Config {
    cache: bool,
    alignment: Option<Alignment>,
    tags: Option<Vec<String>>,
    says: Option<Says>,
    filter: Option<String>,
    dimensions: Option<Dimensions>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cache: false,
            alignment: Some(Alignment::Left),
            tags: None,
            says: None,
            filter: None,
            dimensions: None,
        }
    }
}

impl Config {
    fn alignment_to_string(&self) -> String {
        if let Some(alignment) = &self.alignment {
            match alignment {
                Alignment::Left => "left".to_string(),
                Alignment::Center => "center".to_string(),
                Alignment::Right => "right".to_string(),
            };
        };
        "left".to_string()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //Getting options from config file
    let config = get_config();
    let config_string = fs::read_to_string(config).expect("Cant read config file");
    let config: Config =
        toml::from_str((config_string).as_ref()).expect("Cant parse config string");

    //If cache is set to false it will get a new image.
    if !config.cache {
        let url: String = get_url(&config);
        let mut response = get(url)?;
        let mut file = File::create(get_image_location()).expect("Cant create file");
        copy(&mut response, &mut file).expect("Cant write file");
    }

    Command::new("kitten")
        .arg("icat")
        .arg("--align")
        .arg(config.alignment_to_string())
        .arg(get_image_location())
        .status()
        .expect("Couldnt get image");

    Ok(())
}

fn get_url(config: &Config) -> String {
    let base = "https://cataas.com/cat";
    let mut params: Vec<String> = Vec::new();
    let mut path: Vec<String> = Vec::new();

    if let Some(tags) = &config.tags {
        if !tags.is_empty() {
            if tags.len() != 1 {
                path.push(format!("/{}", tags.join(",")));
            } else {
                path.push(format!("/{}", tags[0]));
            }
        }
    }
    if let Some(says) = &config.says {
        path.push(format!("says/{}", says.sentence));
        if let Some(size) = says.size {
            params.push(format!("fontSize={size}"));
        }
        if let Some(color) = &says.color {
            params.push(format!("fontColor={color}"));
        }
    }
    if let Some(filter) = &config.filter {
        params.push(format!("filter={filter}"));
    }
    if let Some(dimensions) = &config.dimensions {
        params.push(format!(
            "height={}&width={}",
            dimensions.height, dimensions.width
        ));
    }

    let url = format!("{}{}?{}", base, path.join("/"), params.join("&"));
    url
}

fn get_config() -> PathBuf {
    if let Some(home) = home_dir() {
        let mut config_location = home.to_str().unwrap().to_owned();
        config_location.push_str("/.config/kitty-cat/config.toml");
        PathBuf::from(config_location)
    } else {
        panic!("Cant find config file!");
    }
}

fn get_image_location() -> String {
    if let Some(home) = home_dir() {
        let mut image_location = home.to_str().unwrap().to_owned();
        image_location.push_str("/.local/share/kitty-cat/cat.png");
        image_location
    } else {
        panic!("Cant get image location")
    }
}
