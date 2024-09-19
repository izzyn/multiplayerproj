# BuffData
Byte (0) Type -> u8 integer (byte) describing the type (see table below):

0 -> u32 (length 4)
1 -> u64 (length 8)
2 -> I32 (length 4)
3 -> I64 (length 8)
4 -> String (length unknown*)
5 -> Array (length unknown*)
6 -> BeginArgs (length unkown*)


*In cases where the length is unknown, the first 4 bytes of the data vector are the length of the array

Byte (1 -> N) Data
