use crate::output::Render;
use crate::output::RenderFormat;
use crate::output::line::OutputLine;

impl OutputBuffer {
    pub fn new() -> OutputBuffer {
        OutputBuffer { lines: vec![] }
    }

    pub fn add(&mut self, line: OutputLine) -> &mut OutputBuffer {
        self.lines.push(line);

        self
    }

    pub fn append(&mut self, buffer: OutputBuffer) -> &mut OutputBuffer {
        for line in buffer.lines {
            self.lines.push(line);
        }

        self
    }
}

#[derive(Debug, Clone)]
pub struct OutputBuffer {
    lines: Vec<OutputLine>,
}

impl Render for OutputBuffer {
    fn render(self, format: &RenderFormat) -> String {
        let mut output = String::new();

        for (i, line) in self.lines.clone().into_iter().enumerate() {
            output += &line.render(&format);
            if i < self.lines.len() - 1 {
                output += OutputLine::newline(format);
            }
        }

        output
    }
}
