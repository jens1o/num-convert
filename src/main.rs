use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<Error>> {
    println!(
        "Welcome. Enter any number in any format you've been given, use 'q' or 'exit' to exit."
    );

    loop {
        print!("\n> ");
        io::stdout().flush()?;

        let input_string = {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;

            buffer
        };

        let mut input_string = input_string.trim().to_ascii_lowercase();

        if input_string.is_empty() {
            continue;
        }

        if input_string == "q" || input_string == "exit" {
            break;
        }

        // get radix and remove the signs for it out of the input string
        let radix: u32 = {
            if input_string.len() <= 2 {
                // any other radix is impossible(empty input!)
                // do not remove this fallback, otherwise this leads to out-of-bounds
                10
            } else {
                let radix = match &input_string[..=1] {
                    "0x" => 16,
                    "0o" => 8,
                    "0b" => 2,
                    _ => 10,
                };

                // if we have found a different radix, remove the identifier that prepended this number
                if radix != 10 {
                    // remove the two prepending characters
                    input_string = input_string[2..].to_string();
                }

                radix
            }
        };

        let parsed_number_res = usize::from_str_radix(&input_string, radix);

        match parsed_number_res {
            Ok(number) => {
                println!(
                    "Dec: {0}, Hex: {0:#X}, Oct: {0:#o}, Bin: {0:#b} [{1} trailing zeroes]",
                    number,
                    number.trailing_zeros()
                );
            }
            Err(ref e) => {
                eprintln!(
                    "Error while parsing number: {}! Detected base: {}",
                    e, radix
                );
            }
        }
    }

    Ok(())
}
