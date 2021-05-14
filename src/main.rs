use std::io;
use std::fs::File;
use std::io::{Read, Write, Error};
use duckbill::duckbill;

fn main() -> duckbill::DuckResult<()> {
    println!("Welcome to the Just Ducky Second Chance Bill Handler");
    let mut bill_data = Vec::with_capacity(16_000_000);
    loop {
        print!("Enter path to file (press enter to exit): "); io::stdout().flush()?;
        let mut file_choice = String::new();
        io::stdin().read_line(&mut file_choice)?;
        if file_choice.trim().len() == 0 {
            return Ok(());
        }
        let mut bill_file = File::open(file_choice.trim());
        match bill_file {
            Ok(mut bf) => {
                match bf.metadata() {
                    Ok(m) => {
                        if m.len() < 4000 { //min length sanity check
                            println!("That file is too short to be valid.");
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("Could not obtain file metadata.");
                        continue;
                    }
                }
                match bf.read_to_end(&mut bill_data) {
                    Ok(_) => break,
                    Err(_) => println!("Unable to read {}.", file_choice.trim())
                }
            }
            Err(_) => {
                println!("Could not open {}. Please try again.", file_choice.trim());
                continue;
            }
        }
    };

    println!("Please wait while the file is loaded.");
   let bills = match duckbill::BillFile::new(bill_data) {
        Ok(b) => b,
        Err(e) => {
            println!("Problem with bill file: {}", e);
            println!("Contact support. Press enter to continue.");
            io::stdin().read_line(&mut String::new())?;
            return Err(e);
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

    let mut o = File::create("TEST_OUT")?;
    o.write_all(&data);

    Ok(())
}

fn get_acct_id(prompt: &str) -> duckbill::DuckResult<duckbill::DuckAcctId> {
    print!("{}", prompt); io::stdout().flush()?;
    let mut id = String::new();
    std::io::stdin().read_line(&mut id)?;
    Ok(id.trim().as_bytes().to_owned())
//todo: this needs validation lol
}