pub mod modules;
use crate::modules::Style;

enum CallType {
    Bash,
    Discord,
}

fn main_but_cooler(return_type: CallType) {
    let mut getModules = [
        (modules::timeAndDate(), 6),
        (modules::wx(), 11),
    ];

    

    //let mut return_string = String::new();

    let mut return_string = getModules
                            .into_iter()
                            .map(|(x, y)| if x.verify(y) {
                                x
                            } else {
                                modules::TextBit {
                                    text: String::from('X').repeat(y),
                                    len: y,
                                    style: Style::White,
                                }
                            })
                            .map(|x| match return_type {
                                CallType::Bash => {
                                    match x.style {
                                        Style::White => x.text,
                                        Style::Red => format!("\x1b[31m{}\x1b[0m", x.text),
                                        Style::Green => format!("\x1b[32m{}\x1b[0m", x.text),
                                        Style::Blue => format!("\x1b[33m{}\x1b[0m", x.text),
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
