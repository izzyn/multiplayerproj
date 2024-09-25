//shared/src/lib.rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod data {
    use core::f32;
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

    #[repr(u8)]
    enum DataIDs {
        U8,
        U16,
        U32,
        U64,
        I8,
        I16,
        I32,
        I64,
        CHAR,
        F32,
        F64,
        STRING,
        VECTOR,
    }
    impl TryFrom<u8> for DataIDs {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            return match value {
                0 => Ok(DataIDs::U8),
                1 => Ok(DataIDs::U16),
                2 => Ok(DataIDs::U32),
                3 => Ok(DataIDs::U64),
                4 => Ok(DataIDs::I8),
                5 => Ok(DataIDs::I16),
                6 => Ok(DataIDs::I32),
                7 => Ok(DataIDs::I64),
                8 => Ok(DataIDs::CHAR),
                9 => Ok(DataIDs::F32),
                10 => Ok(DataIDs::F64),
                11 => Ok(DataIDs::STRING),
                12 => Ok(DataIDs::VECTOR),
                _ => Err(())
            }
        }

    }
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
        F32(f32),
        F64(f64),
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
        ($id:ident, $len:literal, $type:ty, $name:ident) => {
            fn $name(data : $type) -> [u8 ; $len]{
                
                const LENGTH : usize = $len;
                let converted_data = data.to_be_bytes();
                let mut returndata : [u8 ; LENGTH] = [0;LENGTH];
                returndata[0] = DataIDs::$id as u8;
                for i in 0..returndata.len()-1 {
                    returndata[i+1] = converted_data[i];
                }
                return returndata

            }
        };
    }


    impl ParsedData{

        //Generate all the basic type encoding
        encode!(U8,2, u8, encode_u8);
        encode!(U16,3, u16, encode_u16);
        encode!(U32,5, u32, encode_u32);
        encode!(U64,9, u64, encode_u64);
        encode!(I8,2, i8, encode_i8);
        encode!(I16,3, i16, encode_i16);
        encode!(I32,5, i32, encode_i32);
        encode!(I64,9, i64, encode_i64);

        fn encode_char(data : char) -> [u8; 5]{
            const LENGTH : usize = 5;
            let mut returndata : [u8 ; LENGTH] = [0;LENGTH];
            returndata[0] = DataIDs::CHAR as u8;
            let converted_data = (data as u32).to_be_bytes();
            for i in 0..LENGTH {
                returndata[i+1] = converted_data[i];
            }
            return returndata
        }

        encode!(F32,5, f32, encode_f32);
        encode!(F64,9, i64, encode_f64);

        fn parse(bytes : &[u8]) -> Result<ParsedTree, DataParseError> {
            let mut tree = ParsedTree { nodes : vec![]};
            let mut idx = 0;
            let mut previousid = None;
            let currentid = ParsedNodeId(NonZeroUsize::MIN);
            let currentparent : Option<ParsedNodeId> = None;

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
                let data = match DataIDs::try_from(bytes[typeidx]).ok().ok_or(DataParseError{})?{
                    DataIDs::U8 => {
                        let parsed = ParsedData::U8(u8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(DataParseError{})?));
                        idx += 1;
                        parsed
                    },
                    DataIDs::U16 => {
                        let parsed = ParsedData::U16(u16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(DataParseError{})?));
                        idx += 2;
                        parsed
                    },
                    DataIDs::U32 => {
                        let parsed = ParsedData::U32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?));
                        idx += 4;
                        parsed
                    },
                    DataIDs::U64 => {
                        let parsed = ParsedData::U64(u64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?));
                        idx += 8;
                        parsed
                    },
                    DataIDs::I8 => {
                        let parsed = ParsedData::I8(i8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(DataParseError{})?));
                        idx += 1;
                        parsed
                    },
                    DataIDs::I16 => {
                        let parsed = ParsedData::I16(i16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(DataParseError{})?));
                        idx += 2;
                        parsed
                    },
                    DataIDs::I32 => {
                        let parsed = ParsedData::I32(i32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?));
                        idx += 4;
                        parsed
                    },
                    DataIDs::I64 => {
                        let parsed = ParsedData::I64(i64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?));
                        idx += 8;
                        parsed
                    },
                    DataIDs::CHAR => {
                        let parsed = ParsedData::CHAR(char::from_u32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?)).unwrap());
                        idx += 4;
                        parsed
                    },
                    DataIDs::F32 => {
                        let parsed = ParsedData::F32(f32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(DataParseError{})?));
                        idx += 4;
                        parsed
                    },
                    DataIDs::F64 => {
                        let parsed = ParsedData::F64(f64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(DataParseError{})?));
                        idx += 8;
                        parsed
                    },
                    _ => return Err(DataParseError {})
                };

                //TODO: Rework this part to add support for vectors
                if tree.nodes.len() != 0 {
                    tree.nodes.last_mut().unwrap().next = Some(currentid);
                }
                let node = ParsedNode {
                    parent : currentparent,
                    next : None,
                    data : data,
                    prev : previousid,
                };
                previousid = Some(currentid);
                tree.nodes.push(node);

            }
            return Ok(tree);
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
