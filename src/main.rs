mod get;

use std::{
    error::Error,
    fs::{self, File, remove_file},
    io::copy,
    process::Command,
};

use crate::get::*;
use chrono::Local;
use rand::{rng, seq::IndexedRandom};
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Dimensions {
    height: u32,
    width: u32,
}

#[derive(Deserialize)]
pub struct Says {
    sentence: String,
    size: Option<u8>,
    color: Option<String>,
}

#[derive(Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Deserialize)]
pub enum Filter {
    Blur,
    Mono,
    Negate,
    Custom,
}

#[derive(Deserialize)]
pub struct Config {
    offline: bool,
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
            offline: false,
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

    let cache_folder = format!("{}/cache", get_local_data());

    if !config.offline {
        let now = Local::now();
        let formatted_time = now.format("%H:%M:%S");
        let image_name: String = format!("cat_{formatted_time}.png");
        let image_location: String = format!("{}/{}", get_local_data(), image_name);

        //Makes request to api
        let url: String = get_url(&config);
        let mut response = get(url)?;

        //Creates the .png file by copying data from the response to a file.
        let mut file = File::create(&image_location).expect("Cant create file");
        copy(&mut response, &mut file).expect("Cant write file");

        load_image(config.alignment_to_string(), image_location.to_owned());

        if config.cache {
            let cached_image = format!("{cache_folder}/{image_name}");
            fs::copy(&image_location, cached_image).expect("Couldnt copy image to cache");
        }
        remove_file(image_location).expect("Couldnt remove file");
    } else {
        let images = get_cached_images(cache_folder.clone());
        let mut rng = rng();
        if let Some(image) = images.choose(&mut rng) {
            let image_location = format!("{cache_folder}/{image}");
            load_image(config.alignment_to_string(), image_location);
        }
    }

    Ok(())
}

fn load_image(alignment: String, location: String) {
    Command::new("kitten")
        .arg("icat")
        .arg("--align")
        .arg(alignment)
        .arg(location)
        .status()
        .expect("Couldnt get image");
}
