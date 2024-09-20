
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
                        ParsedData::U8(u8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(DataParseError{})?))
                    },
                    1 => {
                        ParsedData::U16(u16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(DataParseError{})?))
                    },
                    2 => {
                        ParsedData::U32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?))
                    },
                    3 => {
                        ParsedData::U64(u64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?))
                    },
                    4 => {
                        ParsedData::I8(i8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(DataParseError{})?))
                    },
                    5 => {
                        ParsedData::I16(i16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(DataParseError{})?))
                    },
                    6 => {
                        ParsedData::I32(i32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?))
                    },
                    7 => {
                        ParsedData::I64(i64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?))
                    },
                    8 => {
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
