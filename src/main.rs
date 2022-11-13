pub mod modules;
use crate::modules::Style;

enum CallType {
    Bash,
    Discord,
}

fn main_but_cooler(return_type: CallType) {
    let get_modules = [
        (modules::date(), 9),
        (modules::time("-0500", "-0400"), 16),
        (modules::wx("PSM", "NH_ASOS"), 62),
        (modules::wx("WMSA", "MY__ASOS"), 62),
    ];

    

    let return_string = get_modules
                            .into_iter()
                            .map(|(x, y)| if x.verify(y) {
                                x
                            } else {
                                modules::TextBit {
                                    text: String::from('X').repeat(y),
                                    style: Style::White,
                                }
                            })
                            .map(|x| match return_type {
                                CallType::Bash => {
                                    match x.style {
                                        Style::White => x.text,
                                        Style::Red => format!("\x1b[31m{}\x1b[0m", x.text),
                                        Style::Green => format!("\x1b[32m{}\x1b[0m", x.text),
                                        Style::Blue => format!("\x1b[34m{}\x1b[0m", x.text),
                                        Style::Purple => format!("\x1b[35m{}\x1b[0m", x.text),
                                        Style::RedBG => format!("\x1b[41;30m{}\x1b[0m", x.text),
                                        Style::GreenBG => format!("\x1b[42;30m{}\x1b[0m", x.text),
                                        Style::BlueBG => format!("\x1b[44;30m{}\x1b[0m", x.text),
                                        Style::PurpleBG => format!("\x1b[45;30m{}\x1b[0m", x.text),
                                        Style::Bold => format!("\x1b[1m{}\x1b[0m", x.text),
                                        _ => x.text,
                                    }
                                },
                                _ => x.text, 
                            })
                            .collect::<Vec<String>>()
                            .join("-");

    
    

    println!("{return_string}");
}

fn main() {
    main_but_cooler(CallType::Bash);
}
