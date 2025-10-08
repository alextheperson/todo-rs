use crate::Render;
use crate::output::RenderFormat;

#[derive(Debug, Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    Default,
}

impl Render for Color {
    fn render(self, format: &RenderFormat) -> String {
        match format {
            RenderFormat::Plain => "".to_string(),
            RenderFormat::ANSI => {
                let code = match self {
                    Color::Black => 30,
                    Color::Red => 31,
                    Color::Green => 32,
                    Color::Yellow => 33,
                    Color::Blue => 34,
                    Color::Magenta => 35,
                    Color::Cyan => 36,
                    Color::White => 37,
                    _ => 39,
                };
                format!("\u{001b}[{code}m")
            }
            RenderFormat::HTML => {
                let color = match self {
                    Color::Black => "black",
                    Color::Red => "red",
                    Color::Green => "green",
                    Color::Yellow => "yellow",
                    Color::Blue => "blue",
                    Color::Magenta => "magenta",
                    Color::Cyan => "cyan",
                    Color::White => "white",
                    _ => "currentColor",
                };
                format!("color:{color};")
            }
            RenderFormat::HtmlClass => {
                let color = match self {
                    Color::Black => "black",
                    Color::Red => "red",
                    Color::Green => "green",
                    Color::Yellow => "yellow",
                    Color::Blue => "blue",
                    Color::Magenta => "magenta",
                    Color::Cyan => "cyan",
                    Color::White => "white",
                    _ => "",
                };
                color.to_string()
            }
            RenderFormat::JSON => "".to_string(),
        }
    }
}
