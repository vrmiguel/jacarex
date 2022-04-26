pub enum Text {
    Word(String),
    Line(String),
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        match self {
            Self::Word(word) => word,
            Self::Line(line) => line,
        }
    }
}

impl Text {
    pub fn is_line(&self) -> bool {
        matches!(self, Text::Word(_))
    }

    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
