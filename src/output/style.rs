use crate::Render;
use crate::output::RenderFormat;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    bright: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    inverse: bool,
    hidden: bool,
    strikethrough: bool,
}

impl Style {
    pub fn new() -> Style {
        Style {
            bright: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            inverse: false,
            hidden: false,
            strikethrough: false,
        }
    }

    pub fn normal() -> Style {
        Style::new()
    }

    pub fn bright(&mut self) -> &mut Style {
        self.bright = true;
        self
    }
    pub fn dim(&mut self) -> &mut Style {
        self.dim = true;
        self
    }
    pub fn italic(&mut self) -> &mut Style {
        self.italic = true;
        self
    }
    pub fn underline(&mut self) -> &mut Style {
        self.underline = true;
        self
    }
    pub fn blink(&mut self) -> &mut Style {
        self.blink = true;
        self
    }
    pub fn inverse(&mut self) -> &mut Style {
        self.inverse = true;
        self
    }
    pub fn hidden(&mut self) -> &mut Style {
        self.hidden = true;
        self
    }
    pub fn strikethrough(&mut self) -> &mut Style {
        self.strikethrough = true;
        self
    }
}

impl Render for Style {
    fn render(self, format: &RenderFormat) -> String {
        match format {
            RenderFormat::Plain => "".to_string(),
            RenderFormat::ANSI => {
                let mut output = String::new();

                // If nothing, select normal
                if !self.bright
                    && !self.dim
                    && !self.italic
                    && !self.underline
                    && !self.blink
                    && !self.inverse
                    && !self.hidden
                    && !self.strikethrough
                {
                    return "\u{001b}[0m".to_string();
                }

                if self.bright {
                    output += &format!("\u{001b}[{code}m", code = 1);
                }
                if self.dim {
                    output += &format!("\u{001b}[{code}m", code = 2);
                }
                if self.italic {
                    output += &format!("\u{001b}[{code}m", code = 3);
                }
                if self.underline {
                    output += &format!("\u{001b}[{code}m", code = 4);
                }
                if self.blink {
                    output += &format!("\u{001b}[{code}m", code = 5);
                }
                if self.inverse {
                    output += &format!("\u{001b}[{code}m", code = 7);
                }
                if self.hidden {
                    output += &format!("\u{001b}[{code}m", code = 8);
                }
                if self.strikethrough {
                    output += &format!("\u{001b}[{code}m", code = 9);
                }

                output
            }
            RenderFormat::HTML => {
                let mut output = String::new();

                if self.bright {
                    output += "filter: brightness(1.5);";
                }
                if self.dim {
                    output += "filter: brightness(0.5);";
                }
                if self.italic {
                    output += "font-style: italic;";
                }
                if self.underline {
                    output += "text-decoration: underline;";
                }
                if self.blink {
                    output += "text-decoration: blink;";
                }
                if self.inverse {
                    output += "filter: invert(1);";
                }
                if self.hidden {
                    output += "opacity: 0.2;";
                }
                if self.strikethrough {
                    output += "text-decoration: line-through;";
                }

                output
            }
            RenderFormat::HtmlClass => {
                let mut output = String::new();

                if self.bright {
                    output += "bright ";
                }
                if self.dim {
                    output += "dim ";
                }
                if self.italic {
                    output += "italic ";
                }
                if self.underline {
                    output += "underline ";
                }
                if self.blink {
                    output += "blink ";
                }
                if self.inverse {
                    output += "inverse ";
                }
                if self.hidden {
                    output += "hidden ";
                }
                if self.strikethrough {
                    output += "strikethrough ";
                }

                output
            }
            RenderFormat::JSON => "".to_string(),
        }
    }
}
