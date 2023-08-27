use std::io;
use std::io::Write;
use duckbill::duckfile::duckacctid::DuckAcctId;
use duckbill::duckfile::duckbill::DuckBill;
use duckbill::duckfile::duckerror::DuckError;
use m_menu::MMenu;
use crate::applogic::{AppLogic, AppState};


pub fn tui() -> Result<(), DuckError> {
    let mut state = AppLogic::new();
    let mut main_menu = MMenu::new();
    main_menu.add_entry("1", "Select bill file", true);
    main_menu.add_entry("2", "Chop to end (resume print job)", false);
    main_menu.add_entry("3", "Chop from start", false);
    main_menu.add_entry("4", "Chop range of bills", false);
    main_menu.add_entry("5", "Extract single bill", false);


    println!("Welcome to the Just Ducky Second Chance Bill Handler");
    println!("====================================================");
    // main menu ui loop
    loop {
        let file_ready = *state.get_state() != AppState::NoFile;
        main_menu.set_choice_avail("2", file_ready).expect("Menu 2 avail error");
        main_menu.set_choice_avail("3", file_ready).expect("Menu 3 avail error");
        main_menu.set_choice_avail("4", file_ready).expect("Menu 4 avail error");
        main_menu.set_choice_avail("5", file_ready).expect("Menu 5 avail error");

        println!("\nMain Menu");
        println!("=========");
        println!("{}", main_menu);
        if file_ready {
            println!("Current file: {}", state.get_current_filename());
        }
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
            "1" => { //select file
                print!("Enter path to file (press enter to cancel): ");
                io::stdout().flush()?;
                let mut file_input = String::new();
                io::stdin().read_line(&mut file_input)?;

                match state.load_file(&file_input.trim()) {
                    Ok(_) => {
                        println!("{} bills found", state.original_bills.get_bill_count());
                    }
                    Err(_) => {
                        println!("No luck getting file, returning to main menu.");
                    }
                }
            },
            "2" => { //chop to end
                let s = get_acct_id("Account ID (for example, 01-0123456-0): ");
                if s.is_err() {
                    println!("Cancelled or invalid ID.");
                    continue;
                }
                output_selection(&state, state.original_bills.get_index_of_account(&s.unwrap()).map(|bill_idx| &state.original_bills[bill_idx..]) );
            },
            "3" => { //chop from start
                let s = get_acct_id("Account ID (for example, 01-0123456-0): ");
                if s.is_err() {
                    println!("Cancelled or invalid ID.");
                    continue;
                }
                output_selection(&state, state.original_bills.get_index_of_account(&s.unwrap()).map(|bill_idx| &state.original_bills[..=bill_idx]) );
            },
            "4" => { //chop range
                let s = get_acct_id("Beginning Account ID (for example, 01-0123456-0): ");
                if s.is_err() {
                    println!("Cancelled or invalid ID.");
                    continue;
                }
                let e = get_acct_id("Ending Account ID (for example, 01-0123456-0): ");
                if e.is_err() {
                    println!("Cancelled or invalid ID.");
                    continue;
                }

                let start_idx = state.original_bills.get_index_of_account(&s.unwrap());
                let end_idx = state.original_bills.get_index_of_account(&e.unwrap());

                if start_idx.is_some() && end_idx.is_some() {
                    let (start, end) = if start_idx.unwrap() > end_idx.unwrap() {
                        println!("End is before start, swapping...");
                        (end_idx.unwrap(), start_idx.unwrap())
                    } else {
                        (start_idx.unwrap(), end_idx.unwrap())
                    };
                    let sel = &state.original_bills[start..=end];
                    output_selection(&state, Some(sel));
                } else {
                    println!("One of your account IDs was not found in the billfile.");
                }
            }
            "5" => { //extract single
                let s = get_acct_id("Account ID (for example, 01-0123456-0): ");
                if s.is_err() {
                    continue;
                }
                output_selection(&state, state.original_bills.get_index_of_account(&s.unwrap()).map(|bill_idx| &state.original_bills[bill_idx..=bill_idx]) );
            },

            _ => { println!("Well, this should be impossible."); }
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

fn output_selection(state: &AppLogic, selection: Option<&[DuckBill]>) {
    if let Some(sel) = selection {
        match state.create_output_file(sel.to_vec()) {
            Ok(_) => {
                println!("Your processed file is ready");
            }
            Err(_) => {
                println!("There was an error creating your output file.");
            }
        }
    } else {
        println!("Account ID not found in billfile.");
    }
}