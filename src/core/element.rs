use crate::core::button::Button;
use crate::core::image::Image;
use crate::core::text::Text;

pub enum Element {
    Text(Text),
    Button(Button),
    Image(Image),
}

impl Element {
    pub fn render_html(&self) -> String {
        match self {
            Element::Text(t) => t.render_html(),
            Element::Button(b) => b.render_html(),
            Element::Image(i) => i.render_html(),
        }
    }

    pub fn render_css(&self) -> String {
        match self {
            Element::Text(t) => t.render_css(),
            Element::Button(b) => b.render_css(),
            Element::Image(i) => i.render_css(),
        }
    }
}
