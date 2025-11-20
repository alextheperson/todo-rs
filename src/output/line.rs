use crate::output::Render;
use crate::output::RenderFormat;
use crate::output::segment::OutputSegment;

#[derive(Debug, Clone)]
pub struct OutputLine {
    content: Vec<OutputSegment>,
}

impl OutputLine {
    pub fn new() -> OutputLine {
        OutputLine { content: vec![] }
    }
    pub fn add(&mut self, segment: OutputSegment) -> &mut OutputLine {
        self.content.push(segment);

        self
    }

    pub fn newline(format: &RenderFormat) -> &'static str {
        match format {
            RenderFormat::Plain | RenderFormat::ANSI | RenderFormat::Pango => return "\n",
            RenderFormat::HTML | RenderFormat::HtmlClass => return "<br>",
        }
    }
}

impl Render for OutputLine {
    fn render(self, format: &RenderFormat) -> String {
        let mut output = String::new();

        for segment in self.content {
            output += &segment.render(&format);
        }

        output
    }
}
