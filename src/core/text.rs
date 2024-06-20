use crate::core::color::Color;

pub struct Text {
    pub text: String,
    pub font_size: u32,
    pub color: Color,
}

impl Text {
    pub fn new(text: &str, font_size: u32, color: Color) -> Self {
        Self {
            text: text.to_string(),
            font_size,
            color,
        }
    }

    pub fn render_html(&self) -> String {
        format!("<p class=\"text-{}\">{}</p>", self.font_size, self.text)
    }

    pub fn render_css(&self) -> String {
        format!(
            ".text-{} {{
                font-size: {}px;
                color: {};
            }}",
            self.font_size, self.font_size, self.color.to_css()
        )
    }
}
