use crate::core::color::Color;

pub struct TextArea {
    pub placeholder: String,
    pub value: String,
    pub width: u32,
    pub height: u32,
    pub margin: u32,
    pub padding: u32,
}

impl TextArea {
    pub fn new(placeholder: &str, value: &str, width: u32, height: u32) -> Self {
        Self {
            placeholder: placeholder.to_string(),
            value: value.to_string(),
            width,
            height,
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
            "<textarea placeholder=\"{}\" class=\"textarea-{}-{}\">{}</textarea>",
            self.placeholder, self.width, self.height, self.value
        )
    }

    pub fn render_css(&self) -> String {
        format!(
            ".textarea-{}-{} {{
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
