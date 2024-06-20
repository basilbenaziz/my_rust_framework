mod core;

use core::{Color, Element, Screen};
use core::text::Text;
use core::button::Button;
use core::image::Image;
use core::input::Input;
use core::textarea::TextArea;
use core::checkbox::Checkbox;

fn main() {
    let screen = Screen {
        elements: vec![
            Element::Text(Text::new("Hello World", 20, Color::Red)),

            Element::Text(Text::new("Welcome to Rust", 18, Color::Green)),

            Element::Button(Button::new("Click Me", Color::Red, 500, 50)),

            Element::Button(Button::new("Submit", Color::Green, 150, 40)
                        .padding(5)
                        .margin(5)),

            Element::Image(Image::new("https://example.com/path/to/image.png", 200, 200, "An example image")),

            Element::Input(Input::new("text", "Enter your name", "", 300, 40)),

            Element::TextArea(TextArea::new("Enter your message", "", 300, 100)),

            Element::Checkbox(Checkbox::new("Accept terms and conditions", false)),

        ],
        width: 1024,
        height: 768,
        padding: 10,
        background_color: Color::Blue,
    };

    screen.save_files();
}
