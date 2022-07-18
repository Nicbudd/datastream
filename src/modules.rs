use chrono::{DateTime, Utc};
use ureq;

pub struct TextBit {
    pub text: String,
    pub len: usize, // should be a constant! do not determine it dynamically with text.len() or something.
    pub style: Style,
}

impl TextBit {
    pub fn check_len(&self) -> bool {
        self.text.len() == self.len
    }

    pub fn verify(&self, len: usize) -> bool {
        self.text.len() == len
    }
}

pub enum Style {
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


// MODULES

pub fn timeAndDate() -> TextBit {
    let now = Utc::now();
    let format = format!("{}", now.format("%y%m%d"));

    TextBit {text: format, len: 6, style: Style::Bold}
}

fn getWx(station: &str, network: &str) -> Result<String, ()> {
    let url = &format!("http://mesonet.agron.iastate.edu/json/current.py?station={station}&network={network}");

    ureq::get(url)
        .call().map_err(|_| ())?
        .into_string().map_err(|_| ())
}

pub fn wx() -> TextBit {
    match getWx("PSM", "NH_ASOS") {
        Ok(s) => {
            println!("{}", s);
            TextBit{text: s, len: 11, style: Style::Red} 
        },
        Err(_) => TextBit{text: String::from("UNAVAILABLE"), len: 11, style: Style::Red},
    }
}
