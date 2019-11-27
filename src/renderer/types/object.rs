use std::error::Error;
use std::fs;

use yaml_rust::YamlLoader;

pub struct Location {
    pub x: i32,
    pub y: i32,
}

pub struct Representation {
    null_char: char,
    data: Vec<Vec<char>>,
}

impl Representation {
    pub fn new(
        null_char: char,
        data: Vec<Vec<char>>,
    ) ->
        Representation
    {
        Representation {
            null_char: null_char,
            data: data
        }
    }

    pub fn from_file(
        filepath: &str
    ) ->
        Result<Representation, Box<dyn Error>>
    {
        let contents =
            fs::read_to_string(filepath).unwrap();

        let doc =
            &YamlLoader::load_from_str(&contents)?[0];

        let data_str = doc["data"].as_str().unwrap();
        let mut data_vec = Vec::<Vec<char>>::new();

        for part in data_str.split('\n') {
            let line: Vec<char> =
                part.chars().collect();

            data_vec.push(line);
        }

        let null_char_str =
            doc["null_char"].as_str().unwrap();
        let null_char_char =
            null_char_str.chars().next().unwrap();

        Ok(Representation {
            null_char: null_char_char,
            data: data_vec
        })
    }

    pub fn null_char(&self) -> char {
        (*self).null_char
    }

    pub fn data(&self) -> &Vec<Vec<char>> {
        &(*self).data
    }
}
