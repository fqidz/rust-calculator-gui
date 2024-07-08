mod lexer;
mod parser;

use lexer::{tokenizer, Token};
use parser::parse;
use slint::{SharedString, Weak};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;

    ui.on_input_entered({
        let ui_handle: Weak<AppWindow> = ui.as_weak();
        move |string: SharedString| {
            let ui: AppWindow = ui_handle.unwrap();

            // Program
            let mut errors: Vec<String> = Vec::new();

            let mut infix: Vec<Token> = Vec::new();
            match tokenizer(string.into()) {
                Ok(val) => infix = val,
                Err(e) => errors.push(e),
            }

            let mut result: f32 = 0.0;
            match parse(infix) {
                Ok(val) => result = val,
                Err(e) => errors.push(e),
            }

            ui.set_result(result.to_string().into());

            if !errors.is_empty() {
                // let errors_string: String = errors.join("\n");
                // ui.set_result(errors_string.into());
                ui.set_result("".to_string().into());
                ui.set_error(errors[0].to_owned().into());
            }
        }
    });
    ui.on_button_pressed({
        let ui_handle: Weak<AppWindow> = ui.as_weak();
        move |string: SharedString| {
            let ui: AppWindow = ui_handle.unwrap();

            // Program
            let mut text: String = ui.get_line_edit_text().to_string();
            text.push_str(&string.to_string());
            // println!("{:?}", text);
            ui.set_line_edit_text(text.into());
        }
    });
    ui.on_backspace({
        let ui_handle: Weak<AppWindow> = ui.as_weak();
        move |string: SharedString| {
            let ui: AppWindow = ui_handle.unwrap();

            // Program
            let mut original_text: std::str::Chars = string.chars();
            original_text.next_back();
            ui.set_result(original_text.collect::<String>().into());
        }
    });
    ui.run()
}

// fn main() {
//     let infix: Vec<Token> = tokenizer("3+4*2/(1-5*(7+9+3))*2*3".to_string()).unwrap();
//     println!("{:?}", parser::parse(infix))
// }