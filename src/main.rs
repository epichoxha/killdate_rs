use chrono::{Local, NaiveDate};
use obfstr::obfstr as obf;

fn expiration_check(expiration_date: &str) {
    // Print the current date for debugging
    let current_date = Local::now().date_naive();
    #[cfg(debug_assertions)]
    println!("{} {:?}", obf!("[+] Current date:"), current_date);

    // Match on the result of parsing the expiration date
    match NaiveDate::parse_from_str(expiration_date, "%Y-%m-%d") {
        Ok(parsed_date) => match parsed_date.cmp(&current_date) {
            std::cmp::Ordering::Less => {
                // Expiration date is in the past
                if cfg!(debug_assertions) {
                    println!(
                        "{} {:?}",
                        obf!("[!] expired, expiration date:"),
                        parsed_date
                    );
                }
                std::process::exit(0);
            }
            _ => {
                // Expiration date is today or in the future
                #[cfg(debug_assertions)]
                println!(
                    "{} {:?}",
                    obf!("[+] Expiration date ({}) is valid"),
                    parsed_date
                );
            }
        },
        Err(_) => {
            // Failed to parse the date
            if cfg!(debug_assertions) {
                println!(
                    "{}",
                    obf!("[!] Failed to parse expiration date. Exiting...")
                );
            }
            std::process::exit(0);
        }
    }
}

fn main() {
    // Example usage
    expiration_check("2023-11-25"); // Replace with the desired expiration date
    #[cfg(debug_assertions)]
    println!("{}", obf!("[!] Program continues..."));
}
