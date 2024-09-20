//shared/src/lib.rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod data {
    use std::{error::Error, fmt, num::NonZeroUsize};

    #[derive(Debug)]
    pub struct DataParseError {}

    impl Error for DataParseError {}

    impl fmt::Display for DataParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error trying to parse type")
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ParsedNodeId(NonZeroUsize);

    enum ParsedData {
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
        CHAR(char),
        STRING(String),
        VECTOR {
            first_child: Option<ParsedNodeId>,
            last_child: Option<ParsedNodeId>,
        },
        BEGINARGS {
            first_child: Option<ParsedNodeId>,
            last_child: Option<ParsedNodeId>,
        },
    }

    struct ParsedNode {
        parent: ParsedNodeId,

        // you don't need these fields but they might be handy, as you wish
        next: Option<ParsedNodeId>,
        prev: Option<ParsedNodeId>,

        data: ParsedData,
    }

    struct ParsedTree {
        nodes: Vec<ParsedNode>,
    }


    impl ParsedData{
        fn parse(bytes : &[u8]) -> Result<ParsedTree, DataParseError> {
            let mut idx = 0;

            loop {
                if idx >= bytes.len() {
                    break;
                }

                let typeidx = idx;
                idx += 1;
                let data = match bytes[typeidx] {
                    0 => {
                        idx += 1;
                        ParsedData::U8(u8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(DataParseError{})?))
                    },
                    1 => {
                        idx += 2;
                        ParsedData::U16(u16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(DataParseError{})?))
                    },
                    2 => {
                        idx += 4;
                        ParsedData::U32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?))
                    },
                    3 => {
                        idx += 8;
                        ParsedData::U64(u64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?))
                    },
                    4 => {
                        idx += 1;
                        ParsedData::I8(i8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(DataParseError{})?))
                    },
                    5 => {
                        idx += 2;
                        ParsedData::I16(i16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(DataParseError{})?))
                    },
                    6 => {
                        idx += 4;
                        ParsedData::I32(i32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?))
                    },
                    7 => {
                        idx += 8;
                        ParsedData::I64(i64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?))
                    },
                    8 => {
                        idx += 4;
                        ParsedData::CHAR(char::from_u32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?)).unwrap())
                    },
                    _ => return Err(DataParseError {})
                };
            }
            return Err(DataParseError {  });
        }
        fn parse_node(bytes : &[u8], tree : &mut ParsedTree) -> Result<ParsedNode, DataParseError> {
            return Err(DataParseError { })

        }
    }
    macro_rules! encode {
        ($id:literal, $len:literal, $type:ty, $name:ident) => {
            fn $name(data : $type) -> [u8 ; $len]{
                
                const LENGTH : usize = $len;
                let converted_data = data.to_be_bytes();
                let mut returndata : [u8 ; LENGTH] = [0;LENGTH];
                returndata[0] = $id;
                for i in 0..returndata.len()-1 {
                    returndata[i+1] = converted_data[i];
                }
                return returndata

            }
        };
    }
    //Generate all the basic type encoding
    encode!(0,2, u8, encode_u8);
    encode!(1,3, u16, encode_u16);
    encode!(2,5, u32, encode_u32);
    encode!(3,9, u64, encode_u64);
    encode!(4,2, i8, encode_i8);
    encode!(5,3, i16, encode_i16);
    encode!(6,5, i32, encode_i32);
    encode!(7,9, i64, encode_i64);
    fn encode_char(data : char) -> [u8; 5]{
        const LENGTH : usize = 5;
        let mut returndata : [u8 ; LENGTH] = [0;LENGTH];
        returndata[0] = 8;
        let converted_data = (data as u32).to_be_bytes();
        for i in 0..LENGTH {
            returndata[i+1] = converted_data[i];
        }
        return returndata
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
