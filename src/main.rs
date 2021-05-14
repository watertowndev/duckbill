use std::io;
use std::fs::File;
use std::io::{Read, Write, Error};
use duckbill::duckbill;

fn main() -> duckbill::DuckResult<()> {
    let mut file_choice: String;

    // Main menu loop
    println!("Welcome to the Just Ducky Second Chance Bill Handler");
    let bills = loop {
        file_choice = String::new();
        print!("Enter path to file (press enter to exit): "); io::stdout().flush()?;
        io::stdin().read_line(&mut file_choice)?;
        if file_choice.trim().len() == 0 {
            return Ok(());
        }

        let bill_file = File::open(file_choice.trim());
        match bill_file {
            Err(e) => println!("Error opening file: {}", e),
            Ok(mut bf) => {
                match bf.metadata() {
                    Err(e) => println!("Error obtaining metadata: {}", e),
                    Ok(m) => {
                        // minimum length sanity check
                        if m.len() < 4000 {
                            println!("That file is too short to be valid.");
                            continue;
                        }
                        // maximum length sanity check
                        if m.len() > 100_000_000 {
                            println!("That file is too large to be valid.");
                            continue;
                        }
                        println!("File is {} KiB.", m.len()/1024);
                    }
                };

                println!("Loading, please wait...");
                let mut bill_data = Vec::with_capacity(16_000_000);
                match bf.read_to_end(&mut bill_data) {
                    Err(e) => println!("Error reading file: {}", e),
                    Ok(_) => match duckbill::BillFile::new(bill_data) {
                                Ok(b) => break b,
                                Err(e) => println!("Problem with bill file: {}", e)
                    }
                }
            }
        }
    };

    println!("File loaded, {} bills found.", bills.get_bill_count());

    println!("Available options:");
    println!("1   Chop to end (resume print job)");
    println!("2   Chop from start");
    println!("3   Chop range of bills");
    println!("4   Extract single bill");
    print!("Enter choice (press enter to quit): "); io::stdout().flush()?;
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;

    let mut data = bills.get_header();
    match choice.trim() {
        "1" => {
            let s = match get_acct_id("Account ID (for example, 01-0123456-0): ") {
                Ok(a) => a,
                Err(e) => return Err(e)
            };
            data.append(&mut bills.chop_to_end(&s).unwrap());
        },
        "2" => {
            let s = match get_acct_id("Account ID (for example, 01-0123456-0): ") {
                Ok(a) => a,
                Err(e) => return Err(e)
            };
            data.append(&mut bills.chop_from_start(&s).unwrap());
        },
        "3" => {
            let s = match get_acct_id("Beginning Account ID (for example, 01-0123456-0): ") {
                Ok(a) => a,
                Err(e) => return Err(e)
            };
            let e = match get_acct_id("Ending Account ID (for example, 01-0123456-0): ") {
                Ok(a) => a,
                Err(e) => return Err(e)
            };
            data.append(&mut bills.chop_range(&s, &e).unwrap());
        }
        "4" => {
            let s = match get_acct_id("Account ID (for example, 01-0123456-0): ") {
                Ok(a) => a,
                Err(e) => return Err(e)
            };
            data.append(&mut bills.chop_single_bill(&s).unwrap());
        } ,
        _ => ()
    };
    data.append(&mut bills.get_footer());

    let mut output_file = String::from(file_choice.trim());
    output_file.push_str(".DUCKED");

    let mut o = File::create(output_file)?;
    o.write_all(&data);
    println!("Your processed file is ready: {}", output_file);

    Ok(())
}

fn get_acct_id(prompt: &str) -> duckbill::DuckResult<duckbill::DuckAcctId> {
    print!("{}", prompt); io::stdout().flush()?;
    let mut id = String::new();
    std::io::stdin().read_line(&mut id)?;
    Ok(id.trim().as_bytes().to_owned())
//todo: this needs validation lol
}