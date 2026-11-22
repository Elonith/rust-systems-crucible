use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let tot_items: i8 = env::args().count() as i8;

    const WRONG_USAGE: u8 = 1;
    const UNKNOWN_UNIT: u8 = 2;

    if tot_items != 4 {
        eprintln!("Error: Wrong number of arguments.");
        ExitCode::from(WRONG_USAGE)
    } else {
        let input = get_input(1);
        let from_unit = get_input(2).to_lowercase();
        let to_unit = get_input(3);

        // Err handling is still to be implemented robustly with exit code Wrong usage
        let parse_result = input.parse::<f64>();

        if parse_result.is_err() {
            eprintln!("Failed to parse: `{input}` is not a valid input.");
            ExitCode::from(WRONG_USAGE)
        } else {
            let input: f64 = input.trim().parse().unwrap();

            //println!("These are the elements you inputed: \n{input}\n{from_unit}\n{to_unit}");

            if from_unit.to_lowercase() == to_unit.to_lowercase() {
                println!("This is the converted value: {input}{}", to_unit);
                ExitCode::SUCCESS
            } else {
                let output: (f64, i32, i32, bool) = get_output(input, &from_unit, &to_unit);

                if output.1 == 0 {
                    eprintln!("Error: Unknown unit `{from_unit}`.");
                    ExitCode::from(UNKNOWN_UNIT)
                } else if output.2 == 0 {
                    eprintln!("Error: Unknown unit `{to_unit}`.");
                    ExitCode::from(UNKNOWN_UNIT)
                } else if !output.3 {
                    eprintln!("Error: Units from different families.");
                    ExitCode::from(WRONG_USAGE)
                } else {
                    println!("This is the converted value: {:.2}{}", output.0, to_unit);

                    ExitCode::SUCCESS
                }
            }
        }
    }
}

fn get_output(val: f64, from_unit: &str, to_unit: &str) -> (f64, i32, i32, bool) {
    const DATA: [(&str, &str, f64, f64); 25] = [
        ("f", "c", 0.555556, -17.78),
        ("c", "f", 1.8, 32.0),
        ("mi", "km", 1.60934, 0.0),
        ("km", "mi", 0.621371, 0.0),
        ("kg", "lb", 2.20462, 0.0),
        ("ft", "m", 0.3048, 0.0),
        ("m", "ft", 3.28084, 0.0),
        ("in", "cm", 2.54, 0.0),
        ("cm", "in", 0.393701, 0.0),
        ("yd", "m", 0.9144, 0.0),
        ("m", "yd", 1.09361, 0.0),
        ("c", "k", 1.0, 273.15),
        ("k", "c", 1.0, -273.15),
        ("oz", "g", 28.3495, 0.0),
        ("g", "oz", 0.035274, 0.0),
        ("b", "kb", 0.001, 0.0),
        ("kb", "b", 1000.0, 0.0),
        ("mb", "kb", 1000.0, 0.0),
        ("kb", "mb", 0.001, 0.0),
        ("b", "kib", 0.0009765625, 0.0),
        ("kib", "b", 1024.0, 0.0),
        ("gal", "l", 3.78541, 0.0),
        ("l", "gal", 0.264172, 0.0),
        ("mph", "km/h", 1.60934, 0.0),
        ("km/h", "mph", 0.621371, 0.0),
    ];

    let mut matched_from_unit: i32 = 0;
    let mut matched_to_unit: i32 = 0;
    let mut output: f64 = val;
    let mut converted: bool = false;

    for item in DATA {
        if item.0 == from_unit {
            matched_from_unit += 1;
        }
        if item.1 == to_unit.to_lowercase() {
            matched_to_unit += 1;
        }

        if item.0 == from_unit && item.1 == to_unit.to_lowercase() {
            output = val * item.2 + item.3;
            converted = true;
        }
    }

    (output, matched_from_unit, matched_to_unit, converted)

    // Error handling will be here for various scenarios
}

fn get_input(term: usize) -> String {
    env::args().nth(term).unwrap()
}
