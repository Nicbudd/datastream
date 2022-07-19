use chrono::{DateTime, Utc, Local, Timelike};
use regex::Regex;
use serde::{Deserialize, Serialize};

pub struct TextBit {
    pub text: String,
    pub style: Style,
}

impl TextBit {
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

pub fn date() -> TextBit {
    let now = Utc::now();
    let format = format!("{}", now.format("%y%m%d"));

    TextBit {text: format, style: Style::Bold}
}

pub fn time(standard: &str, daylight: &str) -> TextBit {

    let loc = Local::now();
    let utc = Utc::now();

    let loc_fmt = loc.format("%H%M%S");
    let utc_fmt = utc.format("%H%M");

    let tz = format!("{}", loc.format("%z"));

    let daylight_or_standard = {
        if standard.eq(daylight) {
            "L"
        } else if standard.eq(&tz) {
            "S"
        } else if daylight.eq(&tz) {
            "D"
        } else {
            "?"
        }
    };

    let swatch_internet_time: f32 = ((utc.minute() * 60) + ((utc.hour() + 1) * 3600)) as f32 / 86.4;

    let format = format!("{}{}{}Z@{:03}", loc_fmt, daylight_or_standard, utc_fmt, swatch_internet_time as u32);

    TextBit {text: format, style: Style::Bold}
}



//    (( [UTC+1 minutes] * 60) + ( [UTC+1 hours] * 3600)) / 86.4

#[derive(Serialize, Deserialize)]
struct WxResponse {
    server_gentime: String,
    id: String,
    network: String,
    last_ob: Observation
}  

#[derive(Serialize, Deserialize)]
struct Observation {
    local_valid: String,
    utc_valid: String,
    airtempF: f32,
    max_dayairtempF: f32,
    min_dayairtempF: f32,
    dewpointtempF: f32,
    windspeedkt: f32,
    winddirectiondeg: f32,
    altimeterin: f32,
    mslpmb: f32,
    skycovercode: Vec<String>,
    skylevelft: Vec<f32>,
    visibilitymile: f32,
    raw: String,
    presentwx: Vec<String>
}  

fn get_format_wx(station: &str, network: &str) -> Result<String, ()> {
    let url = &format!("http://mesonet.agron.iastate.edu/json/current.py?station={station}&network={network}");

    let json_string: String = ureq::get(url)
        .call()
        .map_err(|_| ())?
        .into_string()
        .map_err(|_| ())?;
    
    let resp: WxResponse = serde_json::from_str(&json_string) // TODO: Why is this erroring out
                            .map_err(|_| ())?;

    let zulu_time_re = Regex::new(r"\d{4}-\d{2}-\d{2}T(\d{2}):(\d{2}):\d{2}Z")
                            .map_err(|_| ())?;

    let loc_valid_caps = zulu_time_re.captures_iter(&resp.last_ob.utc_valid).next().ok_or(())?;

    let loc_valid_formatted = format!("{}{}Z",
                                loc_valid_caps.get(1).ok_or(())?.as_str(),
                                loc_valid_caps.get(2).ok_or(())?.as_str());
    
    let return_string = format!(
        "{}{}", resp.id, loc_valid_formatted);

    Ok(return_string)
}


pub fn wx(station: &str, network: &str) -> TextBit {

    match get_format_wx(station, network) {
        Ok(s) => {
            TextBit{text: s, style: Style::Red} 
        },
        Err(_) => {
            TextBit{text: String::from("UNAVAILABLE"), style: Style::Red}
        }
    }
}
