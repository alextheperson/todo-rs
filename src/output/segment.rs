use crate::output::Render;
use crate::output::RenderFormat;
use crate::output::color;
use crate::output::style;

#[derive(Debug, Clone)]
pub struct OutputSegment {
    color: color::Color,
    style: style::Style,
    content: String,
}

impl OutputSegment {
    pub fn new(content: &str, color: color::Color, style: style::Style) -> OutputSegment {
        OutputSegment {
            content: content.to_string(),
            color: color,
            style: style,
        }
    }
}

impl Render for OutputSegment {
    fn render(self, format: &RenderFormat) -> String {
        match format {
            RenderFormat::ANSI | RenderFormat::Plain => format!(
                "{style}{color}{content}{nostyle}{nocolor}",
                style = self.style.render(&format),
                color = self.color.render(&format),
                content = self.content,
                nostyle = style::Style::normal().render(&format),
                nocolor = color::Color::Default.render(&format)
            ),
            RenderFormat::HTML => format!(
                "<span style=\"{style}{color}\">{content}</span>",
                style = self.style.render(&format),
                color = self.color.render(&format),
                content = self.content
            ),
            RenderFormat::HtmlClass => format!(
                "<span class=\"{style}{color}\">{content}</span>",
                style = self.style.render(&format),
                color = self.color.render(&format),
                content = self.content
            ),
            RenderFormat::Pango => format!(
                "<span {style}{color}>{content}</span>",
                style = self.style.render(&format),
                color = self.color.render(&format),
                content = self.content
            ),
        }
    }
}
