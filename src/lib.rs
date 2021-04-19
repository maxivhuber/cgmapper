use serde_json::Value;
use std::{fs::File, io::Read, process::exit};

#[derive(Debug)]
pub struct Color {
    rgb: (u8, u8, u8),
    hex: String,
}

impl Color {
    pub fn new(vertex: &u64, file: &mut File) -> Self {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let json: Vec<Value> = serde_json::from_str(&contents).unwrap();
        let data = json
            .iter()
            .map(|f| {
                (
                    f[0].as_array()
                        .unwrap()
                        .iter()
                        .map(|f| f.as_u64().unwrap())
                        .collect::<Vec<u64>>(),
                    f[1].as_str().unwrap(),
                )
            })
            .find(|(x, _y)| x.contains(vertex));

        if let Some((_nodes, color)) = data {
            let c1 = &color[1..];
            let c2 = (
                u8::from_str_radix(&color[1..3], 16).unwrap(),
                u8::from_str_radix(&color[3..5], 16).unwrap(),
                u8::from_str_radix(&color[5..], 16).unwrap(),
            );
            Self {
                rgb: c2,
                hex: c1.to_string(),
            }
        } else {
            //no such vertex
            exit(-3)
        }
    }
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        self.rgb
    }
    pub fn get_hex(&self) -> &str {
        &self.hex
    }
}
