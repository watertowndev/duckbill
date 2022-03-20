pub mod duckbill {
    pub type DuckResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    pub type DuckBill = std::collections::HashMap<Vec<u8>, (usize, usize)>;
    pub type DuckAcctId = Vec<u8>;
    pub type DuckData = Vec<u8>;

    pub struct BillFile {
        bill_file: Vec<u8>,
        bill_marks: Vec<usize>,
        bill_index: DuckBill,
        bill_count: u32
    }

    impl BillFile {

        pub fn new(mut file_data: Vec<u8>) -> DuckResult<BillFile> {
            if file_data.len() < 4000 { //min length sanity check
                return Err("File is too short to be valid.".into());
            }
            let mut bf = BillFile {
                bill_file: file_data,
                bill_marks: Vec::new(),
                bill_index: DuckBill::new(),
                bill_count: 0
            };

            const ACCT_STR_BYTES: &[u8; 8] = b"Acct No:";
            const ACCT_STR_BYTES_LEN: usize = ACCT_STR_BYTES.len();
            const ACCT_NUMBER_LEN: usize = b"01-0123456-0".len();
            const ACCT_MARK_LEN: usize = ACCT_STR_BYTES_LEN + ACCT_NUMBER_LEN;
            const RECORD_MARK_BYTES: [u8; 3] = [0x1b, 0x45, 0x0d];

            //find all markers
            for i in 0..bf.bill_file.len() - ACCT_MARK_LEN {
                if bf.bill_file[i..i + 3] == RECORD_MARK_BYTES {
                    bf.bill_marks.push(i);
                }
            }

            if bf.bill_marks.len() < 2 {
                return Err("Not enough marks to be a valid bill file.".into());
            }

            //index all bills
            for m in 0..bf.bill_marks.len() - 1 {
                for i in bf.bill_marks[m]..bf.bill_marks[m + 1] - ACCT_STR_BYTES_LEN {
                    if &bf.bill_file[i..i + ACCT_STR_BYTES_LEN] == ACCT_STR_BYTES {
                        let bill_id = bf.bill_file[i + ACCT_STR_BYTES_LEN + 1..=i + ACCT_STR_BYTES_LEN + ACCT_NUMBER_LEN].to_owned();
                        bf.bill_index.insert(bill_id, (bf.bill_marks[m], bf.bill_marks[m + 1]));
                    }
                }
            }
            let bill_count_bytes = bf.bill_file[bf.bill_file.len()-7..bf.bill_file.len()-1].to_owned();
            let bill_count_str = String::from_utf8(bill_count_bytes);
            let bill_count_parse = match bill_count_str {
                Ok(b) => b.parse::<u32>(),
                Err(_) => return Err("Could not find length marker.".into())
            };
            bf.bill_count = match bill_count_parse {
                Ok(b) => b,
                Err(_) => return Err("Bad format around length marker.".into())
            };

            if bf.bill_count != bf.bill_marks.len() as u32 - 2 {
                Err("Bill mark count does not match listed bill count.".into())
            }
            else {
                Ok(bf)
            }

        }

        pub fn get_bill_count(&self) -> u32 {
            self.bill_count
        }

        pub fn get_header(&self) -> DuckData {
            self.bill_file[0..self.bill_marks[0]].to_owned()
        }

        pub fn get_footer(&self) -> DuckData {
            self.bill_file[*self.bill_marks.last().unwrap() .. ].to_owned()
        }

        pub fn chop_single_bill(&self, acct_id: &DuckAcctId) -> Option<DuckData> {
            match self.bill_index.get(&acct_id[..]) {
                None => None,
                Some(&(start, end)) => Some(self.bill_file[start..end].to_owned())
            }
        }

        pub fn chop_to_end(&self, acct_id: &DuckAcctId) -> Option<DuckData> {
            match self.bill_index.get(&acct_id[..]) {
                None => None,
                Some(&(start, _)) => Some(self.bill_file[start..*self.bill_marks.last().unwrap()].to_owned())
            }
        }

        pub fn chop_from_start(&self, acct_id: &DuckAcctId) -> Option<DuckData> {
            match self.bill_index.get(&acct_id[..]) {
                None => None,
                Some(&(_, end)) => Some(self.bill_file[1..end].to_owned())
            }
        }

        pub fn chop_range(&self, first_acct_id: &DuckAcctId, last_acct_id: &DuckAcctId) -> Option<DuckData> {
            let first = self.bill_index.get(&first_acct_id[..]);
            let last = self.bill_index.get(&last_acct_id[..]);

            if first == None || last == None {
                return None;
            }
            if last <= first {
                return None;
            }
            let first = first.unwrap().0;
            let last = last.unwrap().1;
            Some(self.bill_file[first..last].to_owned())
        }

        pub fn bill_exists(&self, acct_id: &DuckAcctId) -> bool {
            self.bill_index.contains_key(&acct_id[..])
        }
    }
}