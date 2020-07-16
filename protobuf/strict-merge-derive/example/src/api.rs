use protobuf::{ProtobufResult, ProtobufError, ProtobufEnumStrict};
use protobuf::error::ProtobufError::WireError;
use std::fmt::{Debug, Display};
use crate::other_messages::{compound, Inner};

#[derive(Debug, PartialEq)]
#[derive(protobuf::StrictMerge)]
pub struct Compound {
    #[prototype = "double"]
    #[fieldnumber = 1]
    #[tagsize = 10]
    pub double_default: f64,
    #[prototype = "double"]
    #[fieldnumber = 2]
    #[tagsize = 10]
    pub double_non_default: f64,

    #[prototype = "enum"]
    #[fieldnumber = 3]
    #[tagsize = 10]
    pub enum_default: crate::other_messages::AnEnum,
    #[prototype = "enum"]
    #[fieldnumber = 4]
    #[tagsize = 10]
    pub enum_o_default: Option<crate::other_messages::AnEnum>,
    #[prototype = "enum"]
    #[fieldnumber = 5]
    #[tagsize = 10]
    pub enum_o_empty: Option<crate::other_messages::AnEnum>,

    #[prototype = "message"]
    #[fieldnumber = 6]
    #[tagsize = 1]
    pub message_default: Inner,
    #[prototype = "message"]
    #[fieldnumber = 7]
    #[tagsize = 1]
    pub message_o_default: Option<Inner>,
    #[prototype = "message"]
    #[fieldnumber = 8]
    #[tagsize = 1]
    pub message_o_empty: Option<Inner>,

    #[prototype = "oneof"]
    #[oneof = "double|double|9|1"]
    #[oneof = "_enum|enum|10|1"]
    #[oneof = "message|message|11|1"]
    #[tagsize = 10]
    pub one_of_double: compound::OneOfSomething,
    #[prototype = "oneof"]
    #[oneof = "double|double|12|1"]
    #[oneof = "_enum|enum|13|1"]
    #[oneof = "message|message|14|1"]
    #[tagsize = 10]
    pub one_of_enum: compound::OneOfSomething,
    #[prototype = "oneof"]
    #[oneof = "double|double|15|1"]
    #[oneof = "_enum|enum|16|1"]
    #[oneof = "message|message|17|2"]
    #[tagsize = 10]
    pub one_of_message: compound::OneOfSomething,

    #[prototype = "u8"]
    #[fieldnumber = 18]
    #[tagsize = 10]
    pub bytes_default: ::std::vec::Vec<u8>,
    #[prototype = "u8"]
    #[fieldnumber = 19]
    #[tagsize = 10]
    pub bytes_o_empty: ::std::vec::Vec<u8>,

    #[prototype = "repeated"]
    #[repeatedinner = "double"]
    #[tagsize = 10]
    #[fieldnumber = 20]
    pub vec_double_default: ::std::vec::Vec<f64>,
    #[prototype = "repeated"]
    #[repeatedinner = "double"]
    #[tagsize = 10]
    #[fieldnumber = 21]
    pub vec_double_o_empty: ::std::vec::Vec<f64>,

    #[prototype = "repeated"]
    #[repeatedinner = "message"]
    #[tagsize = 2]
    #[fieldnumber = 22]
    pub vec_message_default: ::std::vec::Vec<Inner>,
    #[prototype = "repeated"]
    #[repeatedinner = "message"]
    #[tagsize = 2]
    #[fieldnumber = 23]
    pub vec_message_o_empty: ::std::vec::Vec<Inner>,

    #[prototype = "repeated"]
    #[repeatedinner = "enum"]
    #[fieldnumber = 24]
    #[tagsize = 10]
    pub vec_enum_default: ::std::vec::Vec<crate::other_messages::AnEnum>,
    #[prototype = "repeated"]
    #[repeatedinner = "enum"]
    #[fieldnumber = 25]
    #[tagsize = 10]
    pub vec_enum_o_empty: ::std::vec::Vec<crate::other_messages::AnEnum>,

    #[prototype = "bool"]
    #[fieldnumber = 26]
    #[tagsize = 2]
    pub bool_true: bool,
    #[prototype = "bool"]
    #[fieldnumber = 27]
    #[tagsize = 2]
    pub bool_false: bool,
    #[prototype = "repeated"]
    #[repeatedinner = "bool"]
    #[fieldnumber = 28]
    #[tagsize = 2]
    pub vec_bool_default: ::std::vec::Vec<bool>,
    #[prototype = "repeated"]
    #[repeatedinner = "bool"]
    #[fieldnumber = 29]
    #[tagsize = 2]
    pub vec_bool_o_default: ::std::vec::Vec<bool>,

    #[prototype = "string"]
    #[fieldnumber = 30]
    #[tagsize = 1]
    pub string_default: std::string::String,
    #[prototype = "string"]
    #[fieldnumber = 31]
    #[tagsize = 1]
    pub string_default_o_empty: std::string::String,
}