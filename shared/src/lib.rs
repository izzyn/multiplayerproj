
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod Data {
    use std::{error::Error, fmt};





    #[derive(Debug)]
    pub enum Type {
        U32,
        U64,
        I32,
        I64,
        String,
        Vector,
        BeginArgs,

    }

    #[derive(Debug)]
    pub struct DataParseError {
        attempt : Type,
    }

    impl Error for DataParseError {}

    impl fmt::Display for DataParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error trying to parse type {:?}", self.attempt)
        }
    }

    pub struct BuffData {
        d_type : Type,
        data : Vec<u8>,


    }
    pub enum ParsedData {
        U32(i32),
        U64(u64),
        String(std::string::String),
        Vector(std::vec::Vec<ParsedData>),
        BeginArgs(std::vec::Vec<ParsedData>),
    }

    impl BuffData {
        fn parse(&self) -> Result<ParsedData, DataParseError> {
            return Ok(ParsedData::U32(1))
        }

        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
