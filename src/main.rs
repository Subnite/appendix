use std::fs::File;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if an argument is provided
    if args.len() != 4 {
        eprintln!("Usage: {} <\"input-file.extention\"> <\"output-file.extention\"> <\"search-prefix\">", args[0]);
        eprintln!("\n-----\nThis program will go through the input file and look for the search-prefix. It will then check the filename after the prefix and read from there (no space after the prefix)");
        eprintln!("\nExample:\nhome.html\nbio.html\n\nNow if you want to add the contents of bio.html somewhere inside of home.html, make a new line and add the prefix followed by the filename in home.html:\n<prefixexample!>bio.html");
        eprintln!("\nrun command:\nappendix home.html output.html <prefixexample!>\n-----");
        std::process::exit(1);
    }

    // Get the filename from the arguments
    let input_filename = &args[1];
    let output_filename = &args[2];
    let search_prefix = &args[3];

    println!("[INFO] Reading from \"{}\" ...", input_filename);

    let final_text = process_file(input_filename.to_string(), search_prefix.to_string()).unwrap();

    write_to_file(final_text, output_filename.to_string()).unwrap();
    println!("[SUCCESS] Output is \"{}\"", output_filename);
}

fn process_file(filename: String, search_prefix: String) -> Result<String, String> {
    // Open the file
    let mut text = String::from("");
    let file = File::open(&filename);
    match file {
        Err(e) => {return Err(e.to_string());},
        _ => ()
    }

    let reader = BufReader::new(file.unwrap());

    // Iterate over lines
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&search_prefix) {
            let prefix_start_idx = line.find(&search_prefix).unwrap();
            let new_filename: String = line[prefix_start_idx + search_prefix.len()..].to_string();
            let new_text = process_file(new_filename, search_prefix.clone());
            match new_text {
                Err(s) => eprintln!("Skipping, couldn't identifying the filename: {}", s),
                Ok(s) => text += &s
            }
        } else {
            text += &line;
        }
        text += "\n";
    }

    Ok(text)
}

fn write_to_file(text: String, filename: String) -> Result<(),String> {
    // Create a new file
    let mut file = File::create(filename).unwrap();

    // Write the text to the file
    let _ = file.write_all(text.as_bytes()).unwrap();
    Ok(())
}
