use std::num::NonZeroUsize;

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
    SIGNAL(u32),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedNodeId(pub NonZeroUsize);

#[derive(Debug)]
pub struct ParsedNode {
    pub parent: Option<ParsedNodeId>,

    // you don't need these fields but they might be handy, as you wish
    pub next: Option<ParsedNodeId>,
    pub prev: Option<ParsedNodeId>,

    pub data: ParsedData,
}

#[derive(Debug)]
pub struct ParsedTree {
    pub nodes: Vec<ParsedNode>,
}
