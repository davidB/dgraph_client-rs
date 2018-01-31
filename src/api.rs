// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub query: ::std::string::String,
    pub vars: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub start_ts: u64,
    pub lin_read: ::protobuf::SingularPtrField<LinRead>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // string query = 1;

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: ::std::string::String) {
        self.query = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut ::std::string::String {
        &mut self.query
    }

    // Take field
    pub fn take_query(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.query, ::std::string::String::new())
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    fn get_query_for_reflect(&self) -> &::std::string::String {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.query
    }

    // repeated .api.Request.VarsEntry vars = 2;

    pub fn clear_vars(&mut self) {
        self.vars.clear();
    }

    // Param is passed by value, moved
    pub fn set_vars(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.vars = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vars(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.vars
    }

    // Take field
    pub fn take_vars(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.vars, ::std::collections::HashMap::new())
    }

    pub fn get_vars(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.vars
    }

    fn get_vars_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.vars
    }

    fn mut_vars_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.vars
    }

    // uint64 start_ts = 13;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }

    fn get_start_ts_for_reflect(&self) -> &u64 {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_ts
    }

    // .api.LinRead lin_read = 14;

    pub fn clear_lin_read(&mut self) {
        self.lin_read.clear();
    }

    pub fn has_lin_read(&self) -> bool {
        self.lin_read.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lin_read(&mut self, v: LinRead) {
        self.lin_read = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lin_read(&mut self) -> &mut LinRead {
        if self.lin_read.is_none() {
            self.lin_read.set_default();
        }
        self.lin_read.as_mut().unwrap()
    }

    // Take field
    pub fn take_lin_read(&mut self) -> LinRead {
        self.lin_read.take().unwrap_or_else(|| LinRead::new())
    }

    pub fn get_lin_read(&self) -> &LinRead {
        self.lin_read.as_ref().unwrap_or_else(|| LinRead::default_instance())
    }

    fn get_lin_read_for_reflect(&self) -> &::protobuf::SingularPtrField<LinRead> {
        &self.lin_read
    }

    fn mut_lin_read_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LinRead> {
        &mut self.lin_read
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        for v in &self.lin_read {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.query)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.vars)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_ts = tmp;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lin_read)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.query.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.query);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.vars);
        if self.start_ts != 0 {
            my_size += ::protobuf::rt::value_size(13, self.start_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.lin_read.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.query.is_empty() {
            os.write_string(1, &self.query)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.vars, os)?;
        if self.start_ts != 0 {
            os.write_uint64(13, self.start_ts)?;
        }
        if let Some(ref v) = self.lin_read.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "query",
                    Request::get_query_for_reflect,
                    Request::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "vars",
                    Request::get_vars_for_reflect,
                    Request::mut_vars_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    Request::get_start_ts_for_reflect,
                    Request::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LinRead>>(
                    "lin_read",
                    Request::get_lin_read_for_reflect,
                    Request::mut_lin_read_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_vars();
        self.clear_start_ts();
        self.clear_lin_read();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub json: ::std::vec::Vec<u8>,
    pub schema: ::protobuf::RepeatedField<SchemaNode>,
    pub txn: ::protobuf::SingularPtrField<TxnContext>,
    pub latency: ::protobuf::SingularPtrField<Latency>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // bytes json = 1;

    pub fn clear_json(&mut self) {
        self.json.clear();
    }

    // Param is passed by value, moved
    pub fn set_json(&mut self, v: ::std::vec::Vec<u8>) {
        self.json = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_json(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.json
    }

    // Take field
    pub fn take_json(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.json, ::std::vec::Vec::new())
    }

    pub fn get_json(&self) -> &[u8] {
        &self.json
    }

    fn get_json_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.json
    }

    fn mut_json_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.json
    }

    // repeated .api.SchemaNode schema = 2;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: ::protobuf::RepeatedField<SchemaNode>) {
        self.schema = v;
    }

    // Mutable pointer to the field.
    pub fn mut_schema(&mut self) -> &mut ::protobuf::RepeatedField<SchemaNode> {
        &mut self.schema
    }

    // Take field
    pub fn take_schema(&mut self) -> ::protobuf::RepeatedField<SchemaNode> {
        ::std::mem::replace(&mut self.schema, ::protobuf::RepeatedField::new())
    }

    pub fn get_schema(&self) -> &[SchemaNode] {
        &self.schema
    }

    fn get_schema_for_reflect(&self) -> &::protobuf::RepeatedField<SchemaNode> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SchemaNode> {
        &mut self.schema
    }

    // .api.TxnContext txn = 3;

    pub fn clear_txn(&mut self) {
        self.txn.clear();
    }

    pub fn has_txn(&self) -> bool {
        self.txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn(&mut self, v: TxnContext) {
        self.txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txn(&mut self) -> &mut TxnContext {
        if self.txn.is_none() {
            self.txn.set_default();
        }
        self.txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_txn(&mut self) -> TxnContext {
        self.txn.take().unwrap_or_else(|| TxnContext::new())
    }

    pub fn get_txn(&self) -> &TxnContext {
        self.txn.as_ref().unwrap_or_else(|| TxnContext::default_instance())
    }

    fn get_txn_for_reflect(&self) -> &::protobuf::SingularPtrField<TxnContext> {
        &self.txn
    }

    fn mut_txn_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TxnContext> {
        &mut self.txn
    }

    // .api.Latency latency = 12;

    pub fn clear_latency(&mut self) {
        self.latency.clear();
    }

    pub fn has_latency(&self) -> bool {
        self.latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latency(&mut self, v: Latency) {
        self.latency = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_latency(&mut self) -> &mut Latency {
        if self.latency.is_none() {
            self.latency.set_default();
        }
        self.latency.as_mut().unwrap()
    }

    // Take field
    pub fn take_latency(&mut self) -> Latency {
        self.latency.take().unwrap_or_else(|| Latency::new())
    }

    pub fn get_latency(&self) -> &Latency {
        self.latency.as_ref().unwrap_or_else(|| Latency::default_instance())
    }

    fn get_latency_for_reflect(&self) -> &::protobuf::SingularPtrField<Latency> {
        &self.latency
    }

    fn mut_latency_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Latency> {
        &mut self.latency
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        for v in &self.schema {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.txn {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.latency {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.json)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.schema)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.txn)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.latency)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.json.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.json);
        }
        for value in &self.schema {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.txn.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.latency.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.json.is_empty() {
            os.write_bytes(1, &self.json)?;
        }
        for v in &self.schema {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.txn.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.latency.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "json",
                    Response::get_json_for_reflect,
                    Response::mut_json_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SchemaNode>>(
                    "schema",
                    Response::get_schema_for_reflect,
                    Response::mut_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TxnContext>>(
                    "txn",
                    Response::get_txn_for_reflect,
                    Response::mut_txn_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Latency>>(
                    "latency",
                    Response::get_latency_for_reflect,
                    Response::mut_latency_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_json();
        self.clear_schema();
        self.clear_txn();
        self.clear_latency();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Assigned {
    // message fields
    pub uids: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub context: ::protobuf::SingularPtrField<TxnContext>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Assigned {}

impl Assigned {
    pub fn new() -> Assigned {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Assigned {
        static mut instance: ::protobuf::lazy::Lazy<Assigned> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Assigned,
        };
        unsafe {
            instance.get(Assigned::new)
        }
    }

    // repeated .api.Assigned.UidsEntry uids = 1;

    pub fn clear_uids(&mut self) {
        self.uids.clear();
    }

    // Param is passed by value, moved
    pub fn set_uids(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.uids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uids(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.uids
    }

    // Take field
    pub fn take_uids(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.uids, ::std::collections::HashMap::new())
    }

    pub fn get_uids(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.uids
    }

    fn get_uids_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.uids
    }

    fn mut_uids_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.uids
    }

    // .api.TxnContext context = 2;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: TxnContext) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut TxnContext {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> TxnContext {
        self.context.take().unwrap_or_else(|| TxnContext::new())
    }

    pub fn get_context(&self) -> &TxnContext {
        self.context.as_ref().unwrap_or_else(|| TxnContext::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<TxnContext> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TxnContext> {
        &mut self.context
    }
}

impl ::protobuf::Message for Assigned {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.uids)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(1, &self.uids);
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(1, &self.uids, os)?;
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Assigned {
    fn new() -> Assigned {
        Assigned::new()
    }

    fn descriptor_static(_: ::std::option::Option<Assigned>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "uids",
                    Assigned::get_uids_for_reflect,
                    Assigned::mut_uids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TxnContext>>(
                    "context",
                    Assigned::get_context_for_reflect,
                    Assigned::mut_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Assigned>(
                    "Assigned",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Assigned {
    fn clear(&mut self) {
        self.clear_uids();
        self.clear_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Assigned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Assigned {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mutation {
    // message fields
    pub set_json: ::std::vec::Vec<u8>,
    pub delete_json: ::std::vec::Vec<u8>,
    pub set_nquads: ::std::vec::Vec<u8>,
    pub del_nquads: ::std::vec::Vec<u8>,
    pub set: ::protobuf::RepeatedField<NQuad>,
    pub del: ::protobuf::RepeatedField<NQuad>,
    pub start_ts: u64,
    pub commit_now: bool,
    pub ignore_index_conflict: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mutation {}

impl Mutation {
    pub fn new() -> Mutation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mutation {
        static mut instance: ::protobuf::lazy::Lazy<Mutation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mutation,
        };
        unsafe {
            instance.get(Mutation::new)
        }
    }

    // bytes set_json = 1;

    pub fn clear_set_json(&mut self) {
        self.set_json.clear();
    }

    // Param is passed by value, moved
    pub fn set_set_json(&mut self, v: ::std::vec::Vec<u8>) {
        self.set_json = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_json(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.set_json
    }

    // Take field
    pub fn take_set_json(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.set_json, ::std::vec::Vec::new())
    }

    pub fn get_set_json(&self) -> &[u8] {
        &self.set_json
    }

    fn get_set_json_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.set_json
    }

    fn mut_set_json_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.set_json
    }

    // bytes delete_json = 2;

    pub fn clear_delete_json(&mut self) {
        self.delete_json.clear();
    }

    // Param is passed by value, moved
    pub fn set_delete_json(&mut self, v: ::std::vec::Vec<u8>) {
        self.delete_json = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_json(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.delete_json
    }

    // Take field
    pub fn take_delete_json(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.delete_json, ::std::vec::Vec::new())
    }

    pub fn get_delete_json(&self) -> &[u8] {
        &self.delete_json
    }

    fn get_delete_json_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.delete_json
    }

    fn mut_delete_json_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.delete_json
    }

    // bytes set_nquads = 3;

    pub fn clear_set_nquads(&mut self) {
        self.set_nquads.clear();
    }

    // Param is passed by value, moved
    pub fn set_set_nquads(&mut self, v: ::std::vec::Vec<u8>) {
        self.set_nquads = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_nquads(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.set_nquads
    }

    // Take field
    pub fn take_set_nquads(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.set_nquads, ::std::vec::Vec::new())
    }

    pub fn get_set_nquads(&self) -> &[u8] {
        &self.set_nquads
    }

    fn get_set_nquads_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.set_nquads
    }

    fn mut_set_nquads_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.set_nquads
    }

    // bytes del_nquads = 4;

    pub fn clear_del_nquads(&mut self) {
        self.del_nquads.clear();
    }

    // Param is passed by value, moved
    pub fn set_del_nquads(&mut self, v: ::std::vec::Vec<u8>) {
        self.del_nquads = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_del_nquads(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.del_nquads
    }

    // Take field
    pub fn take_del_nquads(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.del_nquads, ::std::vec::Vec::new())
    }

    pub fn get_del_nquads(&self) -> &[u8] {
        &self.del_nquads
    }

    fn get_del_nquads_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.del_nquads
    }

    fn mut_del_nquads_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.del_nquads
    }

    // repeated .api.NQuad set = 10;

    pub fn clear_set(&mut self) {
        self.set.clear();
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: ::protobuf::RepeatedField<NQuad>) {
        self.set = v;
    }

    // Mutable pointer to the field.
    pub fn mut_set(&mut self) -> &mut ::protobuf::RepeatedField<NQuad> {
        &mut self.set
    }

    // Take field
    pub fn take_set(&mut self) -> ::protobuf::RepeatedField<NQuad> {
        ::std::mem::replace(&mut self.set, ::protobuf::RepeatedField::new())
    }

    pub fn get_set(&self) -> &[NQuad] {
        &self.set
    }

    fn get_set_for_reflect(&self) -> &::protobuf::RepeatedField<NQuad> {
        &self.set
    }

    fn mut_set_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NQuad> {
        &mut self.set
    }

    // repeated .api.NQuad del = 11;

    pub fn clear_del(&mut self) {
        self.del.clear();
    }

    // Param is passed by value, moved
    pub fn set_del(&mut self, v: ::protobuf::RepeatedField<NQuad>) {
        self.del = v;
    }

    // Mutable pointer to the field.
    pub fn mut_del(&mut self) -> &mut ::protobuf::RepeatedField<NQuad> {
        &mut self.del
    }

    // Take field
    pub fn take_del(&mut self) -> ::protobuf::RepeatedField<NQuad> {
        ::std::mem::replace(&mut self.del, ::protobuf::RepeatedField::new())
    }

    pub fn get_del(&self) -> &[NQuad] {
        &self.del
    }

    fn get_del_for_reflect(&self) -> &::protobuf::RepeatedField<NQuad> {
        &self.del
    }

    fn mut_del_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NQuad> {
        &mut self.del
    }

    // uint64 start_ts = 13;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }

    fn get_start_ts_for_reflect(&self) -> &u64 {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_ts
    }

    // bool commit_now = 14;

    pub fn clear_commit_now(&mut self) {
        self.commit_now = false;
    }

    // Param is passed by value, moved
    pub fn set_commit_now(&mut self, v: bool) {
        self.commit_now = v;
    }

    pub fn get_commit_now(&self) -> bool {
        self.commit_now
    }

    fn get_commit_now_for_reflect(&self) -> &bool {
        &self.commit_now
    }

    fn mut_commit_now_for_reflect(&mut self) -> &mut bool {
        &mut self.commit_now
    }

    // bool ignore_index_conflict = 15;

    pub fn clear_ignore_index_conflict(&mut self) {
        self.ignore_index_conflict = false;
    }

    // Param is passed by value, moved
    pub fn set_ignore_index_conflict(&mut self, v: bool) {
        self.ignore_index_conflict = v;
    }

    pub fn get_ignore_index_conflict(&self) -> bool {
        self.ignore_index_conflict
    }

    fn get_ignore_index_conflict_for_reflect(&self) -> &bool {
        &self.ignore_index_conflict
    }

    fn mut_ignore_index_conflict_for_reflect(&mut self) -> &mut bool {
        &mut self.ignore_index_conflict
    }
}

impl ::protobuf::Message for Mutation {
    fn is_initialized(&self) -> bool {
        for v in &self.set {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.del {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.set_json)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.delete_json)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.set_nquads)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.del_nquads)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.set)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.del)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_ts = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.commit_now = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignore_index_conflict = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.set_json.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.set_json);
        }
        if !self.delete_json.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.delete_json);
        }
        if !self.set_nquads.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.set_nquads);
        }
        if !self.del_nquads.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.del_nquads);
        }
        for value in &self.set {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.del {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.start_ts != 0 {
            my_size += ::protobuf::rt::value_size(13, self.start_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commit_now != false {
            my_size += 2;
        }
        if self.ignore_index_conflict != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.set_json.is_empty() {
            os.write_bytes(1, &self.set_json)?;
        }
        if !self.delete_json.is_empty() {
            os.write_bytes(2, &self.delete_json)?;
        }
        if !self.set_nquads.is_empty() {
            os.write_bytes(3, &self.set_nquads)?;
        }
        if !self.del_nquads.is_empty() {
            os.write_bytes(4, &self.del_nquads)?;
        }
        for v in &self.set {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.del {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.start_ts != 0 {
            os.write_uint64(13, self.start_ts)?;
        }
        if self.commit_now != false {
            os.write_bool(14, self.commit_now)?;
        }
        if self.ignore_index_conflict != false {
            os.write_bool(15, self.ignore_index_conflict)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mutation {
    fn new() -> Mutation {
        Mutation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mutation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "set_json",
                    Mutation::get_set_json_for_reflect,
                    Mutation::mut_set_json_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "delete_json",
                    Mutation::get_delete_json_for_reflect,
                    Mutation::mut_delete_json_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "set_nquads",
                    Mutation::get_set_nquads_for_reflect,
                    Mutation::mut_set_nquads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "del_nquads",
                    Mutation::get_del_nquads_for_reflect,
                    Mutation::mut_del_nquads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NQuad>>(
                    "set",
                    Mutation::get_set_for_reflect,
                    Mutation::mut_set_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NQuad>>(
                    "del",
                    Mutation::get_del_for_reflect,
                    Mutation::mut_del_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    Mutation::get_start_ts_for_reflect,
                    Mutation::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "commit_now",
                    Mutation::get_commit_now_for_reflect,
                    Mutation::mut_commit_now_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore_index_conflict",
                    Mutation::get_ignore_index_conflict_for_reflect,
                    Mutation::mut_ignore_index_conflict_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mutation>(
                    "Mutation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        self.clear_set_json();
        self.clear_delete_json();
        self.clear_set_nquads();
        self.clear_del_nquads();
        self.clear_set();
        self.clear_del();
        self.clear_start_ts();
        self.clear_commit_now();
        self.clear_ignore_index_conflict();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mutation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AssignedIds {
    // message fields
    pub startId: u64,
    pub endId: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AssignedIds {}

impl AssignedIds {
    pub fn new() -> AssignedIds {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AssignedIds {
        static mut instance: ::protobuf::lazy::Lazy<AssignedIds> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AssignedIds,
        };
        unsafe {
            instance.get(AssignedIds::new)
        }
    }

    // uint64 startId = 1;

    pub fn clear_startId(&mut self) {
        self.startId = 0;
    }

    // Param is passed by value, moved
    pub fn set_startId(&mut self, v: u64) {
        self.startId = v;
    }

    pub fn get_startId(&self) -> u64 {
        self.startId
    }

    fn get_startId_for_reflect(&self) -> &u64 {
        &self.startId
    }

    fn mut_startId_for_reflect(&mut self) -> &mut u64 {
        &mut self.startId
    }

    // uint64 endId = 2;

    pub fn clear_endId(&mut self) {
        self.endId = 0;
    }

    // Param is passed by value, moved
    pub fn set_endId(&mut self, v: u64) {
        self.endId = v;
    }

    pub fn get_endId(&self) -> u64 {
        self.endId
    }

    fn get_endId_for_reflect(&self) -> &u64 {
        &self.endId
    }

    fn mut_endId_for_reflect(&mut self) -> &mut u64 {
        &mut self.endId
    }
}

impl ::protobuf::Message for AssignedIds {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.startId = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.endId = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.startId != 0 {
            my_size += ::protobuf::rt::value_size(1, self.startId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.endId != 0 {
            my_size += ::protobuf::rt::value_size(2, self.endId, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.startId != 0 {
            os.write_uint64(1, self.startId)?;
        }
        if self.endId != 0 {
            os.write_uint64(2, self.endId)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AssignedIds {
    fn new() -> AssignedIds {
        AssignedIds::new()
    }

    fn descriptor_static(_: ::std::option::Option<AssignedIds>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "startId",
                    AssignedIds::get_startId_for_reflect,
                    AssignedIds::mut_startId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "endId",
                    AssignedIds::get_endId_for_reflect,
                    AssignedIds::mut_endId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AssignedIds>(
                    "AssignedIds",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AssignedIds {
    fn clear(&mut self) {
        self.clear_startId();
        self.clear_endId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AssignedIds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AssignedIds {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Operation {
    // message fields
    pub schema: ::std::string::String,
    pub drop_attr: ::std::string::String,
    pub drop_all: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Operation {}

impl Operation {
    pub fn new() -> Operation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Operation {
        static mut instance: ::protobuf::lazy::Lazy<Operation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Operation,
        };
        unsafe {
            instance.get(Operation::new)
        }
    }

    // string schema = 1;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: ::std::string::String) {
        self.schema = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut ::std::string::String {
        &mut self.schema
    }

    // Take field
    pub fn take_schema(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.schema, ::std::string::String::new())
    }

    pub fn get_schema(&self) -> &str {
        &self.schema
    }

    fn get_schema_for_reflect(&self) -> &::std::string::String {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.schema
    }

    // string drop_attr = 2;

    pub fn clear_drop_attr(&mut self) {
        self.drop_attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_drop_attr(&mut self, v: ::std::string::String) {
        self.drop_attr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_drop_attr(&mut self) -> &mut ::std::string::String {
        &mut self.drop_attr
    }

    // Take field
    pub fn take_drop_attr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.drop_attr, ::std::string::String::new())
    }

    pub fn get_drop_attr(&self) -> &str {
        &self.drop_attr
    }

    fn get_drop_attr_for_reflect(&self) -> &::std::string::String {
        &self.drop_attr
    }

    fn mut_drop_attr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.drop_attr
    }

    // bool drop_all = 3;

    pub fn clear_drop_all(&mut self) {
        self.drop_all = false;
    }

    // Param is passed by value, moved
    pub fn set_drop_all(&mut self, v: bool) {
        self.drop_all = v;
    }

    pub fn get_drop_all(&self) -> bool {
        self.drop_all
    }

    fn get_drop_all_for_reflect(&self) -> &bool {
        &self.drop_all
    }

    fn mut_drop_all_for_reflect(&mut self) -> &mut bool {
        &mut self.drop_all
    }
}

impl ::protobuf::Message for Operation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.schema)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.drop_attr)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.drop_all = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.schema.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.schema);
        }
        if !self.drop_attr.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.drop_attr);
        }
        if self.drop_all != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.schema.is_empty() {
            os.write_string(1, &self.schema)?;
        }
        if !self.drop_attr.is_empty() {
            os.write_string(2, &self.drop_attr)?;
        }
        if self.drop_all != false {
            os.write_bool(3, self.drop_all)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Operation {
    fn new() -> Operation {
        Operation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Operation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "schema",
                    Operation::get_schema_for_reflect,
                    Operation::mut_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "drop_attr",
                    Operation::get_drop_attr_for_reflect,
                    Operation::mut_drop_attr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "drop_all",
                    Operation::get_drop_all_for_reflect,
                    Operation::mut_drop_all_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Operation>(
                    "Operation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Operation {
    fn clear(&mut self) {
        self.clear_schema();
        self.clear_drop_attr();
        self.clear_drop_all();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Operation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Payload {
    // message fields
    pub Data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Payload {}

impl Payload {
    pub fn new() -> Payload {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Payload {
        static mut instance: ::protobuf::lazy::Lazy<Payload> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Payload,
        };
        unsafe {
            instance.get(Payload::new)
        }
    }

    // bytes Data = 1;

    pub fn clear_Data(&mut self) {
        self.Data.clear();
    }

    // Param is passed by value, moved
    pub fn set_Data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.Data
    }

    // Take field
    pub fn take_Data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.Data, ::std::vec::Vec::new())
    }

    pub fn get_Data(&self) -> &[u8] {
        &self.Data
    }

    fn get_Data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.Data
    }

    fn mut_Data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.Data
    }
}

impl ::protobuf::Message for Payload {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.Data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.Data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.Data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.Data.is_empty() {
            os.write_bytes(1, &self.Data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Payload {
    fn new() -> Payload {
        Payload::new()
    }

    fn descriptor_static(_: ::std::option::Option<Payload>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "Data",
                    Payload::get_Data_for_reflect,
                    Payload::mut_Data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Payload>(
                    "Payload",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Payload {
    fn clear(&mut self) {
        self.clear_Data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Payload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Payload {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TxnContext {
    // message fields
    pub start_ts: u64,
    pub commit_ts: u64,
    pub aborted: bool,
    pub keys: ::protobuf::RepeatedField<::std::string::String>,
    pub lin_read: ::protobuf::SingularPtrField<LinRead>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TxnContext {}

impl TxnContext {
    pub fn new() -> TxnContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TxnContext {
        static mut instance: ::protobuf::lazy::Lazy<TxnContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TxnContext,
        };
        unsafe {
            instance.get(TxnContext::new)
        }
    }

    // uint64 start_ts = 1;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }

    fn get_start_ts_for_reflect(&self) -> &u64 {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_ts
    }

    // uint64 commit_ts = 2;

    pub fn clear_commit_ts(&mut self) {
        self.commit_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_ts(&mut self, v: u64) {
        self.commit_ts = v;
    }

    pub fn get_commit_ts(&self) -> u64 {
        self.commit_ts
    }

    fn get_commit_ts_for_reflect(&self) -> &u64 {
        &self.commit_ts
    }

    fn mut_commit_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.commit_ts
    }

    // bool aborted = 3;

    pub fn clear_aborted(&mut self) {
        self.aborted = false;
    }

    // Param is passed by value, moved
    pub fn set_aborted(&mut self, v: bool) {
        self.aborted = v;
    }

    pub fn get_aborted(&self) -> bool {
        self.aborted
    }

    fn get_aborted_for_reflect(&self) -> &bool {
        &self.aborted
    }

    fn mut_aborted_for_reflect(&mut self) -> &mut bool {
        &mut self.aborted
    }

    // repeated string keys = 4;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::string::String] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }

    // .api.LinRead lin_read = 13;

    pub fn clear_lin_read(&mut self) {
        self.lin_read.clear();
    }

    pub fn has_lin_read(&self) -> bool {
        self.lin_read.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lin_read(&mut self, v: LinRead) {
        self.lin_read = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lin_read(&mut self) -> &mut LinRead {
        if self.lin_read.is_none() {
            self.lin_read.set_default();
        }
        self.lin_read.as_mut().unwrap()
    }

    // Take field
    pub fn take_lin_read(&mut self) -> LinRead {
        self.lin_read.take().unwrap_or_else(|| LinRead::new())
    }

    pub fn get_lin_read(&self) -> &LinRead {
        self.lin_read.as_ref().unwrap_or_else(|| LinRead::default_instance())
    }

    fn get_lin_read_for_reflect(&self) -> &::protobuf::SingularPtrField<LinRead> {
        &self.lin_read
    }

    fn mut_lin_read_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LinRead> {
        &mut self.lin_read
    }
}

impl ::protobuf::Message for TxnContext {
    fn is_initialized(&self) -> bool {
        for v in &self.lin_read {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_ts = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit_ts = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.aborted = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.keys)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lin_read)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.start_ts != 0 {
            my_size += ::protobuf::rt::value_size(1, self.start_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commit_ts != 0 {
            my_size += ::protobuf::rt::value_size(2, self.commit_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.aborted != false {
            my_size += 2;
        }
        for value in &self.keys {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(ref v) = self.lin_read.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.start_ts != 0 {
            os.write_uint64(1, self.start_ts)?;
        }
        if self.commit_ts != 0 {
            os.write_uint64(2, self.commit_ts)?;
        }
        if self.aborted != false {
            os.write_bool(3, self.aborted)?;
        }
        for v in &self.keys {
            os.write_string(4, &v)?;
        };
        if let Some(ref v) = self.lin_read.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TxnContext {
    fn new() -> TxnContext {
        TxnContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<TxnContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    TxnContext::get_start_ts_for_reflect,
                    TxnContext::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_ts",
                    TxnContext::get_commit_ts_for_reflect,
                    TxnContext::mut_commit_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "aborted",
                    TxnContext::get_aborted_for_reflect,
                    TxnContext::mut_aborted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keys",
                    TxnContext::get_keys_for_reflect,
                    TxnContext::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LinRead>>(
                    "lin_read",
                    TxnContext::get_lin_read_for_reflect,
                    TxnContext::mut_lin_read_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TxnContext>(
                    "TxnContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TxnContext {
    fn clear(&mut self) {
        self.clear_start_ts();
        self.clear_commit_ts();
        self.clear_aborted();
        self.clear_keys();
        self.clear_lin_read();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TxnContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TxnContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Check {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Check {}

impl Check {
    pub fn new() -> Check {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Check {
        static mut instance: ::protobuf::lazy::Lazy<Check> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Check,
        };
        unsafe {
            instance.get(Check::new)
        }
    }
}

impl ::protobuf::Message for Check {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Check {
    fn new() -> Check {
        Check::new()
    }

    fn descriptor_static(_: ::std::option::Option<Check>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Check>(
                    "Check",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Check {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Check {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Check {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Version {
    // message fields
    pub tag: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Version {}

impl Version {
    pub fn new() -> Version {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Version {
        static mut instance: ::protobuf::lazy::Lazy<Version> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Version,
        };
        unsafe {
            instance.get(Version::new)
        }
    }

    // string tag = 1;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::string::String) {
        self.tag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::string::String {
        &mut self.tag
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tag, ::std::string::String::new())
    }

    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    fn get_tag_for_reflect(&self) -> &::std::string::String {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tag
    }
}

impl ::protobuf::Message for Version {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tag)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.tag.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tag);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tag.is_empty() {
            os.write_string(1, &self.tag)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Version {
    fn new() -> Version {
        Version::new()
    }

    fn descriptor_static(_: ::std::option::Option<Version>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    Version::get_tag_for_reflect,
                    Version::mut_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Version>(
                    "Version",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Version {
    fn clear(&mut self) {
        self.clear_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Version {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Version {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LinRead {
    // message fields
    pub ids: ::std::collections::HashMap<u32, u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LinRead {}

impl LinRead {
    pub fn new() -> LinRead {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LinRead {
        static mut instance: ::protobuf::lazy::Lazy<LinRead> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LinRead,
        };
        unsafe {
            instance.get(LinRead::new)
        }
    }

    // repeated .api.LinRead.IdsEntry ids = 1;

    pub fn clear_ids(&mut self) {
        self.ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_ids(&mut self, v: ::std::collections::HashMap<u32, u64>) {
        self.ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ids(&mut self) -> &mut ::std::collections::HashMap<u32, u64> {
        &mut self.ids
    }

    // Take field
    pub fn take_ids(&mut self) -> ::std::collections::HashMap<u32, u64> {
        ::std::mem::replace(&mut self.ids, ::std::collections::HashMap::new())
    }

    pub fn get_ids(&self) -> &::std::collections::HashMap<u32, u64> {
        &self.ids
    }

    fn get_ids_for_reflect(&self) -> &::std::collections::HashMap<u32, u64> {
        &self.ids
    }

    fn mut_ids_for_reflect(&mut self) -> &mut ::std::collections::HashMap<u32, u64> {
        &mut self.ids
    }
}

impl ::protobuf::Message for LinRead {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeUint32, ::protobuf::types::ProtobufTypeUint64>(wire_type, is, &mut self.ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeUint32, ::protobuf::types::ProtobufTypeUint64>(1, &self.ids);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeUint32, ::protobuf::types::ProtobufTypeUint64>(1, &self.ids, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LinRead {
    fn new() -> LinRead {
        LinRead::new()
    }

    fn descriptor_static(_: ::std::option::Option<LinRead>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeUint32, ::protobuf::types::ProtobufTypeUint64>(
                    "ids",
                    LinRead::get_ids_for_reflect,
                    LinRead::mut_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LinRead>(
                    "LinRead",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LinRead {
    fn clear(&mut self) {
        self.clear_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LinRead {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LinRead {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Latency {
    // message fields
    pub parsing_ns: u64,
    pub processing_ns: u64,
    pub encoding_ns: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Latency {}

impl Latency {
    pub fn new() -> Latency {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Latency {
        static mut instance: ::protobuf::lazy::Lazy<Latency> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Latency,
        };
        unsafe {
            instance.get(Latency::new)
        }
    }

    // uint64 parsing_ns = 1;

    pub fn clear_parsing_ns(&mut self) {
        self.parsing_ns = 0;
    }

    // Param is passed by value, moved
    pub fn set_parsing_ns(&mut self, v: u64) {
        self.parsing_ns = v;
    }

    pub fn get_parsing_ns(&self) -> u64 {
        self.parsing_ns
    }

    fn get_parsing_ns_for_reflect(&self) -> &u64 {
        &self.parsing_ns
    }

    fn mut_parsing_ns_for_reflect(&mut self) -> &mut u64 {
        &mut self.parsing_ns
    }

    // uint64 processing_ns = 2;

    pub fn clear_processing_ns(&mut self) {
        self.processing_ns = 0;
    }

    // Param is passed by value, moved
    pub fn set_processing_ns(&mut self, v: u64) {
        self.processing_ns = v;
    }

    pub fn get_processing_ns(&self) -> u64 {
        self.processing_ns
    }

    fn get_processing_ns_for_reflect(&self) -> &u64 {
        &self.processing_ns
    }

    fn mut_processing_ns_for_reflect(&mut self) -> &mut u64 {
        &mut self.processing_ns
    }

    // uint64 encoding_ns = 3;

    pub fn clear_encoding_ns(&mut self) {
        self.encoding_ns = 0;
    }

    // Param is passed by value, moved
    pub fn set_encoding_ns(&mut self, v: u64) {
        self.encoding_ns = v;
    }

    pub fn get_encoding_ns(&self) -> u64 {
        self.encoding_ns
    }

    fn get_encoding_ns_for_reflect(&self) -> &u64 {
        &self.encoding_ns
    }

    fn mut_encoding_ns_for_reflect(&mut self) -> &mut u64 {
        &mut self.encoding_ns
    }
}

impl ::protobuf::Message for Latency {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.parsing_ns = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.processing_ns = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.encoding_ns = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.parsing_ns != 0 {
            my_size += ::protobuf::rt::value_size(1, self.parsing_ns, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.processing_ns != 0 {
            my_size += ::protobuf::rt::value_size(2, self.processing_ns, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.encoding_ns != 0 {
            my_size += ::protobuf::rt::value_size(3, self.encoding_ns, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.parsing_ns != 0 {
            os.write_uint64(1, self.parsing_ns)?;
        }
        if self.processing_ns != 0 {
            os.write_uint64(2, self.processing_ns)?;
        }
        if self.encoding_ns != 0 {
            os.write_uint64(3, self.encoding_ns)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Latency {
    fn new() -> Latency {
        Latency::new()
    }

    fn descriptor_static(_: ::std::option::Option<Latency>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "parsing_ns",
                    Latency::get_parsing_ns_for_reflect,
                    Latency::mut_parsing_ns_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "processing_ns",
                    Latency::get_processing_ns_for_reflect,
                    Latency::mut_processing_ns_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "encoding_ns",
                    Latency::get_encoding_ns_for_reflect,
                    Latency::mut_encoding_ns_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Latency>(
                    "Latency",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Latency {
    fn clear(&mut self) {
        self.clear_parsing_ns();
        self.clear_processing_ns();
        self.clear_encoding_ns();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Latency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Latency {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NQuad {
    // message fields
    pub subject: ::std::string::String,
    pub predicate: ::std::string::String,
    pub object_id: ::std::string::String,
    pub object_value: ::protobuf::SingularPtrField<Value>,
    pub label: ::std::string::String,
    pub lang: ::std::string::String,
    pub facets: ::protobuf::RepeatedField<Facet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NQuad {}

impl NQuad {
    pub fn new() -> NQuad {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NQuad {
        static mut instance: ::protobuf::lazy::Lazy<NQuad> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NQuad,
        };
        unsafe {
            instance.get(NQuad::new)
        }
    }

    // string subject = 1;

    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    fn get_subject_for_reflect(&self) -> &::std::string::String {
        &self.subject
    }

    fn mut_subject_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // string predicate = 2;

    pub fn clear_predicate(&mut self) {
        self.predicate.clear();
    }

    // Param is passed by value, moved
    pub fn set_predicate(&mut self, v: ::std::string::String) {
        self.predicate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_predicate(&mut self) -> &mut ::std::string::String {
        &mut self.predicate
    }

    // Take field
    pub fn take_predicate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.predicate, ::std::string::String::new())
    }

    pub fn get_predicate(&self) -> &str {
        &self.predicate
    }

    fn get_predicate_for_reflect(&self) -> &::std::string::String {
        &self.predicate
    }

    fn mut_predicate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.predicate
    }

    // string object_id = 3;

    pub fn clear_object_id(&mut self) {
        self.object_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_object_id(&mut self, v: ::std::string::String) {
        self.object_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object_id(&mut self) -> &mut ::std::string::String {
        &mut self.object_id
    }

    // Take field
    pub fn take_object_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.object_id, ::std::string::String::new())
    }

    pub fn get_object_id(&self) -> &str {
        &self.object_id
    }

    fn get_object_id_for_reflect(&self) -> &::std::string::String {
        &self.object_id
    }

    fn mut_object_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.object_id
    }

    // .api.Value object_value = 4;

    pub fn clear_object_value(&mut self) {
        self.object_value.clear();
    }

    pub fn has_object_value(&self) -> bool {
        self.object_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object_value(&mut self, v: Value) {
        self.object_value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object_value(&mut self) -> &mut Value {
        if self.object_value.is_none() {
            self.object_value.set_default();
        }
        self.object_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_object_value(&mut self) -> Value {
        self.object_value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_object_value(&self) -> &Value {
        self.object_value.as_ref().unwrap_or_else(|| Value::default_instance())
    }

    fn get_object_value_for_reflect(&self) -> &::protobuf::SingularPtrField<Value> {
        &self.object_value
    }

    fn mut_object_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Value> {
        &mut self.object_value
    }

    // string label = 5;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::std::string::String) {
        self.label = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // Take field
    pub fn take_label(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.label, ::std::string::String::new())
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    fn get_label_for_reflect(&self) -> &::std::string::String {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // string lang = 6;

    pub fn clear_lang(&mut self) {
        self.lang.clear();
    }

    // Param is passed by value, moved
    pub fn set_lang(&mut self, v: ::std::string::String) {
        self.lang = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lang(&mut self) -> &mut ::std::string::String {
        &mut self.lang
    }

    // Take field
    pub fn take_lang(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.lang, ::std::string::String::new())
    }

    pub fn get_lang(&self) -> &str {
        &self.lang
    }

    fn get_lang_for_reflect(&self) -> &::std::string::String {
        &self.lang
    }

    fn mut_lang_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.lang
    }

    // repeated .api.Facet facets = 7;

    pub fn clear_facets(&mut self) {
        self.facets.clear();
    }

    // Param is passed by value, moved
    pub fn set_facets(&mut self, v: ::protobuf::RepeatedField<Facet>) {
        self.facets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_facets(&mut self) -> &mut ::protobuf::RepeatedField<Facet> {
        &mut self.facets
    }

    // Take field
    pub fn take_facets(&mut self) -> ::protobuf::RepeatedField<Facet> {
        ::std::mem::replace(&mut self.facets, ::protobuf::RepeatedField::new())
    }

    pub fn get_facets(&self) -> &[Facet] {
        &self.facets
    }

    fn get_facets_for_reflect(&self) -> &::protobuf::RepeatedField<Facet> {
        &self.facets
    }

    fn mut_facets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Facet> {
        &mut self.facets
    }
}

impl ::protobuf::Message for NQuad {
    fn is_initialized(&self) -> bool {
        for v in &self.object_value {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.facets {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.predicate)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.object_id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.object_value)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.label)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.lang)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.facets)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.subject);
        }
        if !self.predicate.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.predicate);
        }
        if !self.object_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.object_id);
        }
        if let Some(ref v) = self.object_value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.label.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.label);
        }
        if !self.lang.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.lang);
        }
        for value in &self.facets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.subject.is_empty() {
            os.write_string(1, &self.subject)?;
        }
        if !self.predicate.is_empty() {
            os.write_string(2, &self.predicate)?;
        }
        if !self.object_id.is_empty() {
            os.write_string(3, &self.object_id)?;
        }
        if let Some(ref v) = self.object_value.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.label.is_empty() {
            os.write_string(5, &self.label)?;
        }
        if !self.lang.is_empty() {
            os.write_string(6, &self.lang)?;
        }
        for v in &self.facets {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NQuad {
    fn new() -> NQuad {
        NQuad::new()
    }

    fn descriptor_static(_: ::std::option::Option<NQuad>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subject",
                    NQuad::get_subject_for_reflect,
                    NQuad::mut_subject_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "predicate",
                    NQuad::get_predicate_for_reflect,
                    NQuad::mut_predicate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "object_id",
                    NQuad::get_object_id_for_reflect,
                    NQuad::mut_object_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Value>>(
                    "object_value",
                    NQuad::get_object_value_for_reflect,
                    NQuad::mut_object_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "label",
                    NQuad::get_label_for_reflect,
                    NQuad::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lang",
                    NQuad::get_lang_for_reflect,
                    NQuad::mut_lang_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Facet>>(
                    "facets",
                    NQuad::get_facets_for_reflect,
                    NQuad::mut_facets_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NQuad>(
                    "NQuad",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NQuad {
    fn clear(&mut self) {
        self.clear_subject();
        self.clear_predicate();
        self.clear_object_id();
        self.clear_object_value();
        self.clear_label();
        self.clear_lang();
        self.clear_facets();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NQuad {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NQuad {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Value {
    // message oneof groups
    val: ::std::option::Option<Value_oneof_val>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Value {}

#[derive(Clone,PartialEq)]
pub enum Value_oneof_val {
    default_val(::std::string::String),
    bytes_val(::std::vec::Vec<u8>),
    int_val(i64),
    bool_val(bool),
    str_val(::std::string::String),
    double_val(f64),
    geo_val(::std::vec::Vec<u8>),
    date_val(::std::vec::Vec<u8>),
    datetime_val(::std::vec::Vec<u8>),
    password_val(::std::string::String),
    uid_val(u64),
}

impl Value {
    pub fn new() -> Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Value {
        static mut instance: ::protobuf::lazy::Lazy<Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value,
        };
        unsafe {
            instance.get(Value::new)
        }
    }

    // string default_val = 1;

    pub fn clear_default_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_default_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::default_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_default_val(&mut self, v: ::std::string::String) {
        self.val = ::std::option::Option::Some(Value_oneof_val::default_val(v))
    }

    // Mutable pointer to the field.
    pub fn mut_default_val(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Value_oneof_val::default_val(_)) = self.val {
        } else {
            self.val = ::std::option::Option::Some(Value_oneof_val::default_val(::std::string::String::new()));
        }
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::default_val(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_default_val(&mut self) -> ::std::string::String {
        if self.has_default_val() {
            match self.val.take() {
                ::std::option::Option::Some(Value_oneof_val::default_val(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_default_val(&self) -> &str {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::default_val(ref v)) => v,
            _ => "",
        }
    }

    // bytes bytes_val = 2;

    pub fn clear_bytes_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_bytes_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::bytes_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytes_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::std::option::Option::Some(Value_oneof_val::bytes_val(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bytes_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Value_oneof_val::bytes_val(_)) = self.val {
        } else {
            self.val = ::std::option::Option::Some(Value_oneof_val::bytes_val(::std::vec::Vec::new()));
        }
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::bytes_val(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytes_val(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_bytes_val() {
            match self.val.take() {
                ::std::option::Option::Some(Value_oneof_val::bytes_val(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_bytes_val(&self) -> &[u8] {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::bytes_val(ref v)) => v,
            _ => &[],
        }
    }

    // int64 int_val = 3;

    pub fn clear_int_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_int_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::int_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int_val(&mut self, v: i64) {
        self.val = ::std::option::Option::Some(Value_oneof_val::int_val(v))
    }

    pub fn get_int_val(&self) -> i64 {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::int_val(v)) => v,
            _ => 0,
        }
    }

    // bool bool_val = 4;

    pub fn clear_bool_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_bool_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::bool_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_val(&mut self, v: bool) {
        self.val = ::std::option::Option::Some(Value_oneof_val::bool_val(v))
    }

    pub fn get_bool_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::bool_val(v)) => v,
            _ => false,
        }
    }

    // string str_val = 5;

    pub fn clear_str_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_str_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::str_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_str_val(&mut self, v: ::std::string::String) {
        self.val = ::std::option::Option::Some(Value_oneof_val::str_val(v))
    }

    // Mutable pointer to the field.
    pub fn mut_str_val(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Value_oneof_val::str_val(_)) = self.val {
        } else {
            self.val = ::std::option::Option::Some(Value_oneof_val::str_val(::std::string::String::new()));
        }
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::str_val(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_str_val(&mut self) -> ::std::string::String {
        if self.has_str_val() {
            match self.val.take() {
                ::std::option::Option::Some(Value_oneof_val::str_val(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_str_val(&self) -> &str {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::str_val(ref v)) => v,
            _ => "",
        }
    }

    // double double_val = 6;

    pub fn clear_double_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_double_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::double_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_double_val(&mut self, v: f64) {
        self.val = ::std::option::Option::Some(Value_oneof_val::double_val(v))
    }

    pub fn get_double_val(&self) -> f64 {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::double_val(v)) => v,
            _ => 0.,
        }
    }

    // bytes geo_val = 7;

    pub fn clear_geo_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_geo_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::geo_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_geo_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::std::option::Option::Some(Value_oneof_val::geo_val(v))
    }

    // Mutable pointer to the field.
    pub fn mut_geo_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Value_oneof_val::geo_val(_)) = self.val {
        } else {
            self.val = ::std::option::Option::Some(Value_oneof_val::geo_val(::std::vec::Vec::new()));
        }
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::geo_val(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_geo_val(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_geo_val() {
            match self.val.take() {
                ::std::option::Option::Some(Value_oneof_val::geo_val(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_geo_val(&self) -> &[u8] {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::geo_val(ref v)) => v,
            _ => &[],
        }
    }

    // bytes date_val = 8;

    pub fn clear_date_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_date_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::date_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_date_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::std::option::Option::Some(Value_oneof_val::date_val(v))
    }

    // Mutable pointer to the field.
    pub fn mut_date_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Value_oneof_val::date_val(_)) = self.val {
        } else {
            self.val = ::std::option::Option::Some(Value_oneof_val::date_val(::std::vec::Vec::new()));
        }
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::date_val(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_date_val(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_date_val() {
            match self.val.take() {
                ::std::option::Option::Some(Value_oneof_val::date_val(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_date_val(&self) -> &[u8] {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::date_val(ref v)) => v,
            _ => &[],
        }
    }

    // bytes datetime_val = 9;

    pub fn clear_datetime_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_datetime_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::datetime_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_datetime_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::std::option::Option::Some(Value_oneof_val::datetime_val(v))
    }

    // Mutable pointer to the field.
    pub fn mut_datetime_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Value_oneof_val::datetime_val(_)) = self.val {
        } else {
            self.val = ::std::option::Option::Some(Value_oneof_val::datetime_val(::std::vec::Vec::new()));
        }
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::datetime_val(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_datetime_val(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_datetime_val() {
            match self.val.take() {
                ::std::option::Option::Some(Value_oneof_val::datetime_val(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_datetime_val(&self) -> &[u8] {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::datetime_val(ref v)) => v,
            _ => &[],
        }
    }

    // string password_val = 10;

    pub fn clear_password_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_password_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::password_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_password_val(&mut self, v: ::std::string::String) {
        self.val = ::std::option::Option::Some(Value_oneof_val::password_val(v))
    }

    // Mutable pointer to the field.
    pub fn mut_password_val(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Value_oneof_val::password_val(_)) = self.val {
        } else {
            self.val = ::std::option::Option::Some(Value_oneof_val::password_val(::std::string::String::new()));
        }
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::password_val(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_password_val(&mut self) -> ::std::string::String {
        if self.has_password_val() {
            match self.val.take() {
                ::std::option::Option::Some(Value_oneof_val::password_val(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_password_val(&self) -> &str {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::password_val(ref v)) => v,
            _ => "",
        }
    }

    // uint64 uid_val = 11;

    pub fn clear_uid_val(&mut self) {
        self.val = ::std::option::Option::None;
    }

    pub fn has_uid_val(&self) -> bool {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::uid_val(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uid_val(&mut self, v: u64) {
        self.val = ::std::option::Option::Some(Value_oneof_val::uid_val(v))
    }

    pub fn get_uid_val(&self) -> u64 {
        match self.val {
            ::std::option::Option::Some(Value_oneof_val::uid_val(v)) => v,
            _ => 0,
        }
    }
}

impl ::protobuf::Message for Value {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::default_val(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::bytes_val(is.read_bytes()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::int_val(is.read_int64()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::bool_val(is.read_bool()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::str_val(is.read_string()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::double_val(is.read_double()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::geo_val(is.read_bytes()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::date_val(is.read_bytes()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::datetime_val(is.read_bytes()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::password_val(is.read_string()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.val = ::std::option::Option::Some(Value_oneof_val::uid_val(is.read_uint64()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.val {
            match v {
                &Value_oneof_val::default_val(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &Value_oneof_val::bytes_val(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(2, &v);
                },
                &Value_oneof_val::int_val(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Value_oneof_val::bool_val(v) => {
                    my_size += 2;
                },
                &Value_oneof_val::str_val(ref v) => {
                    my_size += ::protobuf::rt::string_size(5, &v);
                },
                &Value_oneof_val::double_val(v) => {
                    my_size += 9;
                },
                &Value_oneof_val::geo_val(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(7, &v);
                },
                &Value_oneof_val::date_val(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(8, &v);
                },
                &Value_oneof_val::datetime_val(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(9, &v);
                },
                &Value_oneof_val::password_val(ref v) => {
                    my_size += ::protobuf::rt::string_size(10, &v);
                },
                &Value_oneof_val::uid_val(v) => {
                    my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.val {
            match v {
                &Value_oneof_val::default_val(ref v) => {
                    os.write_string(1, v)?;
                },
                &Value_oneof_val::bytes_val(ref v) => {
                    os.write_bytes(2, v)?;
                },
                &Value_oneof_val::int_val(v) => {
                    os.write_int64(3, v)?;
                },
                &Value_oneof_val::bool_val(v) => {
                    os.write_bool(4, v)?;
                },
                &Value_oneof_val::str_val(ref v) => {
                    os.write_string(5, v)?;
                },
                &Value_oneof_val::double_val(v) => {
                    os.write_double(6, v)?;
                },
                &Value_oneof_val::geo_val(ref v) => {
                    os.write_bytes(7, v)?;
                },
                &Value_oneof_val::date_val(ref v) => {
                    os.write_bytes(8, v)?;
                },
                &Value_oneof_val::datetime_val(ref v) => {
                    os.write_bytes(9, v)?;
                },
                &Value_oneof_val::password_val(ref v) => {
                    os.write_string(10, v)?;
                },
                &Value_oneof_val::uid_val(v) => {
                    os.write_uint64(11, v)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Value {
    fn new() -> Value {
        Value::new()
    }

    fn descriptor_static(_: ::std::option::Option<Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "default_val",
                    Value::has_default_val,
                    Value::get_default_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "bytes_val",
                    Value::has_bytes_val,
                    Value::get_bytes_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "int_val",
                    Value::has_int_val,
                    Value::get_int_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "bool_val",
                    Value::has_bool_val,
                    Value::get_bool_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "str_val",
                    Value::has_str_val,
                    Value::get_str_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor::<_>(
                    "double_val",
                    Value::has_double_val,
                    Value::get_double_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "geo_val",
                    Value::has_geo_val,
                    Value::get_geo_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "date_val",
                    Value::has_date_val,
                    Value::get_date_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "datetime_val",
                    Value::has_datetime_val,
                    Value::get_datetime_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "password_val",
                    Value::has_password_val,
                    Value::get_password_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "uid_val",
                    Value::has_uid_val,
                    Value::get_uid_val,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Value>(
                    "Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Value {
    fn clear(&mut self) {
        self.clear_default_val();
        self.clear_bytes_val();
        self.clear_int_val();
        self.clear_bool_val();
        self.clear_str_val();
        self.clear_double_val();
        self.clear_geo_val();
        self.clear_date_val();
        self.clear_datetime_val();
        self.clear_password_val();
        self.clear_uid_val();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Value {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Facet {
    // message fields
    pub key: ::std::string::String,
    pub value: ::std::vec::Vec<u8>,
    pub val_type: Facet_ValType,
    pub tokens: ::protobuf::RepeatedField<::std::string::String>,
    pub alias: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Facet {}

impl Facet {
    pub fn new() -> Facet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Facet {
        static mut instance: ::protobuf::lazy::Lazy<Facet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Facet,
        };
        unsafe {
            instance.get(Facet::new)
        }
    }

    // string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // .api.Facet.ValType val_type = 3;

    pub fn clear_val_type(&mut self) {
        self.val_type = Facet_ValType::STRING;
    }

    // Param is passed by value, moved
    pub fn set_val_type(&mut self, v: Facet_ValType) {
        self.val_type = v;
    }

    pub fn get_val_type(&self) -> Facet_ValType {
        self.val_type
    }

    fn get_val_type_for_reflect(&self) -> &Facet_ValType {
        &self.val_type
    }

    fn mut_val_type_for_reflect(&mut self) -> &mut Facet_ValType {
        &mut self.val_type
    }

    // repeated string tokens = 4;

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokens(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tokens
    }

    // Take field
    pub fn take_tokens(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tokens, ::protobuf::RepeatedField::new())
    }

    pub fn get_tokens(&self) -> &[::std::string::String] {
        &self.tokens
    }

    fn get_tokens_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tokens
    }

    fn mut_tokens_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tokens
    }

    // string alias = 5;

    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alias, ::std::string::String::new())
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    fn get_alias_for_reflect(&self) -> &::std::string::String {
        &self.alias
    }

    fn mut_alias_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.alias
    }
}

impl ::protobuf::Message for Facet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.val_type = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tokens)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alias)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        if self.val_type != Facet_ValType::STRING {
            my_size += ::protobuf::rt::enum_size(3, self.val_type);
        }
        for value in &self.tokens {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if !self.alias.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.alias);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        if self.val_type != Facet_ValType::STRING {
            os.write_enum(3, self.val_type.value())?;
        }
        for v in &self.tokens {
            os.write_string(4, &v)?;
        };
        if !self.alias.is_empty() {
            os.write_string(5, &self.alias)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Facet {
    fn new() -> Facet {
        Facet::new()
    }

    fn descriptor_static(_: ::std::option::Option<Facet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    Facet::get_key_for_reflect,
                    Facet::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Facet::get_value_for_reflect,
                    Facet::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Facet_ValType>>(
                    "val_type",
                    Facet::get_val_type_for_reflect,
                    Facet::mut_val_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tokens",
                    Facet::get_tokens_for_reflect,
                    Facet::mut_tokens_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    Facet::get_alias_for_reflect,
                    Facet::mut_alias_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Facet>(
                    "Facet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Facet {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.clear_val_type();
        self.clear_tokens();
        self.clear_alias();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Facet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Facet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Facet_ValType {
    STRING = 0,
    INT = 1,
    FLOAT = 2,
    BOOL = 3,
    DATETIME = 4,
}

impl ::protobuf::ProtobufEnum for Facet_ValType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Facet_ValType> {
        match value {
            0 => ::std::option::Option::Some(Facet_ValType::STRING),
            1 => ::std::option::Option::Some(Facet_ValType::INT),
            2 => ::std::option::Option::Some(Facet_ValType::FLOAT),
            3 => ::std::option::Option::Some(Facet_ValType::BOOL),
            4 => ::std::option::Option::Some(Facet_ValType::DATETIME),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Facet_ValType] = &[
            Facet_ValType::STRING,
            Facet_ValType::INT,
            Facet_ValType::FLOAT,
            Facet_ValType::BOOL,
            Facet_ValType::DATETIME,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Facet_ValType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Facet_ValType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Facet_ValType {
}

impl ::std::default::Default for Facet_ValType {
    fn default() -> Self {
        Facet_ValType::STRING
    }
}

impl ::protobuf::reflect::ProtobufValue for Facet_ValType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SchemaNode {
    // message fields
    pub predicate: ::std::string::String,
    pub field_type: ::std::string::String,
    pub index: bool,
    pub tokenizer: ::protobuf::RepeatedField<::std::string::String>,
    pub reverse: bool,
    pub count: bool,
    pub list: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SchemaNode {}

impl SchemaNode {
    pub fn new() -> SchemaNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SchemaNode {
        static mut instance: ::protobuf::lazy::Lazy<SchemaNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SchemaNode,
        };
        unsafe {
            instance.get(SchemaNode::new)
        }
    }

    // string predicate = 1;

    pub fn clear_predicate(&mut self) {
        self.predicate.clear();
    }

    // Param is passed by value, moved
    pub fn set_predicate(&mut self, v: ::std::string::String) {
        self.predicate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_predicate(&mut self) -> &mut ::std::string::String {
        &mut self.predicate
    }

    // Take field
    pub fn take_predicate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.predicate, ::std::string::String::new())
    }

    pub fn get_predicate(&self) -> &str {
        &self.predicate
    }

    fn get_predicate_for_reflect(&self) -> &::std::string::String {
        &self.predicate
    }

    fn mut_predicate_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.predicate
    }

    // string type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // bool index = 3;

    pub fn clear_index(&mut self) {
        self.index = false;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: bool) {
        self.index = v;
    }

    pub fn get_index(&self) -> bool {
        self.index
    }

    fn get_index_for_reflect(&self) -> &bool {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut bool {
        &mut self.index
    }

    // repeated string tokenizer = 4;

    pub fn clear_tokenizer(&mut self) {
        self.tokenizer.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokenizer(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tokenizer = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokenizer(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tokenizer
    }

    // Take field
    pub fn take_tokenizer(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tokenizer, ::protobuf::RepeatedField::new())
    }

    pub fn get_tokenizer(&self) -> &[::std::string::String] {
        &self.tokenizer
    }

    fn get_tokenizer_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tokenizer
    }

    fn mut_tokenizer_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tokenizer
    }

    // bool reverse = 5;

    pub fn clear_reverse(&mut self) {
        self.reverse = false;
    }

    // Param is passed by value, moved
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }

    pub fn get_reverse(&self) -> bool {
        self.reverse
    }

    fn get_reverse_for_reflect(&self) -> &bool {
        &self.reverse
    }

    fn mut_reverse_for_reflect(&mut self) -> &mut bool {
        &mut self.reverse
    }

    // bool count = 6;

    pub fn clear_count(&mut self) {
        self.count = false;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: bool) {
        self.count = v;
    }

    pub fn get_count(&self) -> bool {
        self.count
    }

    fn get_count_for_reflect(&self) -> &bool {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut bool {
        &mut self.count
    }

    // bool list = 7;

    pub fn clear_list(&mut self) {
        self.list = false;
    }

    // Param is passed by value, moved
    pub fn set_list(&mut self, v: bool) {
        self.list = v;
    }

    pub fn get_list(&self) -> bool {
        self.list
    }

    fn get_list_for_reflect(&self) -> &bool {
        &self.list
    }

    fn mut_list_for_reflect(&mut self) -> &mut bool {
        &mut self.list
    }
}

impl ::protobuf::Message for SchemaNode {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.predicate)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.index = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tokenizer)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.reverse = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.count = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.list = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.predicate.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.predicate);
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.field_type);
        }
        if self.index != false {
            my_size += 2;
        }
        for value in &self.tokenizer {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if self.reverse != false {
            my_size += 2;
        }
        if self.count != false {
            my_size += 2;
        }
        if self.list != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.predicate.is_empty() {
            os.write_string(1, &self.predicate)?;
        }
        if !self.field_type.is_empty() {
            os.write_string(2, &self.field_type)?;
        }
        if self.index != false {
            os.write_bool(3, self.index)?;
        }
        for v in &self.tokenizer {
            os.write_string(4, &v)?;
        };
        if self.reverse != false {
            os.write_bool(5, self.reverse)?;
        }
        if self.count != false {
            os.write_bool(6, self.count)?;
        }
        if self.list != false {
            os.write_bool(7, self.list)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SchemaNode {
    fn new() -> SchemaNode {
        SchemaNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<SchemaNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "predicate",
                    SchemaNode::get_predicate_for_reflect,
                    SchemaNode::mut_predicate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    SchemaNode::get_field_type_for_reflect,
                    SchemaNode::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "index",
                    SchemaNode::get_index_for_reflect,
                    SchemaNode::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tokenizer",
                    SchemaNode::get_tokenizer_for_reflect,
                    SchemaNode::mut_tokenizer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reverse",
                    SchemaNode::get_reverse_for_reflect,
                    SchemaNode::mut_reverse_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "count",
                    SchemaNode::get_count_for_reflect,
                    SchemaNode::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "list",
                    SchemaNode::get_list_for_reflect,
                    SchemaNode::mut_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SchemaNode>(
                    "SchemaNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SchemaNode {
    fn clear(&mut self) {
        self.clear_predicate();
        self.clear_field_type();
        self.clear_index();
        self.clear_tokenizer();
        self.clear_reverse();
        self.clear_count();
        self.clear_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SchemaNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SchemaNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tapi.proto\x12\x03api\"\xc8\x01\n\x07Request\x12\x14\n\x05query\x18\
    \x01\x20\x01(\tR\x05query\x12*\n\x04vars\x18\x02\x20\x03(\x0b2\x16.api.R\
    equest.VarsEntryR\x04vars\x12\x19\n\x08start_ts\x18\r\x20\x01(\x04R\x07s\
    tartTs\x12'\n\x08lin_read\x18\x0e\x20\x01(\x0b2\x0c.api.LinReadR\x07linR\
    ead\x1a7\n\tVarsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01\"\x92\x01\n\x08Re\
    sponse\x12\x12\n\x04json\x18\x01\x20\x01(\x0cR\x04json\x12'\n\x06schema\
    \x18\x02\x20\x03(\x0b2\x0f.api.SchemaNodeR\x06schema\x12!\n\x03txn\x18\
    \x03\x20\x01(\x0b2\x0f.api.TxnContextR\x03txn\x12&\n\x07latency\x18\x0c\
    \x20\x01(\x0b2\x0c.api.LatencyR\x07latency\"\x9b\x01\n\x08Assigned\x12+\
    \n\x04uids\x18\x01\x20\x03(\x0b2\x17.api.Assigned.UidsEntryR\x04uids\x12\
    )\n\x07context\x18\x02\x20\x01(\x0b2\x0f.api.TxnContextR\x07context\x1a7\
    \n\tUidsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05v\
    alue\x18\x02\x20\x01(\tR\x05value:\x028\x01\"\xae\x02\n\x08Mutation\x12\
    \x19\n\x08set_json\x18\x01\x20\x01(\x0cR\x07setJson\x12\x1f\n\x0bdelete_\
    json\x18\x02\x20\x01(\x0cR\ndeleteJson\x12\x1d\n\nset_nquads\x18\x03\x20\
    \x01(\x0cR\tsetNquads\x12\x1d\n\ndel_nquads\x18\x04\x20\x01(\x0cR\tdelNq\
    uads\x12\x1c\n\x03set\x18\n\x20\x03(\x0b2\n.api.NQuadR\x03set\x12\x1c\n\
    \x03del\x18\x0b\x20\x03(\x0b2\n.api.NQuadR\x03del\x12\x19\n\x08start_ts\
    \x18\r\x20\x01(\x04R\x07startTs\x12\x1d\n\ncommit_now\x18\x0e\x20\x01(\
    \x08R\tcommitNow\x122\n\x15ignore_index_conflict\x18\x0f\x20\x01(\x08R\
    \x13ignoreIndexConflict\"=\n\x0bAssignedIds\x12\x18\n\x07startId\x18\x01\
    \x20\x01(\x04R\x07startId\x12\x14\n\x05endId\x18\x02\x20\x01(\x04R\x05en\
    dId\"[\n\tOperation\x12\x16\n\x06schema\x18\x01\x20\x01(\tR\x06schema\
    \x12\x1b\n\tdrop_attr\x18\x02\x20\x01(\tR\x08dropAttr\x12\x19\n\x08drop_\
    all\x18\x03\x20\x01(\x08R\x07dropAll\"\x1d\n\x07Payload\x12\x12\n\x04Dat\
    a\x18\x01\x20\x01(\x0cR\x04Data\"\x9b\x01\n\nTxnContext\x12\x19\n\x08sta\
    rt_ts\x18\x01\x20\x01(\x04R\x07startTs\x12\x1b\n\tcommit_ts\x18\x02\x20\
    \x01(\x04R\x08commitTs\x12\x18\n\x07aborted\x18\x03\x20\x01(\x08R\x07abo\
    rted\x12\x12\n\x04keys\x18\x04\x20\x03(\tR\x04keys\x12'\n\x08lin_read\
    \x18\r\x20\x01(\x0b2\x0c.api.LinReadR\x07linRead\"\x07\n\x05Check\"\x1b\
    \n\x07Version\x12\x10\n\x03tag\x18\x01\x20\x01(\tR\x03tag\"j\n\x07LinRea\
    d\x12'\n\x03ids\x18\x01\x20\x03(\x0b2\x15.api.LinRead.IdsEntryR\x03ids\
    \x1a6\n\x08IdsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\
    \n\x05value\x18\x02\x20\x01(\x04R\x05value:\x028\x01\"n\n\x07Latency\x12\
    \x1d\n\nparsing_ns\x18\x01\x20\x01(\x04R\tparsingNs\x12#\n\rprocessing_n\
    s\x18\x02\x20\x01(\x04R\x0cprocessingNs\x12\x1f\n\x0bencoding_ns\x18\x03\
    \x20\x01(\x04R\nencodingNs\"\xd9\x01\n\x05NQuad\x12\x18\n\x07subject\x18\
    \x01\x20\x01(\tR\x07subject\x12\x1c\n\tpredicate\x18\x02\x20\x01(\tR\tpr\
    edicate\x12\x1b\n\tobject_id\x18\x03\x20\x01(\tR\x08objectId\x12-\n\x0co\
    bject_value\x18\x04\x20\x01(\x0b2\n.api.ValueR\x0bobjectValue\x12\x14\n\
    \x05label\x18\x05\x20\x01(\tR\x05label\x12\x12\n\x04lang\x18\x06\x20\x01\
    (\tR\x04lang\x12\"\n\x06facets\x18\x07\x20\x03(\x0b2\n.api.FacetR\x06fac\
    ets\"\xe1\x02\n\x05Value\x12!\n\x0bdefault_val\x18\x01\x20\x01(\tH\0R\nd\
    efaultVal\x12\x1d\n\tbytes_val\x18\x02\x20\x01(\x0cH\0R\x08bytesVal\x12\
    \x19\n\x07int_val\x18\x03\x20\x01(\x03H\0R\x06intVal\x12\x1b\n\x08bool_v\
    al\x18\x04\x20\x01(\x08H\0R\x07boolVal\x12\x19\n\x07str_val\x18\x05\x20\
    \x01(\tH\0R\x06strVal\x12\x1f\n\ndouble_val\x18\x06\x20\x01(\x01H\0R\tdo\
    ubleVal\x12\x19\n\x07geo_val\x18\x07\x20\x01(\x0cH\0R\x06geoVal\x12\x1b\
    \n\x08date_val\x18\x08\x20\x01(\x0cH\0R\x07dateVal\x12#\n\x0cdatetime_va\
    l\x18\t\x20\x01(\x0cH\0R\x0bdatetimeVal\x12#\n\x0cpassword_val\x18\n\x20\
    \x01(\tH\0R\x0bpasswordVal\x12\x19\n\x07uid_val\x18\x0b\x20\x01(\x04H\0R\
    \x06uidValB\x05\n\x03val\"\xcf\x01\n\x05Facet\x12\x10\n\x03key\x18\x01\
    \x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value\
    \x12-\n\x08val_type\x18\x03\x20\x01(\x0e2\x12.api.Facet.ValTypeR\x07valT\
    ype\x12\x16\n\x06tokens\x18\x04\x20\x03(\tR\x06tokens\x12\x14\n\x05alias\
    \x18\x05\x20\x01(\tR\x05alias\"A\n\x07ValType\x12\n\n\x06STRING\x10\0\
    \x12\x07\n\x03INT\x10\x01\x12\t\n\x05FLOAT\x10\x02\x12\x08\n\x04BOOL\x10\
    \x03\x12\x0c\n\x08DATETIME\x10\x04\"\xb6\x01\n\nSchemaNode\x12\x1c\n\tpr\
    edicate\x18\x01\x20\x01(\tR\tpredicate\x12\x12\n\x04type\x18\x02\x20\x01\
    (\tR\x04type\x12\x14\n\x05index\x18\x03\x20\x01(\x08R\x05index\x12\x1c\n\
    \ttokenizer\x18\x04\x20\x03(\tR\ttokenizer\x12\x18\n\x07reverse\x18\x05\
    \x20\x01(\x08R\x07reverse\x12\x14\n\x05count\x18\x06\x20\x01(\x08R\x05co\
    unt\x12\x12\n\x04list\x18\x07\x20\x01(\x08R\x04list2\xe4\x01\n\x06Dgraph\
    \x12&\n\x05Query\x12\x0c.api.Request\x1a\r.api.Response\"\0\x12(\n\x06Mu\
    tate\x12\r.api.Mutation\x1a\r.api.Assigned\"\0\x12'\n\x05Alter\x12\x0e.a\
    pi.Operation\x1a\x0c.api.Payload\"\0\x123\n\rCommitOrAbort\x12\x0f.api.T\
    xnContext\x1a\x0f.api.TxnContext\"\0\x12*\n\x0cCheckVersion\x12\n.api.Ch\
    eck\x1a\x0c.api.Version\"\0B\x18\n\tio.dgraphB\x0bDgraphProtoJ\xfd1\n\
    \x07\x12\x05\x10\0\x9c\x01\x01\n\xd8\x04\n\x01\x0c\x12\x03\x10\0\x122\
    \xcd\x04\n\x20Copyright\x20(C)\x202017\x20Dgraph\x20Labs,\x20Inc.\x20and\
    \x20Contributors\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\
    \x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20us\
    e\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20Licens\
    e.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\
    \n\n\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Un\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\x20\
    is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20\
    WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20expres\
    s\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x12\x08\x0b\n\x08\n\x01\
    \x08\x12\x03\x14\0\"\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x14\0\"\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x14\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x14\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x14\x07\
    \x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x14\x16!\n\x08\n\x01\x08\x12\
    \x03\x15\0,\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x15\0,\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03\x15\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\x15\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x15\x07\
    \x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x15\x1e+\n\x1d\n\x02\x06\0\
    \x12\x04\x18\0\x1e\x01\x1a\x11\x20Graph\x20response.\n\n\n\n\x03\x06\0\
    \x01\x12\x03\x18\x08\x0e\n\x0b\n\x04\x06\0\x02\0\x12\x03\x19\x08<\n\x0c\
    \n\x05\x06\0\x02\0\x01\x12\x03\x19\x0c\x11\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x03\x19\x13\x1a\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x1908\n\x0b\n\
    \x04\x06\0\x02\x01\x12\x03\x1a\x08<\n\x0c\n\x05\x06\0\x02\x01\x01\x12\
    \x03\x1a\x0c\x12\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x1a\x14\x1c\n\x0c\
    \n\x05\x06\0\x02\x01\x03\x12\x03\x1a08\n\x0b\n\x04\x06\0\x02\x02\x12\x03\
    \x1b\x08;\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\x1b\x0c\x11\n\x0c\n\x05\
    \x06\0\x02\x02\x02\x12\x03\x1b\x13\x1c\n\x0c\n\x05\x06\0\x02\x02\x03\x12\
    \x03\x1b07\n\x0b\n\x04\x06\0\x02\x03\x12\x03\x1c\x08>\n\x0c\n\x05\x06\0\
    \x02\x03\x01\x12\x03\x1c\x0c\x19\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\
    \x1c\x1b%\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03\x1c0:\n\x0b\n\x04\x06\0\
    \x02\x04\x12\x03\x1d\x08;\n\x0c\n\x05\x06\0\x02\x04\x01\x12\x03\x1d\x0c\
    \x18\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03\x1d\x19\x1e\n\x0c\n\x05\x06\0\
    \x02\x04\x03\x12\x03\x1d07\n\n\n\x02\x04\0\x12\x04\x20\0&\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x20\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03!\x08\x19\
    \n\r\n\x05\x04\0\x02\0\x04\x12\x04!\x08\x20\x11\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03!\x08\x0e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03!\x0f\x14\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03!\x17\x18\n2\n\x04\x04\0\x02\x01\x12\x03\"\
    \x08%\"%\x20Support\x20for\x20GraphQL\x20like\x20variables.\n\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\"\x08!\x19\n\x0c\n\x05\x04\0\x02\x01\x06\x12\
    \x03\"\x08\x1b\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\"\x1c\x20\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\"#$\n\x0b\n\x04\x04\0\x02\x02\x12\x03$\
    \x08\x1d\n\r\n\x05\x04\0\x02\x02\x04\x12\x04$\x08\"%\n\x0c\n\x05\x04\0\
    \x02\x02\x05\x12\x03$\x08\x0e\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03$\x0f\
    \x17\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03$\x1a\x1c\n\x0b\n\x04\x04\0\
    \x02\x03\x12\x03%\x08\x1e\n\r\n\x05\x04\0\x02\x03\x04\x12\x04%\x08$\x1d\
    \n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03%\x08\x0f\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03%\x10\x18\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03%\x1b\x1d\
    \n\n\n\x02\x04\x01\x12\x04(\0-\x01\n\n\n\x03\x04\x01\x01\x12\x03(\x08\
    \x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03)\x08\x17\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04)\x08(\x12\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03)\x08\r\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03)\x0e\x12\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03)\x15\x16\n\x0b\n\x04\x04\x01\x02\x01\x12\x03*\x08'\n\x0c\n\
    \x05\x04\x01\x02\x01\x04\x12\x03*\x08\x10\n\x0c\n\x05\x04\x01\x02\x01\
    \x06\x12\x03*\x11\x1b\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03*\x1c\"\n\
    \x0c\n\x05\x04\x01\x02\x01\x03\x12\x03*%&\n\x0b\n\x04\x04\x01\x02\x02\
    \x12\x03+\x08\x1b\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04+\x08*'\n\x0c\n\
    \x05\x04\x01\x02\x02\x06\x12\x03+\x08\x12\n\x0c\n\x05\x04\x01\x02\x02\
    \x01\x12\x03+\x13\x16\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03+\x19\x1a\n\
    \x0b\n\x04\x04\x01\x02\x03\x12\x03,\x08\x1d\n\r\n\x05\x04\x01\x02\x03\
    \x04\x12\x04,\x08+\x1b\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03,\x08\x0f\
    \n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03,\x10\x17\n\x0c\n\x05\x04\x01\
    \x02\x03\x03\x12\x03,\x1a\x1c\n\n\n\x02\x04\x02\x12\x04/\02\x01\n\n\n\
    \x03\x04\x02\x01\x12\x03/\x08\x10\n\x0b\n\x04\x04\x02\x02\0\x12\x030\x08\
    %\n\r\n\x05\x04\x02\x02\0\x04\x12\x040\x08/\x12\n\x0c\n\x05\x04\x02\x02\
    \0\x06\x12\x030\x08\x1b\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x030\x1c\x20\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x030#$\n\x0b\n\x04\x04\x02\x02\x01\x12\
    \x031\x08\x1f\n\r\n\x05\x04\x02\x02\x01\x04\x12\x041\x080%\n\x0c\n\x05\
    \x04\x02\x02\x01\x06\x12\x031\x08\x12\n\x0c\n\x05\x04\x02\x02\x01\x01\
    \x12\x031\x13\x1a\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x031\x1d\x1e\n\n\n\
    \x02\x04\x03\x12\x044\0?\x01\n\n\n\x03\x04\x03\x01\x12\x034\x08\x10\n\
    \x0b\n\x04\x04\x03\x02\0\x12\x035\x08\x1b\n\r\n\x05\x04\x03\x02\0\x04\
    \x12\x045\x084\x12\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x035\x08\r\n\x0c\n\
    \x05\x04\x03\x02\0\x01\x12\x035\x0e\x16\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x035\x19\x1a\n\x0b\n\x04\x04\x03\x02\x01\x12\x036\x08\x1e\n\r\n\x05\
    \x04\x03\x02\x01\x04\x12\x046\x085\x1b\n\x0c\n\x05\x04\x03\x02\x01\x05\
    \x12\x036\x08\r\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x036\x0e\x19\n\x0c\n\
    \x05\x04\x03\x02\x01\x03\x12\x036\x1c\x1d\n\x0b\n\x04\x04\x03\x02\x02\
    \x12\x037\x08\x1d\n\r\n\x05\x04\x03\x02\x02\x04\x12\x047\x086\x1e\n\x0c\
    \n\x05\x04\x03\x02\x02\x05\x12\x037\x08\r\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x037\x0e\x18\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x037\x1b\x1c\n\
    \x0b\n\x04\x04\x03\x02\x03\x12\x038\x08\x1d\n\r\n\x05\x04\x03\x02\x03\
    \x04\x12\x048\x087\x1d\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x038\x08\r\n\
    \x0c\n\x05\x04\x03\x02\x03\x01\x12\x038\x0e\x18\n\x0c\n\x05\x04\x03\x02\
    \x03\x03\x12\x038\x1b\x1c\n\x0b\n\x04\x04\x03\x02\x04\x12\x03:\x08\x20\n\
    \x0c\n\x05\x04\x03\x02\x04\x04\x12\x03:\x08\x10\n\x0c\n\x05\x04\x03\x02\
    \x04\x06\x12\x03:\x11\x16\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\x03:\x17\
    \x1a\n\x0c\n\x05\x04\x03\x02\x04\x03\x12\x03:\x1d\x1f\n\x0b\n\x04\x04\
    \x03\x02\x05\x12\x03;\x08\x20\n\x0c\n\x05\x04\x03\x02\x05\x04\x12\x03;\
    \x08\x10\n\x0c\n\x05\x04\x03\x02\x05\x06\x12\x03;\x11\x16\n\x0c\n\x05\
    \x04\x03\x02\x05\x01\x12\x03;\x17\x1a\n\x0c\n\x05\x04\x03\x02\x05\x03\
    \x12\x03;\x1d\x1f\n\x0b\n\x04\x04\x03\x02\x06\x12\x03<\x08\x1d\n\r\n\x05\
    \x04\x03\x02\x06\x04\x12\x04<\x08;\x20\n\x0c\n\x05\x04\x03\x02\x06\x05\
    \x12\x03<\x08\x0e\n\x0c\n\x05\x04\x03\x02\x06\x01\x12\x03<\x0f\x17\n\x0c\
    \n\x05\x04\x03\x02\x06\x03\x12\x03<\x1a\x1c\n\x0b\n\x04\x04\x03\x02\x07\
    \x12\x03=\x08\x1d\n\r\n\x05\x04\x03\x02\x07\x04\x12\x04=\x08<\x1d\n\x0c\
    \n\x05\x04\x03\x02\x07\x05\x12\x03=\x08\x0c\n\x0c\n\x05\x04\x03\x02\x07\
    \x01\x12\x03=\r\x17\n\x0c\n\x05\x04\x03\x02\x07\x03\x12\x03=\x1a\x1c\n\
    \x0b\n\x04\x04\x03\x02\x08\x12\x03>\x08(\n\r\n\x05\x04\x03\x02\x08\x04\
    \x12\x04>\x08=\x1d\n\x0c\n\x05\x04\x03\x02\x08\x05\x12\x03>\x08\x0c\n\
    \x0c\n\x05\x04\x03\x02\x08\x01\x12\x03>\r\"\n\x0c\n\x05\x04\x03\x02\x08\
    \x03\x12\x03>%'\n\n\n\x02\x04\x04\x12\x04B\0E\x01\n\n\n\x03\x04\x04\x01\
    \x12\x03B\x08\x13\n\x0b\n\x04\x04\x04\x02\0\x12\x03C\x08\x1b\n\r\n\x05\
    \x04\x04\x02\0\x04\x12\x04C\x08B\x15\n\x0c\n\x05\x04\x04\x02\0\x05\x12\
    \x03C\x08\x0e\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03C\x0f\x16\n\x0c\n\x05\
    \x04\x04\x02\0\x03\x12\x03C\x19\x1a\n\x0b\n\x04\x04\x04\x02\x01\x12\x03D\
    \x08\x19\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04D\x08C\x1b\n\x0c\n\x05\x04\
    \x04\x02\x01\x05\x12\x03D\x08\x0e\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\
    \x03D\x0f\x14\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03D\x17\x18\n\n\n\x02\
    \x04\x05\x12\x04G\0K\x01\n\n\n\x03\x04\x05\x01\x12\x03G\x08\x11\n\x0b\n\
    \x04\x04\x05\x02\0\x12\x03H\x08\x1a\n\r\n\x05\x04\x05\x02\0\x04\x12\x04H\
    \x08G\x13\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03H\x08\x0e\n\x0c\n\x05\x04\
    \x05\x02\0\x01\x12\x03H\x0f\x15\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03H\
    \x18\x19\n\x0b\n\x04\x04\x05\x02\x01\x12\x03I\x08\x1d\n\r\n\x05\x04\x05\
    \x02\x01\x04\x12\x04I\x08H\x1a\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03I\
    \x08\x0e\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03I\x0f\x18\n\x0c\n\x05\
    \x04\x05\x02\x01\x03\x12\x03I\x1b\x1c\n\x0b\n\x04\x04\x05\x02\x02\x12\
    \x03J\x08\x1a\n\r\n\x05\x04\x05\x02\x02\x04\x12\x04J\x08I\x1d\n\x0c\n\
    \x05\x04\x05\x02\x02\x05\x12\x03J\x08\x0c\n\x0c\n\x05\x04\x05\x02\x02\
    \x01\x12\x03J\r\x15\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03J\x18\x19\n\
    \x1e\n\x02\x04\x06\x12\x04N\0P\x01\x1a\x12\x20Worker\x20services.\n\n\n\
    \n\x03\x04\x06\x01\x12\x03N\x08\x0f\n\x0b\n\x04\x04\x06\x02\0\x12\x03O\
    \x08\x17\n\r\n\x05\x04\x06\x02\0\x04\x12\x04O\x08N\x11\n\x0c\n\x05\x04\
    \x06\x02\0\x05\x12\x03O\x08\r\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03O\x0e\
    \x12\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03O\x15\x16\n\n\n\x02\x04\x07\
    \x12\x04R\0X\x01\n\n\n\x03\x04\x07\x01\x12\x03R\x08\x12\n\x0b\n\x04\x04\
    \x07\x02\0\x12\x03S\x08\x1c\n\r\n\x05\x04\x07\x02\0\x04\x12\x04S\x08R\
    \x14\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03S\x08\x0e\n\x0c\n\x05\x04\x07\
    \x02\0\x01\x12\x03S\x0f\x17\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03S\x1a\
    \x1b\n\x0b\n\x04\x04\x07\x02\x01\x12\x03T\x08\x1d\n\r\n\x05\x04\x07\x02\
    \x01\x04\x12\x04T\x08S\x1c\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x03T\x08\
    \x0e\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03T\x0f\x18\n\x0c\n\x05\x04\
    \x07\x02\x01\x03\x12\x03T\x1b\x1c\n\x0b\n\x04\x04\x07\x02\x02\x12\x03U\
    \x08\x19\n\r\n\x05\x04\x07\x02\x02\x04\x12\x04U\x08T\x1d\n\x0c\n\x05\x04\
    \x07\x02\x02\x05\x12\x03U\x08\x0c\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\
    \x03U\r\x14\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x03U\x17\x18\n\x0b\n\x04\
    \x04\x07\x02\x03\x12\x03V\x08!\n\x0c\n\x05\x04\x07\x02\x03\x04\x12\x03V\
    \x08\x10\n\x0c\n\x05\x04\x07\x02\x03\x05\x12\x03V\x11\x17\n\x0c\n\x05\
    \x04\x07\x02\x03\x01\x12\x03V\x18\x1c\n\x0c\n\x05\x04\x07\x02\x03\x03\
    \x12\x03V\x1f\x20\n\x0b\n\x04\x04\x07\x02\x04\x12\x03W\x08\x1e\n\r\n\x05\
    \x04\x07\x02\x04\x04\x12\x04W\x08V!\n\x0c\n\x05\x04\x07\x02\x04\x06\x12\
    \x03W\x08\x0f\n\x0c\n\x05\x04\x07\x02\x04\x01\x12\x03W\x10\x18\n\x0c\n\
    \x05\x04\x07\x02\x04\x03\x12\x03W\x1b\x1d\n\t\n\x02\x04\x08\x12\x03Z\0\
    \x10\n\n\n\x03\x04\x08\x01\x12\x03Z\x08\r\n\n\n\x02\x04\t\x12\x04\\\0^\
    \x01\n\n\n\x03\x04\t\x01\x12\x03\\\x08\x0f\n\x0b\n\x04\x04\t\x02\0\x12\
    \x03]\x08\x17\n\r\n\x05\x04\t\x02\0\x04\x12\x04]\x08\\\x11\n\x0c\n\x05\
    \x04\t\x02\0\x05\x12\x03]\x08\x0e\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03]\
    \x0f\x12\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03]\x15\x16\n\n\n\x02\x04\n\
    \x12\x04`\0b\x01\n\n\n\x03\x04\n\x01\x12\x03`\x08\x0f\n\x0b\n\x04\x04\n\
    \x02\0\x12\x03a\x08$\n\r\n\x05\x04\n\x02\0\x04\x12\x04a\x08`\x11\n\x0c\n\
    \x05\x04\n\x02\0\x06\x12\x03a\x08\x1b\n\x0c\n\x05\x04\n\x02\0\x01\x12\
    \x03a\x1c\x1f\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03a\"#\n\n\n\x02\x04\x0b\
    \x12\x04d\0h\x01\n\n\n\x03\x04\x0b\x01\x12\x03d\x08\x0f\n\x0b\n\x04\x04\
    \x0b\x02\0\x12\x03e\x08\x1e\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04e\x08d\
    \x11\n\x0c\n\x05\x04\x0b\x02\0\x05\x12\x03e\x08\x0e\n\x0c\n\x05\x04\x0b\
    \x02\0\x01\x12\x03e\x0f\x19\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03e\x1c\
    \x1d\n\x0b\n\x04\x04\x0b\x02\x01\x12\x03f\x08!\n\r\n\x05\x04\x0b\x02\x01\
    \x04\x12\x04f\x08e\x1e\n\x0c\n\x05\x04\x0b\x02\x01\x05\x12\x03f\x08\x0e\
    \n\x0c\n\x05\x04\x0b\x02\x01\x01\x12\x03f\x0f\x1c\n\x0c\n\x05\x04\x0b\
    \x02\x01\x03\x12\x03f\x1f\x20\n\x0b\n\x04\x04\x0b\x02\x02\x12\x03g\x08\
    \x1f\n\r\n\x05\x04\x0b\x02\x02\x04\x12\x04g\x08f!\n\x0c\n\x05\x04\x0b\
    \x02\x02\x05\x12\x03g\x08\x0e\n\x0c\n\x05\x04\x0b\x02\x02\x01\x12\x03g\
    \x0f\x1a\n\x0c\n\x05\x04\x0b\x02\x02\x03\x12\x03g\x1d\x1e\n\n\n\x02\x04\
    \x0c\x12\x04j\0r\x01\n\n\n\x03\x04\x0c\x01\x12\x03j\x08\r\n\x0b\n\x04\
    \x04\x0c\x02\0\x12\x03k\x08\x1b\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04k\x08\
    j\x0f\n\x0c\n\x05\x04\x0c\x02\0\x05\x12\x03k\x08\x0e\n\x0c\n\x05\x04\x0c\
    \x02\0\x01\x12\x03k\x0f\x16\n\x0c\n\x05\x04\x0c\x02\0\x03\x12\x03k\x19\
    \x1a\n\x0b\n\x04\x04\x0c\x02\x01\x12\x03l\x08\x1d\n\r\n\x05\x04\x0c\x02\
    \x01\x04\x12\x04l\x08k\x1b\n\x0c\n\x05\x04\x0c\x02\x01\x05\x12\x03l\x08\
    \x0e\n\x0c\n\x05\x04\x0c\x02\x01\x01\x12\x03l\x0f\x18\n\x0c\n\x05\x04\
    \x0c\x02\x01\x03\x12\x03l\x1b\x1c\n\x0b\n\x04\x04\x0c\x02\x02\x12\x03m\
    \x08\x1d\n\r\n\x05\x04\x0c\x02\x02\x04\x12\x04m\x08l\x1d\n\x0c\n\x05\x04\
    \x0c\x02\x02\x05\x12\x03m\x08\x0e\n\x0c\n\x05\x04\x0c\x02\x02\x01\x12\
    \x03m\x0f\x18\n\x0c\n\x05\x04\x0c\x02\x02\x03\x12\x03m\x1b\x1c\n\x0b\n\
    \x04\x04\x0c\x02\x03\x12\x03n\x08\x1f\n\r\n\x05\x04\x0c\x02\x03\x04\x12\
    \x04n\x08m\x1d\n\x0c\n\x05\x04\x0c\x02\x03\x06\x12\x03n\x08\r\n\x0c\n\
    \x05\x04\x0c\x02\x03\x01\x12\x03n\x0e\x1a\n\x0c\n\x05\x04\x0c\x02\x03\
    \x03\x12\x03n\x1d\x1e\n\x0b\n\x04\x04\x0c\x02\x04\x12\x03o\x08\x19\n\r\n\
    \x05\x04\x0c\x02\x04\x04\x12\x04o\x08n\x1f\n\x0c\n\x05\x04\x0c\x02\x04\
    \x05\x12\x03o\x08\x0e\n\x0c\n\x05\x04\x0c\x02\x04\x01\x12\x03o\x0f\x14\n\
    \x0c\n\x05\x04\x0c\x02\x04\x03\x12\x03o\x17\x18\n\x0b\n\x04\x04\x0c\x02\
    \x05\x12\x03p\x08\x18\n\r\n\x05\x04\x0c\x02\x05\x04\x12\x04p\x08o\x19\n\
    \x0c\n\x05\x04\x0c\x02\x05\x05\x12\x03p\x08\x0e\n\x0c\n\x05\x04\x0c\x02\
    \x05\x01\x12\x03p\x0f\x13\n\x0c\n\x05\x04\x0c\x02\x05\x03\x12\x03p\x16\
    \x17\n\x0b\n\x04\x04\x0c\x02\x06\x12\x03q\x08\"\n\x0c\n\x05\x04\x0c\x02\
    \x06\x04\x12\x03q\x08\x10\n\x0c\n\x05\x04\x0c\x02\x06\x06\x12\x03q\x11\
    \x16\n\x0c\n\x05\x04\x0c\x02\x06\x01\x12\x03q\x17\x1d\n\x0c\n\x05\x04\
    \x0c\x02\x06\x03\x12\x03q\x20!\n\x0b\n\x02\x04\r\x12\x05t\0\x82\x01\x01\
    \n\n\n\x03\x04\r\x01\x12\x03t\x08\r\n\r\n\x04\x04\r\x08\0\x12\x05u\x08\
    \x81\x01\t\n\x0c\n\x05\x04\r\x08\0\x01\x12\x03u\x0e\x11\n\x0b\n\x04\x04\
    \r\x02\0\x12\x03v\x10'\n\x0c\n\x05\x04\r\x02\0\x05\x12\x03v\x10\x16\n\
    \x0c\n\x05\x04\r\x02\0\x01\x12\x03v\x17\"\n\x0c\n\x05\x04\r\x02\0\x03\
    \x12\x03v%&\n\x0b\n\x04\x04\r\x02\x01\x12\x03w\x10$\n\x0c\n\x05\x04\r\
    \x02\x01\x05\x12\x03w\x10\x15\n\x0c\n\x05\x04\r\x02\x01\x01\x12\x03w\x16\
    \x1f\n\x0c\n\x05\x04\r\x02\x01\x03\x12\x03w\"#\n\x0b\n\x04\x04\r\x02\x02\
    \x12\x03x\x10\"\n\x0c\n\x05\x04\r\x02\x02\x05\x12\x03x\x10\x15\n\x0c\n\
    \x05\x04\r\x02\x02\x01\x12\x03x\x16\x1d\n\x0c\n\x05\x04\r\x02\x02\x03\
    \x12\x03x\x20!\n\x0b\n\x04\x04\r\x02\x03\x12\x03y\x10\"\n\x0c\n\x05\x04\
    \r\x02\x03\x05\x12\x03y\x10\x14\n\x0c\n\x05\x04\r\x02\x03\x01\x12\x03y\
    \x15\x1d\n\x0c\n\x05\x04\r\x02\x03\x03\x12\x03y\x20!\n\x0b\n\x04\x04\r\
    \x02\x04\x12\x03z\x10#\n\x0c\n\x05\x04\r\x02\x04\x05\x12\x03z\x10\x16\n\
    \x0c\n\x05\x04\r\x02\x04\x01\x12\x03z\x17\x1e\n\x0c\n\x05\x04\r\x02\x04\
    \x03\x12\x03z!\"\n\x0b\n\x04\x04\r\x02\x05\x12\x03{\x10&\n\x0c\n\x05\x04\
    \r\x02\x05\x05\x12\x03{\x10\x16\n\x0c\n\x05\x04\r\x02\x05\x01\x12\x03{\
    \x17!\n\x0c\n\x05\x04\r\x02\x05\x03\x12\x03{$%\n%\n\x04\x04\r\x02\x06\
    \x12\x03|\x10\"\"\x18\x20Geo\x20data\x20in\x20WKB\x20format\n\n\x0c\n\
    \x05\x04\r\x02\x06\x05\x12\x03|\x10\x15\n\x0c\n\x05\x04\r\x02\x06\x01\
    \x12\x03|\x16\x1d\n\x0c\n\x05\x04\r\x02\x06\x03\x12\x03|\x20!\n\x0b\n\
    \x04\x04\r\x02\x07\x12\x03}\x10#\n\x0c\n\x05\x04\r\x02\x07\x05\x12\x03}\
    \x10\x15\n\x0c\n\x05\x04\r\x02\x07\x01\x12\x03}\x16\x1e\n\x0c\n\x05\x04\
    \r\x02\x07\x03\x12\x03}!\"\n\x0b\n\x04\x04\r\x02\x08\x12\x03~\x10'\n\x0c\
    \n\x05\x04\r\x02\x08\x05\x12\x03~\x10\x15\n\x0c\n\x05\x04\r\x02\x08\x01\
    \x12\x03~\x16\"\n\x0c\n\x05\x04\r\x02\x08\x03\x12\x03~%&\n\x0b\n\x04\x04\
    \r\x02\t\x12\x03\x7f\x10)\n\x0c\n\x05\x04\r\x02\t\x05\x12\x03\x7f\x10\
    \x16\n\x0c\n\x05\x04\r\x02\t\x01\x12\x03\x7f\x17#\n\x0c\n\x05\x04\r\x02\
    \t\x03\x12\x03\x7f&(\n\x0c\n\x04\x04\r\x02\n\x12\x04\x80\x01\x10\"\n\r\n\
    \x05\x04\r\x02\n\x05\x12\x04\x80\x01\x10\x16\n\r\n\x05\x04\r\x02\n\x01\
    \x12\x04\x80\x01\x17\x1e\n\r\n\x05\x04\r\x02\n\x03\x12\x04\x80\x01\x1f!\
    \n\x0c\n\x02\x04\x0e\x12\x06\x84\x01\0\x92\x01\x01\n\x0b\n\x03\x04\x0e\
    \x01\x12\x04\x84\x01\x08\r\n\x0e\n\x04\x04\x0e\x04\0\x12\x06\x85\x01\x08\
    \x8b\x01\t\n\r\n\x05\x04\x0e\x04\0\x01\x12\x04\x85\x01\r\x14\n\x0e\n\x06\
    \x04\x0e\x04\0\x02\0\x12\x04\x86\x01\x10\x1b\n\x0f\n\x07\x04\x0e\x04\0\
    \x02\0\x01\x12\x04\x86\x01\x10\x16\n\x0f\n\x07\x04\x0e\x04\0\x02\0\x02\
    \x12\x04\x86\x01\x19\x1a\n\x0e\n\x06\x04\x0e\x04\0\x02\x01\x12\x04\x87\
    \x01\x10\x18\n\x0f\n\x07\x04\x0e\x04\0\x02\x01\x01\x12\x04\x87\x01\x10\
    \x13\n\x0f\n\x07\x04\x0e\x04\0\x02\x01\x02\x12\x04\x87\x01\x16\x17\n\x0e\
    \n\x06\x04\x0e\x04\0\x02\x02\x12\x04\x88\x01\x10\x1a\n\x0f\n\x07\x04\x0e\
    \x04\0\x02\x02\x01\x12\x04\x88\x01\x10\x15\n\x0f\n\x07\x04\x0e\x04\0\x02\
    \x02\x02\x12\x04\x88\x01\x18\x19\n\x0e\n\x06\x04\x0e\x04\0\x02\x03\x12\
    \x04\x89\x01\x10\x19\n\x0f\n\x07\x04\x0e\x04\0\x02\x03\x01\x12\x04\x89\
    \x01\x10\x14\n\x0f\n\x07\x04\x0e\x04\0\x02\x03\x02\x12\x04\x89\x01\x17\
    \x18\n\x0e\n\x06\x04\x0e\x04\0\x02\x04\x12\x04\x8a\x01\x10\x1d\n\x0f\n\
    \x07\x04\x0e\x04\0\x02\x04\x01\x12\x04\x8a\x01\x10\x18\n\x0f\n\x07\x04\
    \x0e\x04\0\x02\x04\x02\x12\x04\x8a\x01\x1b\x1c\n\x0c\n\x04\x04\x0e\x02\0\
    \x12\x04\x8d\x01\x08\x17\n\x0f\n\x05\x04\x0e\x02\0\x04\x12\x06\x8d\x01\
    \x08\x8b\x01\t\n\r\n\x05\x04\x0e\x02\0\x05\x12\x04\x8d\x01\x08\x0e\n\r\n\
    \x05\x04\x0e\x02\0\x01\x12\x04\x8d\x01\x0f\x12\n\r\n\x05\x04\x0e\x02\0\
    \x03\x12\x04\x8d\x01\x15\x16\n\x0c\n\x04\x04\x0e\x02\x01\x12\x04\x8e\x01\
    \x08\x18\n\x0f\n\x05\x04\x0e\x02\x01\x04\x12\x06\x8e\x01\x08\x8d\x01\x17\
    \n\r\n\x05\x04\x0e\x02\x01\x05\x12\x04\x8e\x01\x08\r\n\r\n\x05\x04\x0e\
    \x02\x01\x01\x12\x04\x8e\x01\x0e\x13\n\r\n\x05\x04\x0e\x02\x01\x03\x12\
    \x04\x8e\x01\x16\x17\n\x0c\n\x04\x04\x0e\x02\x02\x12\x04\x8f\x01\x08\x1d\
    \n\x0f\n\x05\x04\x0e\x02\x02\x04\x12\x06\x8f\x01\x08\x8e\x01\x18\n\r\n\
    \x05\x04\x0e\x02\x02\x06\x12\x04\x8f\x01\x08\x0f\n\r\n\x05\x04\x0e\x02\
    \x02\x01\x12\x04\x8f\x01\x10\x18\n\r\n\x05\x04\x0e\x02\x02\x03\x12\x04\
    \x8f\x01\x1b\x1c\n\x20\n\x04\x04\x0e\x02\x03\x12\x04\x90\x01\x08#\"\x12\
    \x20tokens\x20of\x20value.\n\n\r\n\x05\x04\x0e\x02\x03\x04\x12\x04\x90\
    \x01\x08\x10\n\r\n\x05\x04\x0e\x02\x03\x05\x12\x04\x90\x01\x11\x17\n\r\n\
    \x05\x04\x0e\x02\x03\x01\x12\x04\x90\x01\x18\x1e\n\r\n\x05\x04\x0e\x02\
    \x03\x03\x12\x04\x90\x01!\"\n0\n\x04\x04\x0e\x02\x04\x12\x04\x91\x01\x08\
    \x19\"\"\x20not\x20stored,\x20only\x20used\x20for\x20query.\n\n\x0f\n\
    \x05\x04\x0e\x02\x04\x04\x12\x06\x91\x01\x08\x90\x01#\n\r\n\x05\x04\x0e\
    \x02\x04\x05\x12\x04\x91\x01\x08\x0e\n\r\n\x05\x04\x0e\x02\x04\x01\x12\
    \x04\x91\x01\x0f\x14\n\r\n\x05\x04\x0e\x02\x04\x03\x12\x04\x91\x01\x17\
    \x18\n\x0c\n\x02\x04\x0f\x12\x06\x94\x01\0\x9c\x01\x01\n\x0b\n\x03\x04\
    \x0f\x01\x12\x04\x94\x01\x08\x12\n\x0c\n\x04\x04\x0f\x02\0\x12\x04\x95\
    \x01\x08\x1d\n\x0f\n\x05\x04\x0f\x02\0\x04\x12\x06\x95\x01\x08\x94\x01\
    \x14\n\r\n\x05\x04\x0f\x02\0\x05\x12\x04\x95\x01\x08\x0e\n\r\n\x05\x04\
    \x0f\x02\0\x01\x12\x04\x95\x01\x0f\x18\n\r\n\x05\x04\x0f\x02\0\x03\x12\
    \x04\x95\x01\x1b\x1c\n\x0c\n\x04\x04\x0f\x02\x01\x12\x04\x96\x01\x08\x18\
    \n\x0f\n\x05\x04\x0f\x02\x01\x04\x12\x06\x96\x01\x08\x95\x01\x1d\n\r\n\
    \x05\x04\x0f\x02\x01\x05\x12\x04\x96\x01\x08\x0e\n\r\n\x05\x04\x0f\x02\
    \x01\x01\x12\x04\x96\x01\x0f\x13\n\r\n\x05\x04\x0f\x02\x01\x03\x12\x04\
    \x96\x01\x16\x17\n\x0c\n\x04\x04\x0f\x02\x02\x12\x04\x97\x01\x08\x17\n\
    \x0f\n\x05\x04\x0f\x02\x02\x04\x12\x06\x97\x01\x08\x96\x01\x18\n\r\n\x05\
    \x04\x0f\x02\x02\x05\x12\x04\x97\x01\x08\x0c\n\r\n\x05\x04\x0f\x02\x02\
    \x01\x12\x04\x97\x01\r\x12\n\r\n\x05\x04\x0f\x02\x02\x03\x12\x04\x97\x01\
    \x15\x16\n\x0c\n\x04\x04\x0f\x02\x03\x12\x04\x98\x01\x08&\n\r\n\x05\x04\
    \x0f\x02\x03\x04\x12\x04\x98\x01\x08\x10\n\r\n\x05\x04\x0f\x02\x03\x05\
    \x12\x04\x98\x01\x11\x17\n\r\n\x05\x04\x0f\x02\x03\x01\x12\x04\x98\x01\
    \x18!\n\r\n\x05\x04\x0f\x02\x03\x03\x12\x04\x98\x01$%\n\x0c\n\x04\x04\
    \x0f\x02\x04\x12\x04\x99\x01\x08\x19\n\x0f\n\x05\x04\x0f\x02\x04\x04\x12\
    \x06\x99\x01\x08\x98\x01&\n\r\n\x05\x04\x0f\x02\x04\x05\x12\x04\x99\x01\
    \x08\x0c\n\r\n\x05\x04\x0f\x02\x04\x01\x12\x04\x99\x01\r\x14\n\r\n\x05\
    \x04\x0f\x02\x04\x03\x12\x04\x99\x01\x17\x18\n\x0c\n\x04\x04\x0f\x02\x05\
    \x12\x04\x9a\x01\x08\x17\n\x0f\n\x05\x04\x0f\x02\x05\x04\x12\x06\x9a\x01\
    \x08\x99\x01\x19\n\r\n\x05\x04\x0f\x02\x05\x05\x12\x04\x9a\x01\x08\x0c\n\
    \r\n\x05\x04\x0f\x02\x05\x01\x12\x04\x9a\x01\r\x12\n\r\n\x05\x04\x0f\x02\
    \x05\x03\x12\x04\x9a\x01\x15\x16\n\x0c\n\x04\x04\x0f\x02\x06\x12\x04\x9b\
    \x01\x08\x16\n\x0f\n\x05\x04\x0f\x02\x06\x04\x12\x06\x9b\x01\x08\x9a\x01\
    \x17\n\r\n\x05\x04\x0f\x02\x06\x05\x12\x04\x9b\x01\x08\x0c\n\r\n\x05\x04\
    \x0f\x02\x06\x01\x12\x04\x9b\x01\r\x11\n\r\n\x05\x04\x0f\x02\x06\x03\x12\
    \x04\x9b\x01\x14\x15b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
