pub struct Image {
    pub src: String,
    pub width: u32,
    pub height: u32,
    pub alt: String,
}

impl Image {
    pub fn render_html(&self) -> String {
        format!("<img class=\"image-{}-{}\" src=\"{}\" alt=\"{}\" />", self.width, self.height, self.src, self.alt)
    }

    pub fn render_css(&self) -> String {
        format!(
            ".image-{}-{} {{
                width: {}px;
                height: {}px;
            }}",
            self.width, self.height, self.width, self.height
        )
    }
}
