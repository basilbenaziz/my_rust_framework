use crate::core::color::Color;

pub struct Image {
    pub src: String,
    pub width: u32,
    pub height: u32,
    pub alt: String,
    pub margin: u32,
    pub padding: u32,
}

impl Image {
    pub fn new(src: &str, width: u32, height: u32, alt: &str) -> Self {
        Self {
            src: src.to_string(),
            width,
            height,
            alt: alt.to_string(),
            margin: 0,
            padding: 0,
        }
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }

    pub fn render_html(&self) -> String {
        format!(
            "<img class=\"image-{}-{}\" src=\"{}\" alt=\"{}\" />",
            self.width, self.height, self.src, self.alt
        )
    }

    pub fn render_css(&self) -> String {
        format!(
            ".image-{}-{} {{
                width: {}px;
                height: {}px;
                margin: {}px;
                padding: {}px;
                max-width: 100%;
                box-sizing: border-box;
            }}",
            self.width, self.height, self.width, self.height, self.margin, self.padding
        )
    }
}
