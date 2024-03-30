use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

#[cfg(feature="native-ui")]
use native_dialog::FileDialog;

use duckbill::duckfile;
use duckbill::duckfile::duckacctid::DuckAcctId;
use duckbill::duckfile::duckbill::DuckBill;
use duckbill::duckfile::duckdata::DuckData;
use duckbill::duckfile::duckerror::DuckError;
use duckfile::DuckFile;
use m_menu::MMenu;

fn main() -> Result<(), DuckError> {
    let mut main_menu = MMenu::new();
    main_menu.add_entry("1", "Select bill file", true);
    main_menu.add_entry("2", "Skip from start to specified bill (resume print job)", false);
    main_menu.add_entry("3", "Skip from specified bill to end", false);
    main_menu.add_entry("4", "Extract range of bills", false);
    main_menu.add_entry("5", "Extract single bill", false);

    let mut file_ready = false;
    let mut original_bills = DuckFile::new();
    let mut output_filename = PathBuf::new();

    println!("Welcome to the Just Ducky Second Chance Bill Handler");
    println!("====================================================");
    // main menu ui loop
    loop {
        main_menu.set_choice_avail("2", file_ready).expect("Menu 2 avail error");
        main_menu.set_choice_avail("3", file_ready).expect("Menu 3 avail error");
        main_menu.set_choice_avail("4", file_ready).expect("Menu 4 avail error");
        main_menu.set_choice_avail("5", file_ready).expect("Menu 5 avail error");

        println!("\nMain Menu");
        println!("=========");
        println!("{}", main_menu);
        print!("Select number (q to quit): ");
        io::stdout().flush()?;
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;

        if !main_menu.valid_choice(&choice.trim()) {
            println!("Bye!");
            return Ok(());
        }
        println!("");


        match choice.trim() {
            "1" => {
                let mut file_choice: Option<PathBuf> = None;
                #[cfg(feature="native-ui")] {
                    println!("Use the dialog to select a file.");
                    let dialog_sel = FileDialog::new()
                            .set_location("~")
                            .show_open_single_file();
                    if let Ok(maybe_path) = dialog_sel {
                        file_choice = maybe_path;
                    }
                    else {
                        println!("No filename given, returning to menu.");
                    }
                }
                #[cfg(not(feature="native-ui"))] {
                    print!("Enter path to file (press enter to cancel): ");
                    io::stdout().flush()?;
                    let mut file_input = String::new();
                    io::stdin().read_line(&mut file_input)?;
                    file_choice = Some(PathBuf::from(&file_input));
                }

                if let Some(filepath) = file_choice {
                    println!("Loading file (this may take a little while)");
                    if let Ok(ob) = get_file(&filepath) {
                        original_bills = ob;
                        file_ready = true;
                        println!("{} bills found", original_bills.get_bill_count());
                        main_menu.add_entry("1", &format!("Select bill file (Current: {}, {} bills)", filepath.to_str().unwrap_or("Non-displayable file!"), original_bills.get_bill_count()), true);

                        let mut outfilestr = filepath.into_os_string();
                        outfilestr.push(".DUCKED");//append our signature extension
                        output_filename = PathBuf::from(outfilestr);
                    }
                }
                else {
                    file_ready = false;
                    main_menu.add_entry("1", "Select bill file", true);
                    println!("No luck getting file, returning to main menu.");
                }
            },
            "2" => {
                let s = get_acct_id("Account ID of first bill to keep (for example, 01-0123456-0): ")?;
                if let Some(sel) = original_bills.get_index_of_account(&s).map(|bill_idx| &original_bills[bill_idx..]) {
                    create_output_file(&output_filename, sel.to_vec())?;
                }
                else {
                    println!("Account ID not valid!");
                }
            },
            "3" => {
                let s = get_acct_id("Account ID of last bill to keep (for example, 01-0123456-0): ")?;
                if let Some(sel) = original_bills.get_index_of_account(&s).map(|bill_idx| &original_bills[..=bill_idx]) {
                    create_output_file(&output_filename, sel.to_vec())?;
                }
                else {
                    println!("Account ID not valid!");
                }
            },
            "4" => {
                let s = get_acct_id("Account ID of starting bill (for example, 01-0123456-0): ")?;
                let e = get_acct_id("Account ID of ending bill (for example, 01-0123456-0): ")?;

                let start_idx = original_bills.get_index_of_account(&s);
                let end_idx = original_bills.get_index_of_account(&e);

                if start_idx.is_some() && end_idx.is_some() {
                    let (start, end) = if start_idx.unwrap() > end_idx.unwrap() {
                        println!("End is before start, swapping...");
                        (end_idx.unwrap(), start_idx.unwrap())
                    }
                    else {
                        (start_idx.unwrap(), end_idx.unwrap())
                    };
                    let sel = &original_bills[start..=end];
                    create_output_file(&output_filename, sel.to_vec())?;
                }
                else {
                    println!("Account ID not valid!");
                }
            }
            "5" => {
                let s = get_acct_id("Account ID of bill (for example, 01-0123456-0): ")?;
                if let Some(sel) = original_bills.get_index_of_account(&s).map(|bill_idx| &original_bills[bill_idx..=bill_idx]) {
                    create_output_file(&output_filename, sel.to_vec())?;
                }
                else {
                    println!("Account ID not valid!");
                }
            } ,

            _ => {println!("Well, this should be impossible.");}
        }
    }
}

fn get_acct_id(prompt: &str) -> Result<DuckAcctId, DuckError> {
    loop {
        print!("{}", prompt); io::stdout().flush()?;
        let mut id = String::new();
        std::io::stdin().read_line(&mut id)?;
        // account ID format is 01-0123456-0
        // plus one newline equals 13
        if id.len() >= 13 {
            let id_bytes = id.trim().as_bytes().to_owned();
            break DuckAcctId::try_from(id_bytes);
        }
        else if id.len() == 1 {
            break Err(DuckError::OpCancelled);
        }
    }
}


fn get_file(file_choice: &PathBuf) -> Result <DuckFile, DuckError>{
    let bill_file = File::open(file_choice);

    match bill_file {
        Err(_) => Err(DuckError::IoError),
        Ok(mut bf) => {
            match bf.metadata() {
                Err(_) => Err(DuckError::IoError),
                Ok(m) => {
                    if m.len() < 4000 { // minimum length sanity check
                        println!("That file is too short to be valid.");
                        Err(DuckError::FileTooSmall)
                    }
                    else if m.len() > 100_000_000 {// maximum length sanity check
                        println!("That file is too large to be valid.");
                        Err(DuckError::FileTooBig)
                    }
                    else {
                        let mut bill_data = Vec::with_capacity(m.len() as usize);
                        match bf.read_to_end(&mut bill_data) {
                            Err(_) => Err(DuckError::IoError),
                            Ok(_) => DuckFile::try_from(DuckData::from(bill_data))
                        }
                    }
                }
            }
        }
    }
}

fn create_output_file(filename: &PathBuf, bill_sel: Vec<DuckBill>) -> Result<(), DuckError>{
    let mut o = File::create(&filename)?;

    let processed_file: DuckFile = bill_sel.try_into()?;
    let new_data: DuckData = processed_file.into();
    if o.write(new_data.as_ref()).is_ok() {
        println!("Working...");
        println!();
        println!("Your processed file is ready: {}", filename.to_str().unwrap_or("Undisplayable filename. Nice work."));
        return Ok(())
    }
    else {
        println!("An error occurred while writing the output file.");
    }
    Ok(())
}