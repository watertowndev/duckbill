pub mod duckbill {
    use std::fs::File;
    use std::io::Read;

    pub type DuckResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    pub type DuckBill = std::collections::HashMap<Vec<u8>, (usize, usize)>;
    pub type DuckAcctId = [u8; 12];
    pub type DuckData = Vec<u8>;

    pub struct BillFile {
        bill_file: Vec<u8>,
        bill_marks: Vec<usize>,
        bill_index: DuckBill
    }

    impl BillFile {

        pub fn new(file: &mut std::fs::File) -> DuckResult<BillFile> {
            let mut bf = BillFile {
                bill_file: Vec::with_capacity(16_000_000),
                bill_marks: Vec::new(),
                bill_index: DuckBill::new()
            };
            file.read_to_end(&mut bf.bill_file);

            if bf.bill_file.len() < 4000 { // min length sanity check
                return Err("Too short".into()); //todo better error handling
            }
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

            Ok(bf)
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
                Some(&(start, _)) => Some(self.bill_file[start..].to_owned())
            }
        }

        pub fn chop_from_start(&self, acct_id: &DuckAcctId) -> Option<DuckData> {
            match self.bill_index.get(&acct_id[..]) {
                None => None,
                Some(&(_, end)) => Some(self.bill_file[0..end].to_owned())
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
    }
}