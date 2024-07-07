mod lexer;
mod parser;

// use std::io::{Error, ErrorKind};
use lexer::{tokenizer, Token};
// use slint::{SharedString, Weak};

slint::include_modules!();


// fn main() -> Result<(), slint::PlatformError> {
//     let ui: AppWindow = AppWindow::new()?;

//     ui.on_input_entered({
//         let ui_handle: Weak<AppWindow> = ui.as_weak();
//         move |string: SharedString| {
//             let ui: AppWindow = ui_handle.unwrap();
//             let result: String = match parse_input(string) {
//                 Ok(v) => v.to_string(),
//                 Err(err) => err.to_string()
//             };
//             ui.set_result(result.into())
//         }
//     });

//     ui.run()
// }
fn main() {
    let infix: Vec<Token> = tokenizer("1+2*2+2*3".to_string()).unwrap();
    println!("{:?}", parser::parse(infix))
}