// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct PublicImportMessage {
    // message fields
    e: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublicImportMessage {}

impl PublicImportMessage {
    pub fn new() -> PublicImportMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublicImportMessage {
        static mut instance: ::protobuf::lazy::Lazy<PublicImportMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublicImportMessage,
        };
        unsafe {
            instance.get(|| {
                PublicImportMessage {
                    e: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 e = 1;

    pub fn clear_e(&mut self) {
        self.e = ::std::option::Option::None;
    }

    pub fn has_e(&self) -> bool {
        self.e.is_some()
    }

    // Param is passed by value, moved
    pub fn set_e(&mut self, v: i32) {
        self.e = ::std::option::Option::Some(v);
    }

    pub fn get_e(&self) -> i32 {
        self.e.unwrap_or(0)
    }
}

impl ::protobuf::Message for PublicImportMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.e = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.e.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.e {
            try!(os.write_int32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PublicImportMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublicImportMessage {
    fn new() -> PublicImportMessage {
        PublicImportMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublicImportMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "e",
                    PublicImportMessage::has_e,
                    PublicImportMessage::get_e,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublicImportMessage>(
                    "PublicImportMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublicImportMessage {
    fn clear(&mut self) {
        self.clear_e();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublicImportMessage {
    fn eq(&self, other: &PublicImportMessage) -> bool {
        self.e == other.e &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublicImportMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x2c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2f, 0x75, 0x6e, 0x69, 0x74, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x6d, 0x70, 0x6f, 0x72,
    0x74, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x18,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x5f, 0x75, 0x6e, 0x69, 0x74, 0x74, 0x65, 0x73,
    0x74, 0x5f, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x22, 0x23, 0x0a, 0x13, 0x50, 0x75, 0x62, 0x6c,
    0x69, 0x63, 0x49, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x0c, 0x0a, 0x01, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x01, 0x65, 0x42, 0x1a, 0x0a,
    0x18, 0x63, 0x6f, 0x6d, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x74, 0x65, 0x73, 0x74, 0x4a, 0xba, 0x0e, 0x0a, 0x06, 0x12, 0x04,
    0x20, 0x00, 0x28, 0x01, 0x0a, 0xf6, 0x0c, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x20, 0x00, 0x12, 0x32,
    0xc1, 0x0c, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x42, 0x75, 0x66, 0x66,
    0x65, 0x72, 0x73, 0x20, 0x2d, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x27, 0x73, 0x20, 0x64,
    0x61, 0x74, 0x61, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x20,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x0a, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68,
    0x74, 0x20, 0x32, 0x30, 0x30, 0x38, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x49, 0x6e,
    0x63, 0x2e, 0x20, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x72, 0x69, 0x67, 0x68, 0x74, 0x73, 0x20, 0x72,
    0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a,
    0x2f, 0x2f, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x2d, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x73, 0x2f, 0x0a, 0x0a, 0x20, 0x52, 0x65, 0x64, 0x69,
    0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x62, 0x69, 0x6e, 0x61, 0x72, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x73, 0x2c, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x6f, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x0a, 0x20,
    0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x76,
    0x69, 0x64, 0x65, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f,
    0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x6d, 0x65, 0x74, 0x3a, 0x0a, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x2a, 0x20, 0x52, 0x65, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x63,
    0x6f, 0x64, 0x65, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x72, 0x65, 0x74, 0x61, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x76, 0x65, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x72, 0x69,
    0x67, 0x68, 0x74, 0x0a, 0x20, 0x6e, 0x6f, 0x74, 0x69, 0x63, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c,
    0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x64, 0x69, 0x73, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x65,
    0x72, 0x2e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2a, 0x20, 0x52, 0x65, 0x64, 0x69, 0x73, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x69, 0x6e,
    0x61, 0x72, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x72, 0x65,
    0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x76,
    0x65, 0x0a, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6e, 0x6f, 0x74,
    0x69, 0x63, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x64,
    0x69, 0x73, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x65, 0x72, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x6e, 0x64, 0x2f, 0x6f, 0x72, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x74,
    0x65, 0x72, 0x69, 0x61, 0x6c, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20,
    0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2a, 0x20, 0x4e,
    0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x49, 0x6e, 0x63, 0x2e, 0x20, 0x6e,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20,
    0x69, 0x74, 0x73, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x6f, 0x72,
    0x73, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x65, 0x6e, 0x64, 0x6f, 0x72, 0x73, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x70, 0x72, 0x6f, 0x6d,
    0x6f, 0x74, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x73, 0x20, 0x64, 0x65, 0x72,
    0x69, 0x76, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x0a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x20,
    0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6e, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x20, 0x54, 0x48, 0x49, 0x53, 0x20, 0x53, 0x4f, 0x46, 0x54, 0x57,
    0x41, 0x52, 0x45, 0x20, 0x49, 0x53, 0x20, 0x50, 0x52, 0x4f, 0x56, 0x49, 0x44, 0x45, 0x44, 0x20,
    0x42, 0x59, 0x20, 0x54, 0x48, 0x45, 0x20, 0x43, 0x4f, 0x50, 0x59, 0x52, 0x49, 0x47, 0x48, 0x54,
    0x20, 0x48, 0x4f, 0x4c, 0x44, 0x45, 0x52, 0x53, 0x20, 0x41, 0x4e, 0x44, 0x20, 0x43, 0x4f, 0x4e,
    0x54, 0x52, 0x49, 0x42, 0x55, 0x54, 0x4f, 0x52, 0x53, 0x0a, 0x20, 0x22, 0x41, 0x53, 0x20, 0x49,
    0x53, 0x22, 0x20, 0x41, 0x4e, 0x44, 0x20, 0x41, 0x4e, 0x59, 0x20, 0x45, 0x58, 0x50, 0x52, 0x45,
    0x53, 0x53, 0x20, 0x4f, 0x52, 0x20, 0x49, 0x4d, 0x50, 0x4c, 0x49, 0x45, 0x44, 0x20, 0x57, 0x41,
    0x52, 0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x2c, 0x20, 0x49, 0x4e, 0x43, 0x4c, 0x55, 0x44,
    0x49, 0x4e, 0x47, 0x2c, 0x20, 0x42, 0x55, 0x54, 0x20, 0x4e, 0x4f, 0x54, 0x0a, 0x20, 0x4c, 0x49,
    0x4d, 0x49, 0x54, 0x45, 0x44, 0x20, 0x54, 0x4f, 0x2c, 0x20, 0x54, 0x48, 0x45, 0x20, 0x49, 0x4d,
    0x50, 0x4c, 0x49, 0x45, 0x44, 0x20, 0x57, 0x41, 0x52, 0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53,
    0x20, 0x4f, 0x46, 0x20, 0x4d, 0x45, 0x52, 0x43, 0x48, 0x41, 0x4e, 0x54, 0x41, 0x42, 0x49, 0x4c,
    0x49, 0x54, 0x59, 0x20, 0x41, 0x4e, 0x44, 0x20, 0x46, 0x49, 0x54, 0x4e, 0x45, 0x53, 0x53, 0x20,
    0x46, 0x4f, 0x52, 0x0a, 0x20, 0x41, 0x20, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x55, 0x4c, 0x41,
    0x52, 0x20, 0x50, 0x55, 0x52, 0x50, 0x4f, 0x53, 0x45, 0x20, 0x41, 0x52, 0x45, 0x20, 0x44, 0x49,
    0x53, 0x43, 0x4c, 0x41, 0x49, 0x4d, 0x45, 0x44, 0x2e, 0x20, 0x49, 0x4e, 0x20, 0x4e, 0x4f, 0x20,
    0x45, 0x56, 0x45, 0x4e, 0x54, 0x20, 0x53, 0x48, 0x41, 0x4c, 0x4c, 0x20, 0x54, 0x48, 0x45, 0x20,
    0x43, 0x4f, 0x50, 0x59, 0x52, 0x49, 0x47, 0x48, 0x54, 0x0a, 0x20, 0x4f, 0x57, 0x4e, 0x45, 0x52,
    0x20, 0x4f, 0x52, 0x20, 0x43, 0x4f, 0x4e, 0x54, 0x52, 0x49, 0x42, 0x55, 0x54, 0x4f, 0x52, 0x53,
    0x20, 0x42, 0x45, 0x20, 0x4c, 0x49, 0x41, 0x42, 0x4c, 0x45, 0x20, 0x46, 0x4f, 0x52, 0x20, 0x41,
    0x4e, 0x59, 0x20, 0x44, 0x49, 0x52, 0x45, 0x43, 0x54, 0x2c, 0x20, 0x49, 0x4e, 0x44, 0x49, 0x52,
    0x45, 0x43, 0x54, 0x2c, 0x20, 0x49, 0x4e, 0x43, 0x49, 0x44, 0x45, 0x4e, 0x54, 0x41, 0x4c, 0x2c,
    0x0a, 0x20, 0x53, 0x50, 0x45, 0x43, 0x49, 0x41, 0x4c, 0x2c, 0x20, 0x45, 0x58, 0x45, 0x4d, 0x50,
    0x4c, 0x41, 0x52, 0x59, 0x2c, 0x20, 0x4f, 0x52, 0x20, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x51, 0x55,
    0x45, 0x4e, 0x54, 0x49, 0x41, 0x4c, 0x20, 0x44, 0x41, 0x4d, 0x41, 0x47, 0x45, 0x53, 0x20, 0x28,
    0x49, 0x4e, 0x43, 0x4c, 0x55, 0x44, 0x49, 0x4e, 0x47, 0x2c, 0x20, 0x42, 0x55, 0x54, 0x20, 0x4e,
    0x4f, 0x54, 0x0a, 0x20, 0x4c, 0x49, 0x4d, 0x49, 0x54, 0x45, 0x44, 0x20, 0x54, 0x4f, 0x2c, 0x20,
    0x50, 0x52, 0x4f, 0x43, 0x55, 0x52, 0x45, 0x4d, 0x45, 0x4e, 0x54, 0x20, 0x4f, 0x46, 0x20, 0x53,
    0x55, 0x42, 0x53, 0x54, 0x49, 0x54, 0x55, 0x54, 0x45, 0x20, 0x47, 0x4f, 0x4f, 0x44, 0x53, 0x20,
    0x4f, 0x52, 0x20, 0x53, 0x45, 0x52, 0x56, 0x49, 0x43, 0x45, 0x53, 0x3b, 0x20, 0x4c, 0x4f, 0x53,
    0x53, 0x20, 0x4f, 0x46, 0x20, 0x55, 0x53, 0x45, 0x2c, 0x0a, 0x20, 0x44, 0x41, 0x54, 0x41, 0x2c,
    0x20, 0x4f, 0x52, 0x20, 0x50, 0x52, 0x4f, 0x46, 0x49, 0x54, 0x53, 0x3b, 0x20, 0x4f, 0x52, 0x20,
    0x42, 0x55, 0x53, 0x49, 0x4e, 0x45, 0x53, 0x53, 0x20, 0x49, 0x4e, 0x54, 0x45, 0x52, 0x52, 0x55,
    0x50, 0x54, 0x49, 0x4f, 0x4e, 0x29, 0x20, 0x48, 0x4f, 0x57, 0x45, 0x56, 0x45, 0x52, 0x20, 0x43,
    0x41, 0x55, 0x53, 0x45, 0x44, 0x20, 0x41, 0x4e, 0x44, 0x20, 0x4f, 0x4e, 0x20, 0x41, 0x4e, 0x59,
    0x0a, 0x20, 0x54, 0x48, 0x45, 0x4f, 0x52, 0x59, 0x20, 0x4f, 0x46, 0x20, 0x4c, 0x49, 0x41, 0x42,
    0x49, 0x4c, 0x49, 0x54, 0x59, 0x2c, 0x20, 0x57, 0x48, 0x45, 0x54, 0x48, 0x45, 0x52, 0x20, 0x49,
    0x4e, 0x20, 0x43, 0x4f, 0x4e, 0x54, 0x52, 0x41, 0x43, 0x54, 0x2c, 0x20, 0x53, 0x54, 0x52, 0x49,
    0x43, 0x54, 0x20, 0x4c, 0x49, 0x41, 0x42, 0x49, 0x4c, 0x49, 0x54, 0x59, 0x2c, 0x20, 0x4f, 0x52,
    0x20, 0x54, 0x4f, 0x52, 0x54, 0x0a, 0x20, 0x28, 0x49, 0x4e, 0x43, 0x4c, 0x55, 0x44, 0x49, 0x4e,
    0x47, 0x20, 0x4e, 0x45, 0x47, 0x4c, 0x49, 0x47, 0x45, 0x4e, 0x43, 0x45, 0x20, 0x4f, 0x52, 0x20,
    0x4f, 0x54, 0x48, 0x45, 0x52, 0x57, 0x49, 0x53, 0x45, 0x29, 0x20, 0x41, 0x52, 0x49, 0x53, 0x49,
    0x4e, 0x47, 0x20, 0x49, 0x4e, 0x20, 0x41, 0x4e, 0x59, 0x20, 0x57, 0x41, 0x59, 0x20, 0x4f, 0x55,
    0x54, 0x20, 0x4f, 0x46, 0x20, 0x54, 0x48, 0x45, 0x20, 0x55, 0x53, 0x45, 0x0a, 0x20, 0x4f, 0x46,
    0x20, 0x54, 0x48, 0x49, 0x53, 0x20, 0x53, 0x4f, 0x46, 0x54, 0x57, 0x41, 0x52, 0x45, 0x2c, 0x20,
    0x45, 0x56, 0x45, 0x4e, 0x20, 0x49, 0x46, 0x20, 0x41, 0x44, 0x56, 0x49, 0x53, 0x45, 0x44, 0x20,
    0x4f, 0x46, 0x20, 0x54, 0x48, 0x45, 0x20, 0x50, 0x4f, 0x53, 0x53, 0x49, 0x42, 0x49, 0x4c, 0x49,
    0x54, 0x59, 0x20, 0x4f, 0x46, 0x20, 0x53, 0x55, 0x43, 0x48, 0x20, 0x44, 0x41, 0x4d, 0x41, 0x47,
    0x45, 0x2e, 0x0a, 0x32, 0x28, 0x20, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x3a, 0x20, 0x6c, 0x69,
    0x75, 0x6a, 0x69, 0x73, 0x69, 0x40, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d,
    0x20, 0x28, 0x50, 0x68, 0x65, 0x72, 0x6c, 0x20, 0x4c, 0x69, 0x75, 0x29, 0x0a, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x22, 0x08, 0x20, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x24, 0x00,
    0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x24, 0x00, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x24, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x24, 0x16, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x26, 0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x26, 0x08,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x27, 0x15, 0x16,
];

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
