//! DuckAcctId constrains a string of bytes to ensure it is a value account ID

use super::{DuckData, DuckError};

/// The base structure
#[derive(Debug,PartialEq)]
pub struct DuckAcctId {
    my_data: Vec<u8>,
}

/// Byte sequence that precedes an account number when parsing
pub const ACCT_STR_BYTES: &[u8] = b"Acct No: ";
/// length of the acct_str byte sequence
pub const ACCT_STR_BYTES_LEN: usize = ACCT_STR_BYTES.len();
/// length of an account number byte sequence
pub const ACCT_NUMBER_LEN: usize = b"01-0123456-0".len();
/// length of the entire byte sequence
pub const ACCT_MARK_LEN: usize = ACCT_STR_BYTES_LEN + ACCT_NUMBER_LEN;

impl DuckAcctId {
    /// validate a given DuckData sequence as a likely-valid ID
    /// only validates syntax; does not verify semantic meaning.
    /// an ID that is formatted correctly could still be invalid to the billing program
    /// valid IDs are of the format 01-0123456-7
    pub fn validate(id: &DuckData) -> bool {
        //format is 01-0123456-0
        for &p in [0, 1, 3, 4, 5, 6, 7, 8, 9, 11].iter() {
            if !DuckData::is_ascii_number(id[p]) {
                return false;
            }
        }

        id[2] == b'-' && id[10] == b'-'
    }
}

impl ToString for DuckAcctId {
    fn to_string(&self) -> String {
        String::from_utf8(self.my_data.clone()).unwrap_or_else(|_| {
            self.my_data
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",")
        })
    }
}

/// thin wrapper that invokes TryFrom<DuckData>
impl TryFrom<Vec<u8>> for DuckAcctId {
    type Error = DuckError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        DuckData::new(value).try_into()
    }
}

/// try to convert from raw DuckData byte sequence to an ID
impl TryFrom<DuckData> for DuckAcctId {
    type Error = DuckError;

    fn try_from(value: DuckData) -> Result<Self, Self::Error> {
        //format is 01-0123456-0
        if value.len() != ACCT_NUMBER_LEN {
            return Err(DuckError::AccountIDTooShort);
        }

        if DuckAcctId::validate(&value) {
            Ok(DuckAcctId{
                my_data: value.into()
            })
        } else {
            Err(DuckError::BadAccountIdFormat)
        }
    }
}

/// convert to DuckData sequence
impl From<DuckAcctId> for DuckData {
    fn from(id: DuckAcctId) -> Self {
        DuckData::new(id.my_data)
    }
}

impl Clone for DuckAcctId {
    fn clone(&self) -> Self {
        DuckAcctId {
            my_data: self.my_data.clone(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::duckfile::duckacctid::DuckAcctId;
    use crate::duckfile::duckdata::DuckData;
    use crate::duckfile::duckerror::DuckError;

    #[test]
    fn to_string_valid_utf8() {
        assert_eq!(
            DuckAcctId {
                my_data: vec![1, 2, 3, 4, 5, 6, 7, 254, 255, 0, 1, 23]
            }
            .to_string(),
            "1,2,3,4,5,6,7,254,255,0,1,23".to_string()
        );
    }

    #[test]
    fn to_string_invalid_utf8() {
        assert_eq!(
            DuckAcctId {
                my_data: vec![65, 66, 67, 68, 69, 70]
            }
            .to_string(),
            "ABCDEF".to_string()
        );
    }

    #[test]
    fn valid_acct_id_ok() {
        let data: DuckData = "01-0123456-0".into();
        assert!(DuckAcctId::validate(&data));
    }

    #[test]
    fn invalid_acct_id_fail() {
        let too_short: DuckData = "moo".into();
        assert!(! DuckAcctId::validate(&too_short));

        let bad_digit: DuckData = "01-01Z3456-0".into();
        assert!(! DuckAcctId::validate(&bad_digit));
    }

    #[test]
    fn try_from_works() {
        let good_data: DuckData = "01-0123456-0".into();
        let good_id_res: Result<DuckAcctId, DuckError> = good_data.try_into();

        assert!(good_id_res.is_ok());

        let bad_data: DuckData = "failure".into();
        let bad_id_res: Result<DuckAcctId, DuckError> = bad_data.try_into();

        assert!(bad_id_res.is_err());
    }
}
