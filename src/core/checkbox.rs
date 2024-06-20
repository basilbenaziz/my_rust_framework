pub struct Checkbox {
    pub label: String,
    pub checked: bool,
}

impl Checkbox {
    pub fn new(label: &str, checked: bool) -> Self {
        Self {
            label: label.to_string(),
            checked,
        }
    }

    pub fn render_html(&self) -> String {
        let checked_attr = if self.checked { "checked" } else { "" };
        format!(
            "<label><input type=\"checkbox\" {}> {}</label>",
            checked_attr, self.label
        )
    }
}
