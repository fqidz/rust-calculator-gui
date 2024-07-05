use std::io::{Error, ErrorKind};
use slint::{SharedString, Weak};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;

    ui.on_input_entered({
        let ui_handle: Weak<AppWindow> = ui.as_weak();
        move |string: SharedString| {
            let ui: AppWindow = ui_handle.unwrap();
            let result: String = match parse_input(string) {
                Ok(v) => v.to_string(),
                Err(err) => err.to_string()
            };
            ui.set_result(result.into())
        }
    });

    ui.run()
}


fn parse_input(input: SharedString) -> Result<f32, Error> {
    let symbols: Vec<char> = vec!['+', '-', '/', '*'];

    let string: Vec<char> = input
        .to_string()
        .trim()
        .parse::<String>()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let symbol_indeces: Vec<usize> = string
        .iter()
        .enumerate()
        .filter(|(_, &r)| symbols.contains(&r))
        .map(|(index, _)| index)
        .collect::<Vec<usize>>(); 


    if symbol_indeces.len() == 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "No symbol found"));
    } 
    else if symbol_indeces.len() == 1 {
        let first_part: f32 = match string[0..symbol_indeces[0]]
            .into_iter()
            .collect::<String>()
            .parse::<f32>()
        {
            Ok(val) => val,
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, "First part not found")),
        };
        let second_part: f32 = match string[symbol_indeces[0] + 1..]
            .into_iter()
            .collect::<String>()
            .parse::<f32>()
        {
            Ok(val) => val,
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Second part not found")),
        };
        let answer: f32 = evaluate_expression(first_part, second_part, string[symbol_indeces[0]]);

        // print!("{} {} {} = ", first_part, string[symbol_indeces[0]], second_part);
        return Ok(answer);
    } 
    else {
        return Err(Error::new(ErrorKind::InvalidInput, "More than one symbol found"));
    }
}

fn evaluate_expression(a: f32, b: f32, symbol: char) -> f32 {
    match symbol {
        '+' => return a + b,
        '-' => return a - b,
        '*' => return a * b,
        '/' => return a / b,
        _ => panic!("No symbol passed into function")
    }
}