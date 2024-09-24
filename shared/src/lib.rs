//shared/src/lib.rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod data {
    use std::{error::Error, fmt, num::{NonZero, NonZeroUsize}};

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
        parent: Option<ParsedNodeId>,

        // you don't need these fields but they might be handy, as you wish
        next: Option<ParsedNodeId>,
        prev: Option<ParsedNodeId>,

        data: ParsedData,
    }

    struct ParsedTree {
        nodes: Vec<ParsedNode>,
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


    impl ParsedData{

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

        fn parse(bytes : &[u8]) -> Result<ParsedTree, DataParseError> {
            let mut tree = ParsedTree { nodes : vec![]};
            let mut idx = 0;
            let mut previousid = None;
            let mut currentid = ParsedNodeId(NonZeroUsize::new(1).expect("1 was equal to 0?"));
            let mut currentparent : Option<ParsedNodeId> = None;

            loop {
                if idx >= bytes.len() {
                    break;
                }

                let typeidx = idx;
                idx += 1;
                match currentid.0.checked_add(1) {
                    None => {println!("Parsing tree id caused integer overflow"); return Err(DataParseError{}) },
                    _ => (),
                }
                let data = match bytes[typeidx] {
                    0 => {
                        let parsed = ParsedData::U8(u8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(DataParseError{})?));
                        idx += 1;
                        parsed
                    },
                    1 => {
                        let parsed = ParsedData::U16(u16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(DataParseError{})?));
                        idx += 2;
                        parsed
                    },
                    2 => {
                        let parsed = ParsedData::U32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?));
                        idx += 4;
                        parsed
                    },
                    3 => {
                        let parsed = ParsedData::U64(u64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?));
                        idx += 8;
                        parsed
                    },
                    4 => {
                        let parsed = ParsedData::I8(i8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(DataParseError{})?));
                        idx += 1;
                        parsed
                    },
                    5 => {
                        let parsed = ParsedData::I16(i16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(DataParseError{})?));
                        idx += 2;
                        parsed
                    },
                    6 => {
                        let parsed = ParsedData::I32(i32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?));
                        idx += 4;
                        parsed
                    },
                    7 => {
                        let parsed = ParsedData::I64(i64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?));
                        idx += 8;
                        parsed
                    },
                    8 => {
                        let parsed = ParsedData::CHAR(char::from_u32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?)).unwrap());
                        idx += 4;
                        parsed
                    },
                    _ => return Err(DataParseError {})
                };
                let mut node = ParsedNode {
                    parent : currentparent,
                    next : None,
                    data : data,
                    prev : previousid,


                };

            }
            return Err(DataParseError {  });
        }
        fn parse_node(bytes : &[u8], tree : &mut ParsedTree) -> Result<ParsedNode, DataParseError> {
            return Err(DataParseError { })

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
