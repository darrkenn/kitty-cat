use std::{env::home_dir, fs, path::PathBuf};

use crate::Config;

pub fn get_url(config: &Config) -> String {
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

pub fn get_config() -> PathBuf {
    if let Some(home) = home_dir() {
        let mut config_location = home.to_str().unwrap().to_owned();
        config_location.push_str("/.config/kitty-cat/config.toml");
        PathBuf::from(config_location)
    } else {
        panic!("Cant find config file!");
    }
}

pub fn get_local_data() -> String {
    if let Some(home) = home_dir() {
        let mut image_location = home.to_str().unwrap().to_owned();
        image_location.push_str("/.local/share/kitty-cat");
        image_location
    } else {
        panic!("Cant get image location")
    }
}

pub fn get_cached_images(cache_location: String) -> Vec<String> {
    let mut images: Vec<String> = Vec::new();

    if let Ok(files) = fs::read_dir(cache_location) {
        for file in files.flatten() {
            if let Some(extension) = file.path().extension() {
                if extension == "png" {
                    if let Some(name) = file.file_name().to_str() {
                        images.push(name.to_string());
                    }
                }
            }
        }
    };
    images
}
