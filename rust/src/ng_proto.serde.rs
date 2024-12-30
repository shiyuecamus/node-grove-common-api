// @generated
impl serde::Serialize for BoolArray {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.BoolArray", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BoolArray {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BoolArray;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.BoolArray")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BoolArray, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BoolArray {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.BoolArray", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommonResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.CommonResponse", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.msg.is_empty() {
            struct_ser.serialize_field("msg", &self.msg)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommonResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "msg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Msg,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommonResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.CommonResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CommonResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut msg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CommonResponse {
                    code: code__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.CommonResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.operator != 0 {
            len += 1;
        }
        if !self.sub_conditions.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.Condition", len)?;
        if self.r#type != 0 {
            let v = ConditionType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.operator != 0 {
            let v = Operator::try_from(self.operator)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.operator)))?;
            struct_ser.serialize_field("operator", &v)?;
        }
        if !self.sub_conditions.is_empty() {
            struct_ser.serialize_field("subConditions", &self.sub_conditions)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                condition::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                condition::Value::IntValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("intValue", ToString::to_string(&v).as_str())?;
                }
                condition::Value::FloatValue(v) => {
                    struct_ser.serialize_field("floatValue", v)?;
                }
                condition::Value::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
                condition::Value::StringArray(v) => {
                    struct_ser.serialize_field("stringArray", v)?;
                }
                condition::Value::IntArray(v) => {
                    struct_ser.serialize_field("intArray", v)?;
                }
                condition::Value::FloatArray(v) => {
                    struct_ser.serialize_field("floatArray", v)?;
                }
                condition::Value::BoolArray(v) => {
                    struct_ser.serialize_field("boolArray", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "operator",
            "subConditions",
            "stringValue",
            "intValue",
            "floatValue",
            "boolValue",
            "stringArray",
            "intArray",
            "floatArray",
            "boolArray",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Operator,
            SubConditions,
            StringValue,
            IntValue,
            FloatValue,
            BoolValue,
            StringArray,
            IntArray,
            FloatArray,
            BoolArray,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "operator" => Ok(GeneratedField::Operator),
                            "subConditions" => Ok(GeneratedField::SubConditions),
                            "stringValue" => Ok(GeneratedField::StringValue),
                            "intValue" => Ok(GeneratedField::IntValue),
                            "floatValue" => Ok(GeneratedField::FloatValue),
                            "boolValue" => Ok(GeneratedField::BoolValue),
                            "stringArray" => Ok(GeneratedField::StringArray),
                            "intArray" => Ok(GeneratedField::IntArray),
                            "floatArray" => Ok(GeneratedField::FloatArray),
                            "boolArray" => Ok(GeneratedField::BoolArray),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.Condition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut operator__ = None;
                let mut sub_conditions__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<ConditionType>()? as i32);
                        }
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = Some(map_.next_value::<Operator>()? as i32);
                        }
                        GeneratedField::SubConditions => {
                            if sub_conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subConditions"));
                            }
                            sub_conditions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::Value::StringValue);
                        }
                        GeneratedField::IntValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| condition::Value::IntValue(x.0));
                        }
                        GeneratedField::FloatValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| condition::Value::FloatValue(x.0));
                        }
                        GeneratedField::BoolValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::Value::BoolValue);
                        }
                        GeneratedField::StringArray => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringArray"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::Value::StringArray)
;
                        }
                        GeneratedField::IntArray => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intArray"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::Value::IntArray)
;
                        }
                        GeneratedField::FloatArray => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatArray"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::Value::FloatArray)
;
                        }
                        GeneratedField::BoolArray => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolArray"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::Value::BoolArray)
;
                        }
                    }
                }
                Ok(Condition {
                    r#type: r#type__.unwrap_or_default(),
                    operator: operator__.unwrap_or_default(),
                    sub_conditions: sub_conditions__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConditionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::And => "AND",
            Self::Or => "OR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConditionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AND",
            "OR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConditionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AND" => Ok(ConditionType::And),
                    "OR" => Ok(ConditionType::Or),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceCredentialsProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mode != 0 {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.csr.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.DeviceCredentialsProto", len)?;
        if self.mode != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("mode", ToString::to_string(&self.mode).as_str())?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.csr.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("csr", pbjson::private::base64::encode(&self.csr).as_str())?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceCredentialsProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mode",
            "clientId",
            "csr",
            "username",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mode,
            ClientId,
            Csr,
            Username,
            Password,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "mode" => Ok(GeneratedField::Mode),
                            "clientId" => Ok(GeneratedField::ClientId),
                            "csr" => Ok(GeneratedField::Csr),
                            "username" => Ok(GeneratedField::Username),
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceCredentialsProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.DeviceCredentialsProto")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceCredentialsProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mode__ = None;
                let mut client_id__ = None;
                let mut csr__ = None;
                let mut username__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Csr => {
                            if csr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("csr"));
                            }
                            csr__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceCredentialsProto {
                    mode: mode__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    csr: csr__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.DeviceCredentialsProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceCredentialsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mode != 0 {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.ca.is_empty() {
            len += 1;
        }
        if !self.certificate.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.DeviceCredentialsResponse", len)?;
        if self.mode != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("mode", ToString::to_string(&self.mode).as_str())?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.ca.is_empty() {
            struct_ser.serialize_field("ca", &self.ca)?;
        }
        if !self.certificate.is_empty() {
            struct_ser.serialize_field("certificate", &self.certificate)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceCredentialsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mode",
            "clientId",
            "ca",
            "certificate",
            "username",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mode,
            ClientId,
            Ca,
            Certificate,
            Username,
            Password,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "mode" => Ok(GeneratedField::Mode),
                            "clientId" => Ok(GeneratedField::ClientId),
                            "ca" => Ok(GeneratedField::Ca),
                            "certificate" => Ok(GeneratedField::Certificate),
                            "username" => Ok(GeneratedField::Username),
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceCredentialsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.DeviceCredentialsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceCredentialsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mode__ = None;
                let mut client_id__ = None;
                let mut ca__ = None;
                let mut certificate__ = None;
                let mut username__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ca => {
                            if ca__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ca"));
                            }
                            ca__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Certificate => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificate"));
                            }
                            certificate__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceCredentialsResponse {
                    mode: mode__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    ca: ca__.unwrap_or_default(),
                    certificate: certificate__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.DeviceCredentialsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceInfoProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tenant_id != 0 {
            len += 1;
        }
        if self.product_id != 0 {
            len += 1;
        }
        if self.device_id != 0 {
            len += 1;
        }
        if !self.device_name.is_empty() {
            len += 1;
        }
        if !self.device_type.is_empty() {
            len += 1;
        }
        if !self.additional_info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.DeviceInfoProto", len)?;
        if self.tenant_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tenantId", ToString::to_string(&self.tenant_id).as_str())?;
        }
        if self.product_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("productId", ToString::to_string(&self.product_id).as_str())?;
        }
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if !self.device_name.is_empty() {
            struct_ser.serialize_field("deviceName", &self.device_name)?;
        }
        if !self.device_type.is_empty() {
            struct_ser.serialize_field("deviceType", &self.device_type)?;
        }
        if !self.additional_info.is_empty() {
            struct_ser.serialize_field("additionalInfo", &self.additional_info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceInfoProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenantId",
            "productId",
            "deviceId",
            "deviceName",
            "deviceType",
            "additionalInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TenantId,
            ProductId,
            DeviceId,
            DeviceName,
            DeviceType,
            AdditionalInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenantId" => Ok(GeneratedField::TenantId),
                            "productId" => Ok(GeneratedField::ProductId),
                            "deviceId" => Ok(GeneratedField::DeviceId),
                            "deviceName" => Ok(GeneratedField::DeviceName),
                            "deviceType" => Ok(GeneratedField::DeviceType),
                            "additionalInfo" => Ok(GeneratedField::AdditionalInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceInfoProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.DeviceInfoProto")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceInfoProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant_id__ = None;
                let mut product_id__ = None;
                let mut device_id__ = None;
                let mut device_name__ = None;
                let mut device_type__ = None;
                let mut additional_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TenantId => {
                            if tenant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenantId"));
                            }
                            tenant_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceName => {
                            if device_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceName"));
                            }
                            device_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeviceType => {
                            if device_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceType"));
                            }
                            device_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdditionalInfo => {
                            if additional_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalInfo"));
                            }
                            additional_info__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceInfoProto {
                    tenant_id: tenant_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    device_id: device_id__.unwrap_or_default(),
                    device_name: device_name__.unwrap_or_default(),
                    device_type: device_type__.unwrap_or_default(),
                    additional_info: additional_info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.DeviceInfoProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceProvisionConnectRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.secret.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.DeviceProvisionConnectRequest", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.secret.is_empty() {
            struct_ser.serialize_field("secret", &self.secret)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceProvisionConnectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "secret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Secret,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "secret" => Ok(GeneratedField::Secret),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceProvisionConnectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.DeviceProvisionConnectRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceProvisionConnectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut secret__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceProvisionConnectRequest {
                    key: key__.unwrap_or_default(),
                    secret: secret__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.DeviceProvisionConnectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceProvisionConnectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        if self.product_id != 0 {
            len += 1;
        }
        if self.tenant_id != 0 {
            len += 1;
        }
        if !self.device_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.DeviceProvisionConnectResponse", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        if self.product_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("productId", ToString::to_string(&self.product_id).as_str())?;
        }
        if self.tenant_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tenantId", ToString::to_string(&self.tenant_id).as_str())?;
        }
        if !self.device_type.is_empty() {
            struct_ser.serialize_field("deviceType", &self.device_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceProvisionConnectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
            "productId",
            "tenantId",
            "deviceType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
            ProductId,
            TenantId,
            DeviceType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            "productId" => Ok(GeneratedField::ProductId),
                            "tenantId" => Ok(GeneratedField::TenantId),
                            "deviceType" => Ok(GeneratedField::DeviceType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceProvisionConnectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.DeviceProvisionConnectResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceProvisionConnectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                let mut product_id__ = None;
                let mut tenant_id__ = None;
                let mut device_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TenantId => {
                            if tenant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenantId"));
                            }
                            tenant_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceType => {
                            if device_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceType"));
                            }
                            device_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceProvisionConnectResponse {
                    common: common__,
                    product_id: product_id__.unwrap_or_default(),
                    tenant_id: tenant_id__.unwrap_or_default(),
                    device_type: device_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.DeviceProvisionConnectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceProvisionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gateway {
            len += 1;
        }
        if !self.device_name.is_empty() {
            len += 1;
        }
        if self.credentials.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.DeviceProvisionRequest", len)?;
        if self.gateway {
            struct_ser.serialize_field("gateway", &self.gateway)?;
        }
        if !self.device_name.is_empty() {
            struct_ser.serialize_field("deviceName", &self.device_name)?;
        }
        if let Some(v) = self.credentials.as_ref() {
            struct_ser.serialize_field("credentials", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceProvisionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gateway",
            "deviceName",
            "credentials",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Gateway,
            DeviceName,
            Credentials,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "gateway" => Ok(GeneratedField::Gateway),
                            "deviceName" => Ok(GeneratedField::DeviceName),
                            "credentials" => Ok(GeneratedField::Credentials),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceProvisionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.DeviceProvisionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceProvisionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gateway__ = None;
                let mut device_name__ = None;
                let mut credentials__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Gateway => {
                            if gateway__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gateway"));
                            }
                            gateway__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeviceName => {
                            if device_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceName"));
                            }
                            device_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeviceProvisionRequest {
                    gateway: gateway__.unwrap_or_default(),
                    device_name: device_name__.unwrap_or_default(),
                    credentials: credentials__,
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.DeviceProvisionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceProvisionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common.is_some() {
            len += 1;
        }
        if self.device_info.is_some() {
            len += 1;
        }
        if self.credentials.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.DeviceProvisionResponse", len)?;
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        if let Some(v) = self.device_info.as_ref() {
            struct_ser.serialize_field("deviceInfo", v)?;
        }
        if let Some(v) = self.credentials.as_ref() {
            struct_ser.serialize_field("credentials", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceProvisionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common",
            "deviceInfo",
            "credentials",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Common,
            DeviceInfo,
            Credentials,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "common" => Ok(GeneratedField::Common),
                            "deviceInfo" => Ok(GeneratedField::DeviceInfo),
                            "credentials" => Ok(GeneratedField::Credentials),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceProvisionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.DeviceProvisionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceProvisionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common__ = None;
                let mut device_info__ = None;
                let mut credentials__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                        GeneratedField::DeviceInfo => {
                            if device_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceInfo"));
                            }
                            device_info__ = map_.next_value()?;
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeviceProvisionResponse {
                    common: common__,
                    device_info: device_info__,
                    credentials: credentials__,
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.DeviceProvisionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DriverMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        if !self.data_point_extensions.is_empty() {
            len += 1;
        }
        if !self.action_extensions.is_empty() {
            len += 1;
        }
        if !self.event_extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.DriverMetadata", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        if !self.data_point_extensions.is_empty() {
            struct_ser.serialize_field("dataPointExtensions", &self.data_point_extensions)?;
        }
        if !self.action_extensions.is_empty() {
            struct_ser.serialize_field("actionExtensions", &self.action_extensions)?;
        }
        if !self.event_extensions.is_empty() {
            struct_ser.serialize_field("eventExtensions", &self.event_extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DriverMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "version",
            "extensions",
            "dataPointExtensions",
            "actionExtensions",
            "eventExtensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Version,
            Extensions,
            DataPointExtensions,
            ActionExtensions,
            EventExtensions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "version" => Ok(GeneratedField::Version),
                            "extensions" => Ok(GeneratedField::Extensions),
                            "dataPointExtensions" => Ok(GeneratedField::DataPointExtensions),
                            "actionExtensions" => Ok(GeneratedField::ActionExtensions),
                            "eventExtensions" => Ok(GeneratedField::EventExtensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DriverMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.DriverMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DriverMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut version__ = None;
                let mut extensions__ = None;
                let mut data_point_extensions__ = None;
                let mut action_extensions__ = None;
                let mut event_extensions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataPointExtensions => {
                            if data_point_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataPointExtensions"));
                            }
                            data_point_extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionExtensions => {
                            if action_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionExtensions"));
                            }
                            action_extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventExtensions => {
                            if event_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventExtensions"));
                            }
                            event_extensions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DriverMetadata {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    extensions: extensions__.unwrap_or_default(),
                    data_point_extensions: data_point_extensions__.unwrap_or_default(),
                    action_extensions: action_extensions__.unwrap_or_default(),
                    event_extensions: event_extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.DriverMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumArrayType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.EnumArrayType", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumArrayType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumArrayType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.EnumArrayType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnumArrayType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EnumArrayType {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.EnumArrayType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.EnumValue", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                enum_value::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                enum_value::Value::IntValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("intValue", ToString::to_string(&v).as_str())?;
                }
                enum_value::Value::FloatValue(v) => {
                    struct_ser.serialize_field("floatValue", v)?;
                }
                enum_value::Value::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "stringValue",
            "intValue",
            "floatValue",
            "boolValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            StringValue,
            IntValue,
            FloatValue,
            BoolValue,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "stringValue" => Ok(GeneratedField::StringValue),
                            "intValue" => Ok(GeneratedField::IntValue),
                            "floatValue" => Ok(GeneratedField::FloatValue),
                            "boolValue" => Ok(GeneratedField::BoolValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.EnumValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnumValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(enum_value::Value::StringValue);
                        }
                        GeneratedField::IntValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| enum_value::Value::IntValue(x.0));
                        }
                        GeneratedField::FloatValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| enum_value::Value::FloatValue(x.0));
                        }
                        GeneratedField::BoolValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(enum_value::Value::BoolValue);
                        }
                    }
                }
                Ok(EnumValue {
                    key: key__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.EnumValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Extension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if !self.enum_info.is_empty() {
            len += 1;
        }
        if !self.conditions.is_empty() {
            len += 1;
        }
        if self.required {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.Extension", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if self.r#type != 0 {
            let v = ExtensionType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.enum_info.is_empty() {
            struct_ser.serialize_field("enumInfo", &self.enum_info)?;
        }
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        if self.required {
            struct_ser.serialize_field("required", &self.required)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Extension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "label",
            "type",
            "enumInfo",
            "conditions",
            "required",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Label,
            Type,
            EnumInfo,
            Conditions,
            Required,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "field" => Ok(GeneratedField::Field),
                            "label" => Ok(GeneratedField::Label),
                            "type" => Ok(GeneratedField::Type),
                            "enumInfo" => Ok(GeneratedField::EnumInfo),
                            "conditions" => Ok(GeneratedField::Conditions),
                            "required" => Ok(GeneratedField::Required),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Extension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.Extension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Extension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut label__ = None;
                let mut r#type__ = None;
                let mut enum_info__ = None;
                let mut conditions__ = None;
                let mut required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<ExtensionType>()? as i32);
                        }
                        GeneratedField::EnumInfo => {
                            if enum_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumInfo"));
                            }
                            enum_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Extension {
                    field: field__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    enum_info: enum_info__.unwrap_or_default(),
                    conditions: conditions__.unwrap_or_default(),
                    required: required__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.Extension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtensionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
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
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExtensionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STRING",
            "INT",
            "FLOAT",
            "BOOL",
            "STRING_ARRAY",
            "INTEGER_ARRAY",
            "FLOAT_ARRAY",
            "BOOLEAN_ARRAY",
            "ENUM",
            "ENUM_ARRAY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtensionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STRING" => Ok(ExtensionType::String),
                    "INT" => Ok(ExtensionType::Int),
                    "FLOAT" => Ok(ExtensionType::Float),
                    "BOOL" => Ok(ExtensionType::Bool),
                    "STRING_ARRAY" => Ok(ExtensionType::StringArray),
                    "INTEGER_ARRAY" => Ok(ExtensionType::IntegerArray),
                    "FLOAT_ARRAY" => Ok(ExtensionType::FloatArray),
                    "BOOLEAN_ARRAY" => Ok(ExtensionType::BooleanArray),
                    "ENUM" => Ok(ExtensionType::Enum),
                    "ENUM_ARRAY" => Ok(ExtensionType::EnumArray),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FloatArray {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.FloatArray", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatArray {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatArray;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.FloatArray")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FloatArray, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(FloatArray {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.FloatArray", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HousekeeperTaskType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::HousekeeperTaskUnset => "HOUSEKEEPER_TASK_UNSET",
            Self::EntityCleanup => "ENTITY_CLEANUP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HousekeeperTaskType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HOUSEKEEPER_TASK_UNSET",
            "ENTITY_CLEANUP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HousekeeperTaskType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HOUSEKEEPER_TASK_UNSET" => Ok(HousekeeperTaskType::HousekeeperTaskUnset),
                    "ENTITY_CLEANUP" => Ok(HousekeeperTaskType::EntityCleanup),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IntArray {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.IntArray", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntArray {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntArray;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.IntArray")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntArray, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(IntArray {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.IntArray", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Operator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
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
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Operator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EQ",
            "NEQ",
            "GT",
            "GTE",
            "LT",
            "LTE",
            "CONTAINS",
            "PREFIX",
            "SUFFIX",
            "REGEX",
            "IN",
            "NOT_IN",
            "BETWEEN",
            "NOT_BETWEEN",
            "NOT_NULL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Operator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EQ" => Ok(Operator::Eq),
                    "NEQ" => Ok(Operator::Neq),
                    "GT" => Ok(Operator::Gt),
                    "GTE" => Ok(Operator::Gte),
                    "LT" => Ok(Operator::Lt),
                    "LTE" => Ok(Operator::Lte),
                    "CONTAINS" => Ok(Operator::Contains),
                    "PREFIX" => Ok(Operator::Prefix),
                    "SUFFIX" => Ok(Operator::Suffix),
                    "REGEX" => Ok(Operator::Regex),
                    "IN" => Ok(Operator::In),
                    "NOT_IN" => Ok(Operator::NotIn),
                    "BETWEEN" => Ok(Operator::Between),
                    "NOT_BETWEEN" => Ok(Operator::NotBetween),
                    "NOT_NULL" => Ok(Operator::NotNull),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ProductInfoProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ng_proto.ProductInfoProto", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProductInfoProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProductInfoProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.ProductInfoProto")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProductInfoProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ProductInfoProto {
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.ProductInfoProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueueMsg {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ts != 0 {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.payload.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.QueueMsg", len)?;
        if self.ts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("ts", ToString::to_string(&self.ts).as_str())?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payload", pbjson::private::base64::encode(&self.payload).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueueMsg {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ts",
            "headers",
            "key",
            "payload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ts,
            Headers,
            Key,
            Payload,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ts" => Ok(GeneratedField::Ts),
                            "headers" => Ok(GeneratedField::Headers),
                            "key" => Ok(GeneratedField::Key),
                            "payload" => Ok(GeneratedField::Payload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueueMsg;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.QueueMsg")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueueMsg, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ts__ = None;
                let mut headers__ = None;
                let mut key__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ts => {
                            if ts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ts"));
                            }
                            ts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueueMsg {
                    ts: ts__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    payload: payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.QueueMsg", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SessionEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unset => "SESSION_EVENT_UNSET",
            Self::Open => "OPEN",
            Self::Closed => "CLOSED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SessionEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SESSION_EVENT_UNSET",
            "OPEN",
            "CLOSED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SessionEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SESSION_EVENT_UNSET" => Ok(SessionEvent::Unset),
                    "OPEN" => Ok(SessionEvent::Open),
                    "CLOSED" => Ok(SessionEvent::Closed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SessionInfoProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transport != 0 {
            len += 1;
        }
        if !self.node_id.is_empty() {
            len += 1;
        }
        if !self.session_id.is_empty() {
            len += 1;
        }
        if self.tenant_id != 0 {
            len += 1;
        }
        if self.device_id != 0 {
            len += 1;
        }
        if self.product_id != 0 {
            len += 1;
        }
        if !self.device_name.is_empty() {
            len += 1;
        }
        if !self.device_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.SessionInfoProto", len)?;
        if self.transport != 0 {
            let v = TransportType::try_from(self.transport)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.transport)))?;
            struct_ser.serialize_field("transport", &v)?;
        }
        if !self.node_id.is_empty() {
            struct_ser.serialize_field("nodeId", &self.node_id)?;
        }
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if self.tenant_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tenantId", ToString::to_string(&self.tenant_id).as_str())?;
        }
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if self.product_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("productId", ToString::to_string(&self.product_id).as_str())?;
        }
        if !self.device_name.is_empty() {
            struct_ser.serialize_field("deviceName", &self.device_name)?;
        }
        if !self.device_type.is_empty() {
            struct_ser.serialize_field("deviceType", &self.device_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SessionInfoProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transport",
            "nodeId",
            "sessionId",
            "tenantId",
            "deviceId",
            "productId",
            "deviceName",
            "deviceType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transport,
            NodeId,
            SessionId,
            TenantId,
            DeviceId,
            ProductId,
            DeviceName,
            DeviceType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transport" => Ok(GeneratedField::Transport),
                            "nodeId" => Ok(GeneratedField::NodeId),
                            "sessionId" => Ok(GeneratedField::SessionId),
                            "tenantId" => Ok(GeneratedField::TenantId),
                            "deviceId" => Ok(GeneratedField::DeviceId),
                            "productId" => Ok(GeneratedField::ProductId),
                            "deviceName" => Ok(GeneratedField::DeviceName),
                            "deviceType" => Ok(GeneratedField::DeviceType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SessionInfoProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.SessionInfoProto")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SessionInfoProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transport__ = None;
                let mut node_id__ = None;
                let mut session_id__ = None;
                let mut tenant_id__ = None;
                let mut device_id__ = None;
                let mut product_id__ = None;
                let mut device_name__ = None;
                let mut device_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transport => {
                            if transport__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            transport__ = Some(map_.next_value::<TransportType>()? as i32);
                        }
                        GeneratedField::NodeId => {
                            if node_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeId"));
                            }
                            node_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TenantId => {
                            if tenant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenantId"));
                            }
                            tenant_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProductId => {
                            if product_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productId"));
                            }
                            product_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceName => {
                            if device_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceName"));
                            }
                            device_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeviceType => {
                            if device_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceType"));
                            }
                            device_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SessionInfoProto {
                    transport: transport__.unwrap_or_default(),
                    node_id: node_id__.unwrap_or_default(),
                    session_id: session_id__.unwrap_or_default(),
                    tenant_id: tenant_id__.unwrap_or_default(),
                    device_id: device_id__.unwrap_or_default(),
                    product_id: product_id__.unwrap_or_default(),
                    device_name: device_name__.unwrap_or_default(),
                    device_type: device_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.SessionInfoProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SessionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::SessionUnset => "SESSION_UNSET",
            Self::Sync => "SYNC",
            Self::Async => "ASYNC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SessionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SESSION_UNSET",
            "SYNC",
            "ASYNC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SessionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SESSION_UNSET" => Ok(SessionType::SessionUnset),
                    "SYNC" => Ok(SessionType::Sync),
                    "ASYNC" => Ok(SessionType::Async),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StringArray {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ng_proto.StringArray", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringArray {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StringArray;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ng_proto.StringArray")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringArray, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StringArray {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ng_proto.StringArray", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransportType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Tcp => "TCP",
            Self::Mqtt => "MQTT",
            Self::Http => "HTTP",
            Self::Coap => "COAP",
            Self::Lwm2m => "LWM2M",
            Self::Snmp => "SNMP",
            Self::Ws => "WS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TransportType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TCP",
            "MQTT",
            "HTTP",
            "COAP",
            "LWM2M",
            "SNMP",
            "WS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransportType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TCP" => Ok(TransportType::Tcp),
                    "MQTT" => Ok(TransportType::Mqtt),
                    "HTTP" => Ok(TransportType::Http),
                    "COAP" => Ok(TransportType::Coap),
                    "LWM2M" => Ok(TransportType::Lwm2m),
                    "SNMP" => Ok(TransportType::Snmp),
                    "WS" => Ok(TransportType::Ws),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
