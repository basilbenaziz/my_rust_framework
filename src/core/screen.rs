use crate::core::element::Element;
use std::fs::File;
use std::io::Write;

pub struct Screen {
    pub elements: Vec<Element>,
    pub width: u32,
    pub height: u32,
    pub padding: u32,
    pub background_color: crate::core::color::Color,
}

impl Screen {
    pub fn render_html(&self) -> String {
        let elements_html: String = self.elements.iter().map(|e| e.render_html()).collect();
        format!(
            "<!DOCTYPE html>
            <html>
            <head>
                <link rel=\"stylesheet\" type=\"text/css\" href=\"style.css\">
            </head>
            <body>
                <div class=\"screen\">
                    {}
                </div>
            </body>
            </html>",
            elements_html
        )
    }

    pub fn render_css(&self) -> String {
        let elements_css: String = self.elements.iter().map(|e| e.render_css()).collect();
        format!(
            ".screen {{
                width: {}px;
                height: {}px;
                padding: {}px;
                background-color: {};
            }}
            {}",
            self.width, self.height, self.padding, self.background_color.to_css(), elements_css
        )
    }

    pub fn save_files(&self) {
        let html_content = self.render_html();
        let css_content = self.render_css();

        let mut html_file = File::create("output.html").expect("Could not create HTML file");
        html_file.write_all(html_content.as_bytes()).expect("Could not write to HTML file");

        let mut css_file = File::create("style.css").expect("Could not create CSS file");
        css_file.write_all(css_content.as_bytes()).expect("Could not write to CSS file");
    }
}
