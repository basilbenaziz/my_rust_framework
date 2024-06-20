pub use crate::core::text::Text;
pub use crate::core::button::Button;
pub use crate::core::image::Image;
pub use crate::core::input::Input;
pub use crate::core::textarea::TextArea;
pub use crate::core::checkbox::Checkbox;

pub enum Element {
    Text(Text),
    Button(Button),
    Image(Image),
    Input(Input),
    TextArea(TextArea),
    Checkbox(Checkbox),
}

impl Element {
    pub fn render_html(&self) -> String {
        match self {
            Element::Text(text) => text.render_html(),
            Element::Button(button) => button.render_html(),
            Element::Image(image) => image.render_html(),
            Element::Input(input) => input.render_html(),
            Element::TextArea(textarea) => textarea.render_html(),
            Element::Checkbox(checkbox) => checkbox.render_html(),
        }
    }

    pub fn render_css(&self) -> String {
        match self {
            Element::Text(text) => text.render_css(),
            Element::Button(button) => button.render_css(),
            Element::Image(image) => image.render_css(),
            Element::Input(input) => input.render_css(),
            Element::TextArea(textarea) => textarea.render_css(),
            Element::Checkbox(_checkbox) => {
                // Handle CSS rendering for Checkbox here if needed
                String::new() // Placeholder for now
            }
        }
    }
}
