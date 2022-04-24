use std::ops::{Index};
use std::slice::SliceIndex;

#[derive(PartialEq, Debug, Default)]
pub struct DuckData {
    my_data: Vec<u8>
}

impl DuckData {
    pub fn len(&self) -> usize {
        self.my_data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.my_data.is_empty()
    }

    pub fn new(data: Vec<u8>) -> DuckData {
        DuckData{
            my_data: data
        }
    }

    pub fn push(&mut self, mut data: DuckData) {
        self.my_data.append(&mut data.my_data)
    }

    pub fn is_ascii_number(b: u8) -> bool {
        (48..=57).contains(&b)
    }
}

impl ToString for DuckData {
    fn to_string(&self) -> String {
        String::from_utf8(self.my_data.clone()).unwrap_or_else(|_| {
            self.my_data
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
    }
}

impl From<Vec<u8>> for DuckData {
    fn from(value: Vec<u8>) -> Self {
        DuckData {
            my_data: value
        }
    }
}

impl From<&[u8]> for DuckData {
    fn from(value: &[u8]) -> Self {
        DuckData {
            my_data: Vec::from(value)
        }
    }
}

impl From<&str> for DuckData {
    fn from(value: &str) -> Self {
        DuckData {
            my_data: value.as_bytes().to_vec()
        }
    }
}

impl From<DuckData> for Vec<u8> {
    fn from(value: DuckData) -> Self {
        value.my_data
    }
}

impl Clone for DuckData {
    fn clone(&self) -> Self {
        DuckData {
            my_data: self.my_data.clone()
        }
    }
}

impl PartialEq<&[u8]> for DuckData {
    fn eq(&self, other: &&[u8]) -> bool {
        self.my_data == *other
    }

    fn ne(&self, other: &&[u8]) -> bool {
        !(self.my_data == *other)
    }
}

impl<I> Index<I> for DuckData
where I: SliceIndex<[u8]>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.my_data[index]
    }
}

impl AsRef<[u8]> for DuckData {
    fn as_ref(&self) -> &[u8] {
        &self.my_data
    }
}



#[cfg(test)]
mod tests {
    use crate::duckfile::duckdata::DuckData;

    #[test]
    fn to_string_valid_utf8() {
        assert_eq!(
            DuckData {
                my_data: vec![1, 2, 3, 4, 5, 6, 7, 254, 255, 0, 1, 23]
            }
                .to_string(),
            "1 2 3 4 5 6 7 254 255 0 1 23".to_string()
        );
    }

    #[test]
    fn to_string_invalid_utf8() {
        assert_eq!(
            DuckData {
                my_data: vec![65, 66, 67, 68, 69, 70]
            }
                .to_string(),
            "ABCDEF".to_string()
        );
    }
}