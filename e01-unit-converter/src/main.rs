use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let tot_items: i8 = env::args().count() as i8;

    const WRONG_USAGE: u8 = 1;
    const UNKNOWN_UNIT: u8 = 2;
    const NOT_AVAILABLE: u8 = 3;

    if tot_items != 4 {
        eprintln!("Error: Wrong number of arguments.");
        ExitCode::from(WRONG_USAGE)
    } else {
        let input = get_input(1);
        let from_unit = get_input(2).to_lowercase();
        let to_unit = get_input(3);

        let parse_result = input.parse::<f64>();

        if parse_result.is_err() {
            eprintln!("Failed to parse: `{input}` is not a valid input.");
            ExitCode::from(WRONG_USAGE)
        } else {
            let input: f64 = input.trim().parse().unwrap();

            // converting to the same unit
            if from_unit.to_lowercase() == to_unit.to_lowercase() {
                println!("{:.2}{}", input, to_unit);
                ExitCode::SUCCESS
            } else {
                let output: (f64, i32, i32, bool, bool) = get_output(input, &from_unit, &to_unit);

                if output.1 == 0 {
                    eprintln!("Error: Unknown unit `{from_unit}`.");
                    ExitCode::from(UNKNOWN_UNIT)
                } else if output.2 == 0 {
                    eprintln!("Error: Unknown unit `{to_unit}`.");
                    ExitCode::from(UNKNOWN_UNIT)
                } else if output.4 {
                    eprintln!("Error: Sorry, conversion not yet available.");
                    ExitCode::from(NOT_AVAILABLE)
                } else if !output.4 && !output.3 {
                    eprintln!("Error: Units from different families.");
                    ExitCode::from(WRONG_USAGE)
                } else {
                    println!("{:.2}{}", output.0, to_unit);

                    ExitCode::SUCCESS
                }
            }
        }
    }
}

fn get_output(val: f64, from_unit: &str, to_unit: &str) -> (f64, i32, i32, bool, bool) {
    const DATA: [(&str, &str, f64, f64, &str); 54] = [
        // TEMPERATURE(t)
        //
        ("f", "c", 5.0 / 9.0, -160.0 / 9.0, "t"),
        ("c", "f", 9.0 / 5.0, 32.0, "t"),
        ("k", "c", 1.0, -273.15, "t"),
        ("c", "k", 1.0, 273.15, "t"),
        ("f", "k", 5.0 / 9.0, 45_967.0 / 180.0, "t"),
        ("k", "f", 9.0 / 5.0, -45_967.0 / 100.0, "t"),
        // LENGTH(l)
        //
        ("mi", "km", 25_146.0 / 15_625.0, 0.0, "l"),
        ("km", "mi", 15_625.0 / 25_146.0, 0.0, "l"),
        ("km", "m", 1_000.0, 0.0, "l"),
        ("m", "km", 1.0 / 1_000.0, 0.0, "l"),
        ("km", "ft", 1_250_000.0 / 381.0, 0.0, "l"),
        ("ft", "km", 381.0 / 1_250_000.0, 0.0, "l"),
        ("ft", "m", 381.0 / 1_250.0, 0.0, "l"),
        ("m", "ft", 1_250.0 / 381.0, 0.0, "l"),
        ("in", "cm", 127.0 / 50.0, 0.0, "l"),
        ("cm", "in", 50.0 / 127.0, 0.0, "l"),
        ("yd", "m", 1_143.0 / 1_250.0, 0.0, "l"),
        ("m", "yd", 1_250.0 / 1_143.0, 0.0, "l"),
        // MASS / WEIGHT(m)
        //
        ("kg", "lb", 100_000_000.0 / 45_359_237.0, 0.0, "m"),
        ("lb", "kg", 45_359_237.0 / 100_000_000.0, 0.0, "m"),
        ("kg", "g", 1_000.0, 0.0, "m"),
        ("g", "kg", 1.0 / 1_000.0, 0.0, "m"),
        ("oz", "g", 45_359_237.0 / 1_600_000.0, 0.0, "m"),
        ("g", "oz", 1_600_000.0 / 45_359_237.0, 0.0, "m"),
        // DIGITAL STORAGE: DECIMAL UNITS(d)
        //
        ("b", "kb", 1.0 / 1_000.0, 0.0, "d"),
        ("kb", "b", 1_000.0, 0.0, "d"),
        ("kb", "mb", 1.0 / 1_000.0, 0.0, "d"),
        ("mb", "kb", 1_000.0, 0.0, "d"),
        ("mb", "gb", 1.0 / 1_000.0, 0.0, "d"),
        ("gb", "mb", 1_000.0, 0.0, "d"),
        ("kb", "gb", 1.0 / 1_000_000.0, 0.0, "d"),
        ("gb", "kb", 1_000_000.0, 0.0, "d"),
        ("b", "gb", 1.0 / 1_000_000_000.0, 0.0, "d"),
        ("gb", "b", 1_000_000_000.0, 0.0, "d"),
        // DIGITAL STORAGE: BINARY UNITS
        //
        ("b", "kib", 1.0 / 1_024.0, 0.0, "d"),
        ("kib", "b", 1_024.0, 0.0, "d"),
        ("kib", "mib", 1.0 / 1_024.0, 0.0, "d"),
        ("mib", "kib", 1_024.0, 0.0, "d"),
        ("mib", "gib", 1.0 / 1_024.0, 0.0, "d"),
        ("gib", "mib", 1_024.0, 0.0, "d"),
        ("b", "mib", 1.0 / 1_048_576.0, 0.0, "d"),
        ("mib", "b", 1_048_576.0, 0.0, "d"),
        ("b", "gib", 1.0 / 1_073_741_824.0, 0.0, "d"),
        ("gib", "b", 1_073_741_824.0, 0.0, "d"),
        ("kib", "gib", 1.0 / 1_048_576.0, 0.0, "d"),
        ("gib", "kib", 1_048_576.0, 0.0, "d"),
        ("gb", "kib", 1_953_125.0 / 2.0, 0.0, "d"),
        ("kib", "gb", 2.0 / 1_953_125.0, 0.0, "d"),
        ("gb", "mib", 15_625_000.0 / 16_384.0, 0.0, "d"),
        ("mib", "gb", 16_384.0 / 15_625_000.0, 0.0, "d"),
        ("gb", "gib", 1_953_125.0 / 2_097_152.0, 0.0, "d"),
        ("gib", "gb", 2_097_152.0 / 1_953_125.0, 0.0, "d"),
        // VOLUME(v)
        //
        ("gal", "l", 473_176_473.0 / 125_000_000.0, 0.0, "v"),
        ("l", "gal", 125_000_000.0 / 473_176_473.0, 0.0, "v"),
    ];

    // User entered units matches with the units available in the database
    let mut matched_from_unit: i32 = 0;
    let mut matched_to_unit: i32 = 0;

    let mut output: f64 = val;
    let mut converted: bool = false;
    // Using "s" and "r" to initialize both variables instead of using letters already used by the
    // families
    let mut from_unit_family: &str = "s";
    let mut to_unit_family: &str = "r";
    // For unavailable operation between members of the same family.
    let mut same_family_but_no_conversion: bool = false;

    for item in DATA {
        if item.0 == from_unit {
            matched_from_unit += 1;
            from_unit_family = item.4;
        }
        if item.1 == to_unit.to_lowercase() {
            matched_to_unit += 1;
            to_unit_family = item.4;
        }

        if item.0 == from_unit && item.1 == to_unit.to_lowercase() {
            output = val * item.2 + item.3;
            converted = true;
        }
    }

    // Operation unavailable
    if !converted && from_unit_family == to_unit_family {
        same_family_but_no_conversion = true;
    }

    (
        output,
        matched_from_unit,
        matched_to_unit,
        converted,
        same_family_but_no_conversion,
    )

    // Error handling will be here for various scenarios
}

fn get_input(term: usize) -> String {
    env::args().nth(term).unwrap()
}
