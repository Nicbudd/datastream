pub mod modules;

fn main() {
    let getModules = [
        (modules::timeAndDate(), 6),
    ];
    
    let textFields = getModules
                        .iter()
                        .map(|(x, y)| x.verify(*y).unwrap());
                        
    
    
    let mut return_string = String::new();
    

    println!("\x1b[31mHello, world!\x1b[0m");
}
