use std::fs::File;
use std::io;
use std::io::{Read, Write};
use duckbill::duckbill;

fn main() -> duckbill::DuckResult<()> {
    let filename = "UBQBILLS.527";
    let s = b"52-1188750-1";
    let s: Vec<u8> = b"52-0063200-0"[..].to_owned();

    println!("Welcome to the Just Ducky Second Chance Bill Handler");
    println!("Using file {}", filename);
    println!("Please wait while the file is loaded.");
    let mut f = File::open("UBQBILLS.527")?;
    let bills = duckbill::BillFile::new(&mut f)?;
    println!("File loaded.");

    println!("Available options:");
    println!("1   Chop to end (resume print job)");
    println!("2   Chop from start");
    println!("3   Chop range of bills");
    println!("4   Extract single bill");
    println!("Anything else to quit");
    print!("Enter choice: "); io::stdout().flush();
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice);

    let mut data = bills.get_header();
    match choice.trim() {
        "1" => {
            let s = get_acct_id("Account ID (for example, 01-0123456-0): ");
            data.append(&mut bills.chop_to_end(&s).unwrap());
        },
        "2" => {
            let s = get_acct_id("Account ID (for example, 01-0123456-0): ");
            data.append(&mut bills.chop_from_start(&s).unwrap());
        },
        "3" => {
            let s = get_acct_id("Beginning Account ID (for example, 01-0123456-0): ");
            let e = get_acct_id("Ending Account ID (for example, 01-0123456-0): ");
            data.append(&mut bills.chop_range(&s, &e).unwrap());
        }
        "4" => {
            let s = get_acct_id("Account ID (for example, 01-0123456-0): ");
            data.append(&mut bills.chop_single_bill(&s).unwrap());
        } ,
        _ => ()
    };
    data.append(&mut bills.get_footer());

    let mut o = File::create("TESTOUT")?;
    o.write_all(&data);

    Ok(())
}

fn get_acct_id(prompt: &str) -> duckbill::DuckAcctId {
    print!("{}", prompt); io::stdout().flush();
    let mut id = String::new();
    std::io::stdin().read_line(&mut id);
    id.trim().as_bytes().to_owned()
//todo: this needs validation lol
}