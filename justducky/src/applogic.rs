use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use duckbill::duckfile::duckbill::{DuckBill, DuckResult};
use duckbill::duckfile::duckdata::DuckData;
use duckbill::duckfile::duckerror::DuckError;
use duckbill::duckfile::DuckFile;



#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppState {
    NoFile,
    FileReady,
    ChopToEnd,
    ChopFromStart,
    ChopRange,
    ExtractOne,
}

pub struct AppLogic {
    state: AppState,
    file_path: Option<PathBuf>,
    output_file_path: Option<PathBuf>,
    pub original_bills: DuckFile,
}


impl AppLogic {
    pub fn new() -> AppLogic {
        AppLogic {
            state: AppState::NoFile,
            file_path: None,
            output_file_path: None,
            original_bills: DuckFile::new(),
        }
    }

    pub fn change_state(&mut self, to_state: AppState) -> Result<(),()> {
        self.state = to_state;
        Ok(())
    }

    pub fn get_state(&self) -> &AppState {
        &self.state
    }

    pub fn load_file_str(&mut self, file_path: &str) -> Result<(),()> {
        let file_pb = PathBuf::from(file_path);
        self.load_file_pathbuf(&file_pb)
    }
    pub fn load_file_pathbuf(&mut self, file_pb: &PathBuf) -> Result<(),()> {
        match AppLogic::try_loading_billfile(&file_pb) {
            Ok(ob) => {
                let mut outfilestr = file_pb.clone().into_os_string();
                outfilestr.push(".DUCKED");//append our signature extension
                self.output_file_path = Some(PathBuf::from(outfilestr));

                self.file_path = Some(file_pb.clone());
                self.original_bills = ob;
                self.change_state(AppState::FileReady);
                Ok(())
            }
            Err(_) => {
                Err(())
            }
        }
    }

    pub fn get_current_filename(&self) -> String {
        if self.file_path.is_some() {
            let osstr = self.file_path.clone().unwrap().into_os_string();
            osstr.to_str().unwrap_or("").to_string()
        }
        else {
            "".to_string()
        }
    }


    fn try_loading_billfile(file_choice: &PathBuf) -> Result <DuckFile, DuckError>{
        let bill_file = File::open(file_choice);

        match bill_file {
            Err(e) => {
                println!("{}", e);
                Err(DuckError::IoError)
            }
            Ok(mut bf) => {
                match bf.metadata() {
                    Err(_) => Err(DuckError::IoError),
                    Ok(m) => {
                        if m.len() < 4000 { // minimum length sanity check
                            Err(DuckError::FileTooSmall)
                        }
                        else if m.len() > 100_000_000 {// maximum length sanity check
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

    pub fn create_output_file(&self, bill_sel: Vec<DuckBill>) -> DuckResult<()>{
        if self.output_file_path == None {
            return Err(DuckError::IoError);
        }
        let filename = self.output_file_path.clone().unwrap();
        let mut o = File::create(&filename)?;

        let processed_file: DuckFile = bill_sel.try_into()?;
        let new_data: DuckData = processed_file.into();
        match o.write(new_data.as_ref()) {
            Ok(_) => Ok(()),
            Err(_) => Err(DuckError::IoError)
        }
    }
}
