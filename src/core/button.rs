use crate::core::color::Color;

pub struct Button {
    pub text: String,
    pub background_color: Color,
    pub width: u32,
    pub height: u32,
    pub margin: u32,
    pub padding: u32,
}

impl Button {
    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }

    pub fn render_html(&self) -> String {
        format!("<button class=\"button-{}-{}\">{}</button>", self.width, self.height, self.text)
    }

    pub fn render_css(&self) -> String {
        format!(
            ".button-{}-{} {{
                background-color: {};
                width: {}px;
                height: {}px;
                margin: {}px;
                padding: {}px;
            }}",
            self.width, self.height, self.background_color.to_css(), self.width, self.height, self.margin, self.padding
        )
    }
}
