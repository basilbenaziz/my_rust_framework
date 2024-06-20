pub enum Color {
    Red,
    Green,
    Blue,
    // Add more colors as needed
}

impl Color {
    pub fn to_css(&self) -> &str {
        match self {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
            // Add more color mappings as needed
        }
    }
}
