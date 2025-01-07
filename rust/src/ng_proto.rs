// @generated
// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueMsg {
    #[prost(int64, tag="1")]
    pub ts: i64,
    #[prost(map="string, string", tag="2")]
    pub headers: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonResponse {
    #[prost(int32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfoProto {
    #[prost(int64, tag="1")]
    pub tenant_id: i64,
    #[prost(int64, tag="2")]
    pub product_id: i64,
    #[prost(int64, tag="4")]
    pub device_id: i64,
    #[prost(string, tag="5")]
    pub device_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub device_type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub additional_info: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ProductInfoProto {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionInfoProto {
    #[prost(enumeration="TransportType", tag="1")]
    pub transport: i32,
    #[prost(string, tag="2")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub tenant_id: i64,
    #[prost(int64, tag="5")]
    pub device_id: i64,
    #[prost(int64, tag="6")]
    pub product_id: i64,
    #[prost(string, tag="7")]
    pub device_name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub device_type: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionEvent {
    Unspecified = 0,
    Open = 1,
    Closed = 2,
}
impl SessionEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "SESSION_EVENT_UNSPECIFIED",
            Self::Open => "OPEN",
            Self::Closed => "CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SESSION_EVENT_UNSPECIFIED" => Some(Self::Unspecified),
            "OPEN" => Some(Self::Open),
            "CLOSED" => Some(Self::Closed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionType {
    Unspecified = 0,
    Sync = 1,
    Async = 2,
}
impl SessionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "SESSION_TYPE_UNSPECIFIED",
            Self::Sync => "SYNC",
            Self::Async => "ASYNC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SESSION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SYNC" => Some(Self::Sync),
            "ASYNC" => Some(Self::Async),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransportType {
    Unspecified = 0,
    Tcp = 1,
    Mqtt = 2,
    Http = 3,
    Coap = 4,
    Lwm2m = 5,
    Snmp = 6,
    Ws = 7,
}
impl TransportType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "TRANSPORT_TYPE_UNSPECIFIED",
            Self::Tcp => "TCP",
            Self::Mqtt => "MQTT",
            Self::Http => "HTTP",
            Self::Coap => "COAP",
            Self::Lwm2m => "LWM2M",
            Self::Snmp => "SNMP",
            Self::Ws => "WS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSPORT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TCP" => Some(Self::Tcp),
            "MQTT" => Some(Self::Mqtt),
            "HTTP" => Some(Self::Http),
            "COAP" => Some(Self::Coap),
            "LWM2M" => Some(Self::Lwm2m),
            "SNMP" => Some(Self::Snmp),
            "WS" => Some(Self::Ws),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HousekeeperTaskType {
    HousekeeperTaskUnset = 0,
    EntityCleanup = 1,
}
impl HousekeeperTaskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::HousekeeperTaskUnset => "HOUSEKEEPER_TASK_UNSET",
            Self::EntityCleanup => "ENTITY_CLEANUP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOUSEKEEPER_TASK_UNSET" => Some(Self::HousekeeperTaskUnset),
            "ENTITY_CLEANUP" => Some(Self::EntityCleanup),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceProvisionConnectRequest {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceProvisionConnectResponse {
    #[prost(message, optional, tag="1")]
    pub common: ::core::option::Option<CommonResponse>,
    #[prost(int64, tag="2")]
    pub product_id: i64,
    #[prost(int64, tag="3")]
    pub tenant_id: i64,
    #[prost(string, tag="4")]
    pub device_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceProvisionRequest {
    #[prost(bool, tag="1")]
    pub gateway: bool,
    #[prost(string, tag="2")]
    pub device_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub credentials: ::core::option::Option<DeviceCredentialsProto>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceCredentialsProto {
    #[prost(int64, tag="1")]
    pub mode: i64,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub csr: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceProvisionResponse {
    #[prost(message, optional, tag="1")]
    pub common: ::core::option::Option<CommonResponse>,
    #[prost(message, optional, tag="2")]
    pub device_info: ::core::option::Option<DeviceInfoProto>,
    #[prost(message, optional, tag="3")]
    pub credentials: ::core::option::Option<DeviceCredentialsResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceCredentialsResponse {
    #[prost(int64, tag="1")]
    pub mode: i64,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub ca: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub certificate: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(oneof="enum_value::Value", tags="2, 3, 4, 5")]
    pub value: ::core::option::Option<enum_value::Value>,
}
/// Nested message and enum types in `EnumValue`.
pub mod enum_value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="2")]
        StringValue(::prost::alloc::string::String),
        #[prost(int64, tag="3")]
        IntValue(i64),
        #[prost(float, tag="4")]
        FloatValue(f32),
        #[prost(bool, tag="5")]
        BoolValue(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumArrayType {
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<EnumValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringArray {
    #[prost(string, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntArray {
    #[prost(int64, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatArray {
    #[prost(float, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolArray {
    #[prost(bool, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    #[prost(enumeration="ConditionType", tag="1")]
    pub r#type: i32,
    #[prost(enumeration="Operator", tag="2")]
    pub operator: i32,
    #[prost(message, repeated, tag="11")]
    pub sub_conditions: ::prost::alloc::vec::Vec<Condition>,
    #[prost(oneof="condition::Value", tags="3, 4, 5, 6, 7, 8, 9, 10")]
    pub value: ::core::option::Option<condition::Value>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="3")]
        StringValue(::prost::alloc::string::String),
        #[prost(int64, tag="4")]
        IntValue(i64),
        #[prost(float, tag="5")]
        FloatValue(f32),
        #[prost(bool, tag="6")]
        BoolValue(bool),
        #[prost(message, tag="7")]
        StringArray(super::StringArray),
        #[prost(message, tag="8")]
        IntArray(super::IntArray),
        #[prost(message, tag="9")]
        FloatArray(super::FloatArray),
        #[prost(message, tag="10")]
        BoolArray(super::BoolArray),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extension {
    #[prost(string, tag="1")]
    pub field: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(enumeration="ExtensionType", tag="3")]
    pub r#type: i32,
    #[prost(message, repeated, tag="4")]
    pub enum_info: ::prost::alloc::vec::Vec<EnumValue>,
    #[prost(message, repeated, tag="5")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    #[prost(bool, tag="6")]
    pub required: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriverMetadata {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub extensions: ::prost::alloc::vec::Vec<Extension>,
    #[prost(message, repeated, tag="6")]
    pub data_point_extensions: ::prost::alloc::vec::Vec<Extension>,
    #[prost(message, repeated, tag="7")]
    pub action_extensions: ::prost::alloc::vec::Vec<Extension>,
    #[prost(message, repeated, tag="8")]
    pub event_extensions: ::prost::alloc::vec::Vec<Extension>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExtensionType {
    Unspecified = 0,
    String = 1,
    Int = 2,
    Float = 3,
    Bool = 4,
    StringArray = 5,
    IntegerArray = 6,
    FloatArray = 7,
    BooleanArray = 8,
    Enum = 9,
    EnumArray = 10,
}
impl ExtensionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "EXTENSION_TYPE_UNSPECIFIED",
            Self::String => "STRING",
            Self::Int => "INT",
            Self::Float => "FLOAT",
            Self::Bool => "BOOL",
            Self::StringArray => "STRING_ARRAY",
            Self::IntegerArray => "INTEGER_ARRAY",
            Self::FloatArray => "FLOAT_ARRAY",
            Self::BooleanArray => "BOOLEAN_ARRAY",
            Self::Enum => "ENUM",
            Self::EnumArray => "ENUM_ARRAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXTENSION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "STRING" => Some(Self::String),
            "INT" => Some(Self::Int),
            "FLOAT" => Some(Self::Float),
            "BOOL" => Some(Self::Bool),
            "STRING_ARRAY" => Some(Self::StringArray),
            "INTEGER_ARRAY" => Some(Self::IntegerArray),
            "FLOAT_ARRAY" => Some(Self::FloatArray),
            "BOOLEAN_ARRAY" => Some(Self::BooleanArray),
            "ENUM" => Some(Self::Enum),
            "ENUM_ARRAY" => Some(Self::EnumArray),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Operator {
    Unspecified = 0,
    Eq = 1,
    Neq = 2,
    Gt = 3,
    Gte = 4,
    Lt = 5,
    Lte = 6,
    Contains = 7,
    Prefix = 8,
    Suffix = 9,
    Regex = 10,
    In = 11,
    NotIn = 12,
    Between = 13,
    NotBetween = 14,
    NotNull = 15,
}
impl Operator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "OPERATOR_UNSPECIFIED",
            Self::Eq => "EQ",
            Self::Neq => "NEQ",
            Self::Gt => "GT",
            Self::Gte => "GTE",
            Self::Lt => "LT",
            Self::Lte => "LTE",
            Self::Contains => "CONTAINS",
            Self::Prefix => "PREFIX",
            Self::Suffix => "SUFFIX",
            Self::Regex => "REGEX",
            Self::In => "IN",
            Self::NotIn => "NOT_IN",
            Self::Between => "BETWEEN",
            Self::NotBetween => "NOT_BETWEEN",
            Self::NotNull => "NOT_NULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATOR_UNSPECIFIED" => Some(Self::Unspecified),
            "EQ" => Some(Self::Eq),
            "NEQ" => Some(Self::Neq),
            "GT" => Some(Self::Gt),
            "GTE" => Some(Self::Gte),
            "LT" => Some(Self::Lt),
            "LTE" => Some(Self::Lte),
            "CONTAINS" => Some(Self::Contains),
            "PREFIX" => Some(Self::Prefix),
            "SUFFIX" => Some(Self::Suffix),
            "REGEX" => Some(Self::Regex),
            "IN" => Some(Self::In),
            "NOT_IN" => Some(Self::NotIn),
            "BETWEEN" => Some(Self::Between),
            "NOT_BETWEEN" => Some(Self::NotBetween),
            "NOT_NULL" => Some(Self::NotNull),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConditionType {
    Unspecified = 0,
    And = 1,
    Or = 2,
}
impl ConditionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "CONDITION_TYPE_UNSPECIFIED",
            Self::And => "AND",
            Self::Or => "OR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONDITION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "AND" => Some(Self::And),
            "OR" => Some(Self::Or),
            _ => None,
        }
    }
}
include!("ng_proto.serde.rs");
// @@protoc_insertion_point(module)
