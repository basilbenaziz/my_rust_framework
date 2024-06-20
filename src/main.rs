// src/main.rs

mod core;

use core::{Color, Element, Screen, Button, Text, Image};

fn main() {
    Screen {
        elements: vec![
            Element::Text(Text {
                text: "Hello World".to_string(),
                font_size: 20,
                color: Color::Red,
            }),
            Element::Text(Text {
                text: "Welcome to Rust".to_string(),
                font_size: 18,
                color: Color::Green,
            }),
            Element::Button(Button {
                text: "Click Me".to_string(),
                background_color: Color::Red,
                width: 500,
                height: 50,
                margin: 0,
                padding: 0,
            }
            .padding(10)
            .margin(10)),
            Element::Button(Button {
                text: "Submit".to_string(),
                background_color: Color::Green,
                width: 150,
                height: 40,
                margin: 0,
                padding: 0,
            }
            .padding(5)
            .margin(5)),
            Element::Image(Image {
                src: "path/to/image.png".to_string(),
                width: 200,
                height: 200,
                alt: "An example image".to_string(),
            }),
        ],
        width: 1024,
        height: 768,
        padding: 10,
        background_color: Color::Blue,
    }.save_files();
}
