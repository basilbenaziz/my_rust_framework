use crate::core::color::Color;

pub struct Input {
    pub input_type: String,
    pub placeholder: String,
    pub value: String,
    pub width: u32,
    pub height: u32,
    pub margin: u32,
    pub padding: u32,
}

impl Input {
    pub fn new(input_type: &str, placeholder: &str, value: &str, width: u32, height: u32) -> Self {
        Self {
            input_type: input_type.to_string(),
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
            "<input type=\"{}\" placeholder=\"{}\" value=\"{}\" class=\"input-{}-{}\" />",
            self.input_type, self.placeholder, self.value, self.width, self.height
        )
    }

    pub fn render_css(&self) -> String {
        format!(
            ".input-{}-{} {{
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
