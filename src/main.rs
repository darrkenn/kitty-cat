mod config;
mod get;

use std::{
    env,
    error::Error,
    fs::{self, File, remove_file},
    io::copy,
    process::{self, Command},
};

use crate::{config::*, get::*};
use ansi_term::Color;
use chrono::Local;
use rand::{rng, seq::IndexedRandom};
use reqwest::blocking::{Response, get};
use serde_json::Value;

const IMAGE_FORMATS: [&str; 3] = ["jpeg", "png", "gif"];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        for arg in args {
            match arg.as_str() {
                "-t" => {
                    get_tags()?;
                }
                _ => {}
            }
        }
    } else {
        //Load config
        let config = get_config();
        let config_string = fs::read_to_string(config).expect("Cant read config file");
        let config: Config =
            toml::from_str((config_string).as_ref()).expect("Cant parse config string");

        let cache_folder = format!("{}/cache", get_local_data());
        if !config.offline {
            //Makes request to api
            let url: String = get_url(&config);
            let response = get(&url)?;

            if let Some(response_type) = response.headers().get("content-type") {
                if let Some(value) = response_type.to_str().unwrap().split('/').nth(1) {
                    if IMAGE_FORMATS.contains(&value) {
                        create_image(
                            response,
                            config.cache,
                            config.alignment_to_string(),
                            cache_folder,
                        );
                    } else {
                        println!("Oh no! Response from the server: ");
                        let error = response.text().unwrap_or("Couldnt get error".to_string());
                        if let Ok(json) = serde_json::from_str::<Value>(&error) {
                            println!(
                                "{}",
                                Color::Red.bold().paint(json["message"].as_str().unwrap())
                            );
                        } else {
                            println!("{}", Color::Red.bold().paint(error))
                        }
                        process::exit(1);
                    }
                }
            }
        } else {
            let images = get_cached_images(cache_folder.clone());
            let mut rng = rng();
            if let Some(image) = images.choose(&mut rng) {
                let image_location = format!("{cache_folder}/{image}");
                load_image(config.alignment_to_string(), image_location);
            }
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

fn create_image(mut response: Response, cache: bool, alignment: String, cache_folder: String) {
    let now = Local::now();
    let formatted_time = now.format("%H:%M:%S");
    let image_name: String = format!("cat_{formatted_time}.png");
    let image_location: String = format!("{}/{}", get_local_data(), image_name);
    let mut file = File::create(&image_location).expect("Cant create file");
    copy(&mut response, &mut file).expect("Cant write file");

    load_image(alignment, image_location.to_owned());

    if cache {
        let cached_image = format!("{cache_folder}/{image_name}");
        fs::copy(&image_location, cached_image).expect("Couldnt copy image to cache");
    }
    remove_file(image_location).expect("Couldnt remove file");
}
