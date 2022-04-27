//pub mod billfile;
pub mod duckdata;
pub mod duckacctid;
pub mod duckbill;
pub mod duckerror;

use std::fmt::{Debug, Formatter};
use std::ops::Index;
use std::slice::SliceIndex;
use duckerror::DuckError;
use crate::duckfile::duckbill::DuckBill;
use duckdata::DuckData;
use crate::duckfile::duckacctid::DuckAcctId;


const RECORD_MARK_BYTES: &[u8;3] = &[0x1bu8, 0x45u8, 0x0du8];

pub type DuckMark = usize;

pub struct DuckFile {
    header: DuckData,
    bills: Vec<DuckBill>,
    footer: DuckData,
    bill_count: u32,
}

impl DuckFile {
    const NOMINAL_HEADER_LEN: usize = 23;
    const NOMINAL_FOOTER_PRE_LEN: usize = 36;
    const NOMINAL_FOOTER_POST_LEN: usize = 1;
    const NOMINAL_FOOTER_COUNT_LEN: usize = 6;
    const NOMINAL_FOOTER_LEN: usize = DuckFile::NOMINAL_FOOTER_PRE_LEN + DuckFile::NOMINAL_FOOTER_COUNT_LEN + DuckFile::NOMINAL_FOOTER_POST_LEN;

    pub fn new() -> DuckFile {
        DuckFile {
            header: DuckFile::get_static_header(),
            bills: vec![],
            footer: DuckFile::get_arbitrary_footer(0).unwrap(),
            bill_count: 0,
        }
    }

    fn get_static_header() -> DuckData {
        DuckData::new(vec![0x1bu8, 0x26u8, 0x6cu8, 0x36u8, 0x44u8, 0x0du8, 0x1bu8, 0x28u8, 0x73u8,
                           0x31u8, 0x50u8, 0x1bu8, 0x28u8, 0x73u8, 0x35u8, 0x54u8, 0x1bu8, 0x28u8,
                           0x73u8, 0x31u8, 0x30u8, 0x56u8, 0x0du8]
        )
    }
    fn get_static_footer_pre() -> DuckData {
        DuckData::new(vec![
            0x1bu8, 0x45u8, 0x0du8, 0x1bu8, 0x45u8, 0x0du8, 0x0au8, 0x2au8, 0x2au8, 0x2au8, 0x2au8,
            0x20u8, 0x54u8, 0x4fu8, 0x54u8, 0x41u8, 0x4cu8, 0x20u8, 0x42u8, 0x49u8, 0x4cu8, 0x4cu8,
            0x53u8, 0x20u8, 0x50u8, 0x52u8, 0x49u8, 0x4eu8, 0x54u8, 0x45u8, 0x44u8, 0x20u8, 0x2du8,
            0x20u8, 0x20u8, 0x20u8,
        ])
    }
    fn get_static_footer_post() -> DuckData {
        DuckData::new(vec![0x0du8])
    }

    fn get_arbitrary_footer(count: u32) -> Result<DuckData, DuckError> {
        if count > 999_999 {
            Err(DuckError::BillCountOutOfBounds)
        }
        else {
            let mut new_data: DuckData = DuckFile::get_static_footer_pre();

            new_data.push(format!("{:0>6}", count.to_string()).as_bytes().into());
            new_data.push(DuckFile::get_static_footer_post());
            Ok(new_data)
        }
    }


    pub fn get_bill_count(&self) -> usize {
        self.bills.len()
    }

    pub fn get_index_of_account(&self, acct: &DuckAcctId) -> Option<usize> {
        self.bills.iter().position(|a| a.get_account_id() == acct)
    }

    pub fn get_header(&self) -> &DuckData {
        &self.header
    }

    pub fn get_footer(&self) -> &DuckData {
        &self.footer
    }
}

impl<I> Index<I> for DuckFile
where I: SliceIndex<[DuckBill]>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.bills[index]
    }
}

impl TryFrom<DuckData> for DuckFile {
    type Error = DuckError;

    fn try_from(data: DuckData) -> Result<Self, Self::Error> {
        if data.len() < 4000 {
            //min length sanity check
            return Err(DuckError::DataTooShort);
        }

        //the file should start with a header
        if data[0..DuckFile::NOMINAL_HEADER_LEN] != DuckFile::get_static_header()[..] {
            return Err(DuckError::BadHeaderFormat);
        }


        //next, a set of one or more bill chunks
        let mut start_marks: Vec<DuckMark> = Vec::new();
        let mut bills: Vec<DuckBill> = Vec::new();

        //find all markers
        for i in 0..data.len() - duckacctid::ACCT_MARK_LEN {
            if data[i..i + 3] == *RECORD_MARK_BYTES {
                start_marks.push(i);
            }
        }

        //sanity check: should be more than two marks
        if start_marks.len() < 2 {
            return Err(DuckError::NotEnoughMarkers);
        }

        //sanity check: first mark should be after the header
        if start_marks[0] < DuckFile::NOMINAL_HEADER_LEN {
            return Err(DuckError::HeaderTooShort);
        }

        //build vec of all bills
        for m in 0..start_marks.len() - 1 {
            for i in start_marks[m]..start_marks[m + 1] - duckacctid::ACCT_STR_BYTES_LEN {
                if data[i..i + duckacctid::ACCT_STR_BYTES_LEN] == *duckacctid::ACCT_STR_BYTES {
                    bills.push(DuckBill::new(data[start_marks[m]..start_marks[m + 1]].into())?)
                }
            }
        }
        let bill_count_bytes = data[data.len() - 7..data.len() - 1].to_owned();
        let bill_count = String::from_utf8(bill_count_bytes.into())?.parse::<u32>()? as usize;

        if bill_count != bills.len() || bill_count != start_marks.len() - 2 {
            return Err(DuckError::MarkCountMismatch);
        }

        //and lastly, the footer
        //check length
        if data[start_marks[start_marks.len()-2]..].len() != DuckFile::NOMINAL_FOOTER_LEN {
            return Err(DuckError::BadFooterFormat);
        }
        let footer: DuckData = data[start_marks[start_marks.len()-2]..].into();

        //check values: should match the static values
        if footer[..DuckFile::NOMINAL_FOOTER_PRE_LEN] != DuckFile::get_static_footer_pre()[..]
            || footer[footer.len() - DuckFile::NOMINAL_FOOTER_POST_LEN..] != DuckFile::get_static_footer_post()[..]
        {
            return Err(DuckError::BadFooterFormat);
        }

        //if we're here, everything checks out
        Ok(DuckFile {
            header: data[0..DuckFile::NOMINAL_HEADER_LEN].into(),
            bills,
            footer: data[start_marks[start_marks.len()-2]..].into(),
            bill_count: bill_count as u32,
        })
    }
}

impl TryFrom<Vec<DuckBill>> for DuckFile {
    type Error = DuckError;

    fn try_from(bills: Vec<DuckBill>) -> Result<Self, Self::Error> {
        let mut data = DuckFile::get_static_header();
        let bill_count = bills.len() as u32;
        data.push(bills.into());
        data.push(DuckFile::get_arbitrary_footer(bill_count)?);

        DuckFile::try_from(data)
    }
}

impl From<DuckFile> for DuckData {
    fn from(value: DuckFile) -> Self {
        let mut d = value.header;
        for b in value.bills {
            d.push(b.into());
        }
        d.push(value.footer);
        d
    }
}

impl Debug for DuckFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Header Len: {}\nBill Vec Len: {}\nBill Count: {}\nFooter Len: {}",
            self.header.len(), self.bills.len(), self.bill_count, self.footer.len())
    }
}

impl Default for DuckFile {
    fn default() -> Self {
        DuckFile::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;
    use std::io::Read;
    use crate::duckfile::duckacctid::DuckAcctId;

    pub fn get_test_data() -> DuckFile {
        let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("test_data/anon_bill_file_4.dat");

        let mut data: Vec<u8> = Vec::new();
        if let Ok(mut f) = File::open(test_file) {
            if f.read_to_end(&mut data).is_err() {
                panic!("Cannot read test data file!");
            }
        }

        let test_data: Result<DuckFile, DuckError> = DuckData::new(data).try_into();
        assert!(test_data.is_ok());
        test_data.unwrap()
    }

    #[test]
    fn count_is_correct() {
        let quack = get_test_data();

        assert_eq!(quack.bills.len(), 4); //todo: use method instead of field access
    }

    #[test]
    fn all_acct_ids_found() {
        let quack = get_test_data();

        let look_for = vec![b"52-1111111-1", b"52-2222222-1",b"52-3333333-1",b"52-4444444-1"];
        for &s in look_for {
            let f = quack.bills.iter().find(|b| *b.get_account_id() == DuckAcctId::try_from(s.to_vec()).unwrap());
            assert!(f.is_some());
        }
    }

    #[test]
    fn all_bill_numbers_found() {
        let quack = get_test_data();

        let look_for = vec![6671, 9648, 5956, 7488];
        for n in look_for {
            let f = quack.bills.iter().find(|b| b.get_bill_number() == n);
            assert!(f.is_some());
        }
    }

    #[test]
    fn header_is_sane() {
        let _quack = get_test_data();
        //insane headers will throw during test data acquisition
    }

    #[test]
    fn footer_is_sane() {
        let _quack = get_test_data();
        //insane footers will throw during test data acquisition
    }

    #[test]
    fn bill_count_shenanigans() {
        let mut quack = get_test_data();

        let footer = DuckFile::get_arbitrary_footer(69_420);
        assert!(footer.is_ok());
        quack.footer = footer.unwrap();
        let data2: DuckData = quack.into();
        //data2 contains a mismatched bill count
        let broken: Result<DuckFile, DuckError> = DuckData::new(data2.into()).try_into();
        assert!(broken.is_err());
        assert_eq!(broken.unwrap_err(), DuckError::MarkCountMismatch);
    }

    #[test]
    fn static_header_len() {
        assert_eq!(DuckFile::get_static_header().len(), DuckFile::NOMINAL_HEADER_LEN);
    }

    #[test]
    fn static_footer_lens() {
        assert_eq!(DuckFile::get_static_footer_pre().len(), DuckFile::NOMINAL_FOOTER_PRE_LEN);
        assert_eq!(DuckFile::get_static_footer_post().len(), DuckFile::NOMINAL_FOOTER_POST_LEN);
    }

}
