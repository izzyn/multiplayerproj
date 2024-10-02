//shared/src/lib.rs
pub mod data {
use core::f32;
use std::{error::Error, fmt, num::NonZeroUsize, str::FromStr};

#[derive(Debug)]
pub struct DataParseError {
        message : String
    }

impl Error for DataParseError {}

impl fmt::Display for DataParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


#[repr(u8)]
pub enum DataIDs {
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
    ENDPKG,
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
            13 => Ok(DataIDs::ENDPKG),
            _ => Err(())
        }
    }

}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedNodeId(NonZeroUsize);

#[derive(Debug, Clone)]
pub enum ParsedData {
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

pub struct ParsedNode {
    pub parent: Option<ParsedNodeId>,

    // you don't need these fields but they might be handy, as you wish
    pub next: Option<ParsedNodeId>,
    pub prev: Option<ParsedNodeId>,

    pub data: ParsedData,
}

pub struct ParsedTree {
    pub nodes: Vec<ParsedNode>,
}

macro_rules! encode {
    ($id:ident, $type:ty) => {
        ::paste::paste!{
            pub fn [<encode_ $type>](data : $type) -> [u8 ; size_of::<$type>()+1]{
                
                const LENGTH : usize = size_of::<$type>()+1;
                let converted_data = data.to_be_bytes();
                let mut returndata : [u8 ; LENGTH] = [0;LENGTH];
                returndata[0] = DataIDs::$id as u8;
                for i in 0..returndata.len()-1 {
                    returndata[i+1] = converted_data[i];
                }
                return returndata

            }

        }
    };
}

encode!(U8, u8);
encode!(U16, u16);
encode!(U32, u32);
encode!(U64, u64);
encode!(I8, i8);
encode!(I16, i16);
encode!(I32, i32);
encode!(I64, i64);
pub fn encode_char(data : char) -> [u8; size_of::<char>()+1]{
    const LENGTH : usize = size_of::<char>()+1;
    let mut returndata : [u8 ; LENGTH] = [0;LENGTH];
    returndata[0] = DataIDs::CHAR as u8;
    let converted_data = (data as u32).to_be_bytes();
    for i in 0..LENGTH-1 {
        returndata[i+1] = converted_data[i];
    }
    return returndata
}

encode!(F32, f32);
encode!(F64, f64);


//pub fn format_data()
//Generate all the basic type encoding
pub fn parse(bytes : &[u8]) -> Result<ParsedTree, DataParseError> {
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
            None => {return Err(DataParseError{
                    message : "Parsing tree id caused integer overflow".to_string()
                }) },
            _ => (),
        }
        let datatypeerr = "Error when parsing ".to_string();
        let data = match DataIDs::try_from(bytes[typeidx]).ok().ok_or(DataParseError{message : "Attempted to parse invalid datatype".to_string()})?{
            DataIDs::U8 => {
                let parsed = ParsedData::U8(u8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(
                        DataParseError{message : format!("{datatypeerr} U8")})?));
                idx += 1;
                parsed
            },
            DataIDs::U16 => {
                let parsed = ParsedData::U16(u16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(
                        DataParseError{message : format!("{datatypeerr} U16")})?));
                idx += 2;
                parsed
            },
            DataIDs::U32 => {
                let parsed = ParsedData::U32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(
                        DataParseError{message : format!("{datatypeerr} U32")})?));
                idx += 4;
                parsed
            },
            DataIDs::U64 => {
                let parsed = ParsedData::U64(u64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(
                        DataParseError{message : format!("{datatypeerr} U64")})?));
                idx += 8;
                parsed
            },
            DataIDs::I8 => {
                let parsed = ParsedData::I8(i8::from_be_bytes(*bytes[idx..].first_chunk::<1>().ok_or(
                        DataParseError{message : format!("{datatypeerr} I8")})?));
                idx += 1;
                parsed
            },
            DataIDs::I16 => {
                let parsed = ParsedData::I16(i16::from_be_bytes(*bytes[idx..].first_chunk::<2>().ok_or(
                        DataParseError{message : format!("{datatypeerr} I16")})?));
                idx += 2;
                parsed
            },
            DataIDs::I32 => {
                let parsed = ParsedData::I32(i32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(
                        DataParseError{message : format!("{datatypeerr} I32")})?));
                idx += 4;
                parsed
            },
            DataIDs::I64 => {
                let parsed = ParsedData::I64(i64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(
                        DataParseError{message : format!("{datatypeerr} I64")})?));
                idx += 8;
                parsed
            },
            DataIDs::CHAR => {
                let parsed = ParsedData::CHAR(char::from_u32(u32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(
                        DataParseError{message : format!("{datatypeerr} CHAR")})?)).unwrap());
                idx += 4;
                parsed
            },
            DataIDs::F32 => {
                let parsed = ParsedData::F32(f32::from_be_bytes(*bytes[idx..].first_chunk::<4>().ok_or(
                        DataParseError{message : format!("{datatypeerr} F32")})?));
                idx += 4;
                parsed
            },
            DataIDs::F64 => {
                let parsed = ParsedData::F64(f64::from_be_bytes(*bytes[idx..].first_chunk::<8>().ok_or(
                        DataParseError{message : format!("{datatypeerr} F64")})?));
                idx += 8;
                parsed
            },
            DataIDs::ENDPKG => {
                return Ok(tree)
            }
            _ => return Err(DataParseError {message : String::from("Attempted to parse an invalid datatype")})
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
    return Err(DataParseError {message : String::from("Recieved data without a proper end signal.")});
}

impl ParsedData{
    fn parse_node(bytes : &[u8], tree : &mut ParsedTree) -> Result<ParsedNode, DataParseError> {
            return Err(DataParseError { message : String::from("Couldn't parse node'")})

        }
    }
    

}

macro_rules! test_encoding {
    ($type : ty, $enumvalue : ident) => {
        ::paste::paste! {
            #[test]
            fn [<test_encoding_ $type>](){
                const SIZE : usize = size_of::<$type>()+2;
                println!("{}", SIZE);
                let mut buff : [u8 ; SIZE] = [0; SIZE];
                
                for j in 0..[<TESTS_ $type>].len() {
                    println!("{}", [<TESTS_ $type>].len());
                    let test_bytes = [<encode_ $type>]([<TESTS_ $type>][j]);
                    println!("{}", test_bytes.len());
                    for i in 0..test_bytes.len() {
                        buff[i] = test_bytes[i];
                    }
                    buff[SIZE - 1] = DataIDs::ENDPKG as u8;
                    let tree = match parse(&buff) {
                        Ok(t) => t,
                        Err(e) => panic!("{:?}", e),
                    };
                    
                    let data = tree.nodes[0].data.clone();
                    match data {
                        ParsedData::$enumvalue(contents) => assert_eq!([<TESTS_ $type>][j], contents),
                        _ => panic!("Incorrect datatype parsed"),
                    }
                }
            }

        }
    };
}
#[cfg(test)]
mod tests {
    use std::{i32, u8};

    use super::*;
    use data::*;

    //Test values for primitive types
    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_u8 : [u8; 5] = [2,u8::MAX, 100,5,6];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_u16 : [u16; 5] = [u8::MAX as u16,u16::MAX,4,5,6];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_u32 : [u32; 5] = [2,u32::MAX,4,5,6];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_u64 : [u64; 5] = [2,u64::MAX,4,5,6];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_i8 : [i8; 5] = [-23,i8::MAX,4,5,6];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_i16 : [i16; 5] = [2,i16::MAX,-999,5,6];
    
    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_i32 : [i32; 5] = [2,i32::MAX,-2301578,5,6];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_i64 : [i64; 5] = [2,i64::MAX,4,5,6];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_f32 : [f32; 5] = [-23.0,i8::MAX as f32,0.333333333,6.0, 0.0];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_f64 : [f64; 5] = [2.0,i16::MAX as f64,-999.0,30.0,-2319.392048];

    #[allow(non_snake_case, non_upper_case_globals)]
    const TESTS_char : [char; 5] = ['Ä','\\', '\n', 'å', 'A'];
    //Test functions for primitive types
    test_encoding!(u8, U8);
    test_encoding!(u16, U16);
    test_encoding!(u32, U32);
    test_encoding!(u64, U64);
    test_encoding!(i8,  I8);
    test_encoding!(i16, I16);
    test_encoding!(i32, I32);
    test_encoding!(i64, I64);
    test_encoding!(f32, F32);
    test_encoding!(f64, F64);
    test_encoding!(char, CHAR);
}
