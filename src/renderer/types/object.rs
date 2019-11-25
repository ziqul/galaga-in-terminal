use std::error::Error;
use std::env;
use std::fs;

use yaml_rust::YamlLoader;

pub struct Object {
    pub x: usize,
    pub y: usize,
    pub null_char: char,
    pub data: Vec<Vec<char>>,
}

impl Object {
    pub fn from_file(
        filepath: &str
    ) ->
        Result<Object, Box<dyn Error>>
    {
        let err_msg =
            "renderer::types::Object::from_file(): ".to_owned() +
            "Cannot open '" + filepath +
            "' from '" + env::current_dir()?.to_str().unwrap() + "'.";

        let contents =
            fs::read_to_string(filepath)
                .expect(&err_msg);

        let doc = YamlLoader::load_from_str(&contents)?;

        let mut data_vec = Vec::<Vec<char>>::new();
        let data_str = doc[0]["data"].as_str().unwrap();

        for part in data_str.split('\n') {
            let line: Vec<char> = part.chars().collect();

            data_vec.push(line);
        }

        Ok(Object {
            x: 0,
            y: 0,
            null_char: doc[0]["null_char"].as_str().unwrap().to_owned().remove(0),
            data: data_vec
        })
    }
}
