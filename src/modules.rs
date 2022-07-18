use chrono::{DateTime, Utc};

struct TextBit {
    text: String,
    len: usize, // should be a constant! do not determine it dynamically with text.len() or something.
    style: Style,
}

enum Style {
    White,
    Red,
    Green,
    Blue,
    Purple,
    WhiteBG,
    RedBG,
    GreenBG,
    BlueBG,
    PurpleBG,
    Bold,
}


impl TextBit {
    pub fn check_len(&self) -> bool {
        self.text.len() == self.len
    }

    pub fn verify(&self, len: usize) -> Result<String, ()> {
        match self.text.len() == len {
            true => Ok(self.text),
            false => Err(()),
        }
    }
}


// MODULES

pub fn timeAndDate() -> TextBit {
    let now = Utc::now();
    let format = format!("{}", now.format("%y%m%d"));

    TextBit {text: format, len: 6, style: Style::Bold}
}

