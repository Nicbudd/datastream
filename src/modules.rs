use chrono::{Utc, Local, Timelike};
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
    let now = Local::now();
    let mut format = format!("{}{}", now.format("%y%m%d"), now.format("%a"));
    format = format.to_uppercase();
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

#[derive(Serialize, Deserialize, Debug)]
struct WxResponse {
    server_gentime: String,
    id: String,
    network: String,
    last_ob: Observation
}  

#[derive(Serialize, Deserialize, Debug)]
struct Observation {
    local_valid: String,
    utc_valid: String,

    #[serde(rename="airtemp[F]")]
    airtempF: Option<f32>,

    #[serde(rename="max_dayairtemp[F]")]
    max_dayairtempF: Option<f32>,

    #[serde(rename="min_dayairtemp[F]")]
    min_dayairtempF: Option<f32>,

    #[serde(rename="dewpointtemp[F]")]
    dewpointtempF: Option<f32>,

    #[serde(rename="windspeed[kt]")]
    windspeedkt: Option<f32>,

    #[serde(rename="winddirection[deg]")]
    winddirectiondeg: Option<f32>,

    #[serde(rename="altimeter[in]")]
    altimeterin: Option<f32>,

    #[serde(rename="mslp[mb]")]
    mslpmb: Option<f32>,

    #[serde(rename="skycover[code]")]
    skycovercode: Vec<Option<String>>,

    #[serde(rename="skylevel[ft]")]
    skylevelft: Vec<Option<f32>>,

    #[serde(rename="visibility[mile]")]
    visibilitymile: Option<f32>,

    raw: String,

    presentwx: Vec<Option<String>>
}  

fn fToC(f: f32) -> f32 {
    (f - 32.0) * (5.0/9.0)
}

fn format_temp(t: f32) -> String {
    let round = t.round() as i32;

    if round < 0 {
        format!("M{:02}", round.abs())
    } else {
        format!("{:03}", round.abs())
    }
}

fn get_format_wx(station: &str, network: &str) -> Result<String, ()> {
    let url = &format!("http://mesonet.agron.iastate.edu/json/current.py?station={station}&network={network}");

    let json_string: String = ureq::get(url)
        .call()
        .map_err(|_| ())?
        .into_string()
        .map_err(|_| ())?;

    //println!("{json_string}");
    
    let resp: WxResponse = serde_json::from_str(&json_string).unwrap();

    //println!("{:?}", resp);

    let zulu_time_re = Regex::new(r"\d{4}-\d{2}-\d{2}T(\d{2}):(\d{2}):\d{2}Z")
                            .map_err(|_| ())?;

    let loc_valid_caps = zulu_time_re.captures_iter(&resp.last_ob.utc_valid).next().ok_or(())?;

    let loc_valid_formatted = format!("{}{}Z",
                                loc_valid_caps.get(1).ok_or(())?.as_str(),
                                loc_valid_caps.get(2).ok_or(())?.as_str());

    let temp_f = resp.last_ob.airtempF.ok_or(())?;
    let temp_c = fToC(temp_f);

    let dew_f = resp.last_ob.dewpointtempF.ok_or(())?;
    let dew_c = fToC(dew_f);

    let rh = (100.0 * (std::f32::consts::E.powf((17.625 * dew_c)/(243.04+dew_c)) / std::f32::consts::E.powf((17.625 * temp_c)/(243.04+temp_c)))); 
    let formatted_hum = match rh as u32 {
        100 => String::from("SAT"),
        s => format!("{:02}%", s),
    };


    let mut heat_index = -42.379 
        + 2.04901523*temp_f 
        + 10.14333127*rh 
        - 0.22475541*temp_f*rh
        - 0.00683783*temp_f*temp_f
        - 0.05481717*rh*rh
        + 0.00122874*temp_f*temp_f*rh
        + 0.00085282*temp_f*rh*rh
        - 0.00000199*temp_f*temp_f*rh*rh;

    if rh < 13.0 && temp_f > 80.0 {
        heat_index -= ((13.0-rh)/4.0)*(((17.0-((temp_f-95.0).abs()))/17.0).sqrt());
    } else if rh > 85.0 && temp_f >= 80.0 && temp_f <= 87.0  {
        heat_index += ((rh - 85.0) / 10.0) * ((87.0 - temp_f)/5.0);
    }

    let wind_kts = resp.last_ob.windspeedkt.ok_or(())?;
    let format_wind_kts = if (wind_kts.round() as u32) > 99 {
        99
    } else {
        (wind_kts.round() as u32)
    };

    let wind_mph = wind_kts * 1.15078;
    let format_wind_mph = if (wind_mph.round() as u32) > 99 {
        99
    } else {
        (wind_mph.round() as u32)
    };

    let wind_dir = resp.last_ob.winddirectiondeg.ok_or(())?;

    let wind_chill = 35.74 + (0.6215 * temp_f) - (35.75 * wind_mph.powf(0.16)) + (0.4275 * temp_f * wind_mph.powf(0.16));

    let apparent_temp_f = if temp_f >= 80.0 {
        heat_index
    } else if temp_f <= 50.0 {
        wind_chill
    } else {
        temp_f
    };

    let apparent_temp_c = fToC(apparent_temp_f);


    let mslpmb = (resp.last_ob.mslpmb.ok_or(())? as u32) % 1000;

    

    
    let return_string = format!(
        "{}{}/T{}F{}C/D{}F{}C{}/A{}F{}C/{:03}mb/{:02}mph{:03}@{:02}", resp.id, loc_valid_formatted, 
        format_temp(temp_f), format_temp(temp_c), format_temp(dew_f), format_temp(dew_c), formatted_hum, 
        format_temp(apparent_temp_f), format_temp(apparent_temp_c), mslpmb, format_wind_mph, wind_dir, 
        format_wind_kts);

    //println!("{return_string}");

    Ok(return_string)
}


pub fn wx(station: &str, network: &str) -> TextBit {

    match get_format_wx(station, network) {
        Ok(s) => {
            TextBit{text: s, style: Style::White} 
        },
        Err(_) => {
            TextBit{text: String::from("UNAVAILABLE"), style: Style::Red}
        }
    }
}
