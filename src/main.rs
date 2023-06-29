use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let log_file_path = "input_logg.txt";
    let search_string = "madmange is alive";
    let special_number = 42;
    let output_file_path = "output_logg.txt";

    match find_and_save_content(log_file_path, search_string, special_number, output_file_path) {
        Ok(_) => println!("Content saved successfully!"),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn find_and_save_content(
    log_file_path: &str,
    search_string: &str,
    special_number: u32,
    output_file_path: &str,
) -> std::io::Result<()> {
    let log_file = File::open(log_file_path)?;
    let reader = BufReader::new(log_file);

    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);

    let mut found_string = false;
    let mut found_special_number = false;
    let mut capturing_content = false;
    let mut captured_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if line.contains(search_string) {
            if found_string && found_special_number {
                // Found the input string again, stop capturing
                break;
            } else {
                // Found the input string for the first time, start capturing
                found_string = true;
                capturing_content = true;
                captured_lines.clear(); // Clear previously captured lines
                continue;
            }
        } else if capturing_content {
            captured_lines.push(line.clone()); // Capture the lines between the input strings

            if line.contains(&special_number.to_string()) {
                // Found a line containing the special number between the input strings
                found_special_number = true;
            }
        }
    }

    if found_special_number {
        // Write the captured lines to the output file
        for line in captured_lines {
            writeln!(writer, "{}", line)?;
        }
    }

    writer.flush()?;
    Ok(())
}
