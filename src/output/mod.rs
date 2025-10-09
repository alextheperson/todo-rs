pub mod buffer;
pub mod color;
pub mod line;
pub mod segment;
pub mod style;

#[derive(Debug, Clone)]
pub enum RenderFormat {
    Plain,
    ANSI,
    HTML,
    HtmlClass,
    Pango,
}

pub trait Render {
    fn render(self, format: &RenderFormat) -> String;
}
