use super::duckacctid;
use super::duckacctid::DuckAcctId;
use super::duckdata::DuckData;
use super::duckerror::DuckError;

pub type DuckResult<T> = std::result::Result<T, DuckError>;
pub type DuckBillMap = std::collections::HashMap<Vec<u8>, (usize, usize)>;
pub type DuckIndex = Vec<usize>;

const BILLNUM_STR_BYTES: &[u8] = b"BILL #:    \x1b&a0405v0825H";
const BILLNUM_STR_BYTES_LEN: usize = BILLNUM_STR_BYTES.len();
const BILLNUM_LEN: usize = b"0123456".len();

#[derive(PartialEq, Debug)]
pub struct DuckBill {
    raw_data: DuckData,
    account_id: DuckAcctId,
    bill_number: u32,
}

impl DuckBill {
    pub fn new(raw: DuckData) -> DuckResult<DuckBill> {
        raw.try_into()
    }

    pub fn get_account_id(&self) -> &DuckAcctId {
        &self.account_id
    }

    pub fn get_bill_number(&self) -> u32 {
        self.bill_number
    }

    pub fn get_raw(&self) -> &DuckData {
        &self.raw_data
    }
}

impl ToString for DuckBill {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl TryFrom<DuckData> for DuckBill {
    type Error = DuckError;

    fn try_from(raw_data: DuckData) -> Result<Self, Self::Error> {
        let mut account_id_maybe = None;
        let mut bill_num_maybe = None;
        for i in 0..raw_data.len() - BILLNUM_STR_BYTES_LEN {
            if raw_data[i..i + duckacctid::ACCT_STR_BYTES_LEN] == *duckacctid::ACCT_STR_BYTES {
                account_id_maybe = Some(
                    raw_data[i + duckacctid::ACCT_STR_BYTES_LEN
                        ..i + duckacctid::ACCT_STR_BYTES_LEN + duckacctid::ACCT_NUMBER_LEN]
                        .to_owned(),
                );
            }
            if raw_data[i..i + BILLNUM_STR_BYTES_LEN] == *BILLNUM_STR_BYTES {
                bill_num_maybe = Some(
                    raw_data
                        [i + BILLNUM_STR_BYTES_LEN ..i + BILLNUM_STR_BYTES_LEN + BILLNUM_LEN]
                        .to_owned(),
                );
            }
        }

        match (account_id_maybe, bill_num_maybe) {
            (Some(account_id), Some(bill_number_bytes)) => {
                let bill_num_str = String::from_utf8(bill_number_bytes);
                let bill_number = if bill_num_str.is_ok() {
                    let bill_num_parse = bill_num_str.unwrap().parse::<u32>();
                    if let Ok(bill_num) = bill_num_parse {
                        Ok(bill_num)
                    } else {
                        Err(DuckError::BadBillNumberFormat)
                    }
                } else {
                    Err(DuckError::BadNumberData)
                };

                Ok(DuckBill {
                    raw_data,
                    account_id: account_id.try_into()?,
                    bill_number: bill_number?,
                })
            }
            (_, _) => Err(DuckError::BadIdentifierData),
        }
    }
}

impl AsRef<[u8]> for DuckBill {
    fn as_ref(&self) -> &[u8] {
        self.raw_data.as_ref()
    }
}

impl From<DuckBill> for DuckData {
    fn from(bill: DuckBill) -> Self {
        bill.raw_data
    }
}

impl From<Vec<DuckBill>> for DuckData {
    fn from(bills: Vec<DuckBill>) -> Self {
        let mut all = DuckData::new(vec![]);
        for b in bills {
            all.push(b.into());
        }

        all
    }
}

impl Clone for DuckBill {
    fn clone(&self) -> Self {
        DuckBill {
            raw_data: self.raw_data.clone(),
            account_id: self.account_id.clone(),
            bill_number: self.bill_number
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::duckfile::tests::get_test_data;

    #[test]
    fn clone_works() {
        let test_data = get_test_data();
        assert_eq!(test_data.bills[0], test_data.bills[0].clone());
    }
}