use serde::ser::{Impossible, SerializeMap};
use serde::{Serialize, Serializer};
use std::cell::Cell;
use std::fmt::{self, Display};
use std::io::{self, Write};
use std::panic::{self, AssertUnwindSafe};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Collect<'a, T: ?Sized>(&'a T);

impl<T: ?Sized + Display> Serialize for Collect<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self.0)
    }
}

struct Chunks(Cell<usize>);

impl Display for Chunks {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in ["ab", "cd", "ef", "gh", "ij", "kl"] {
            self.0.set(self.0.get() + 1);
            formatter.write_str(chunk)?;
        }
        Ok(())
    }
}

struct BudgetWriter {
    remaining: usize,
    bytes: Vec<u8>,
}

impl BudgetWriter {
    fn new(remaining: usize) -> Self {
        Self {
            remaining,
            bytes: Vec::new(),
        }
    }
}

impl Write for BudgetWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "fixed output budget exhausted",
            ));
        }
        let written = self.remaining.min(bytes.len());
        self.bytes.extend_from_slice(&bytes[..written]);
        self.remaining -= written;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn json_direct<T: ?Sized + Serialize>(
    value: &T,
    writer: &mut BudgetWriter,
) -> serde_json::Result<()> {
    value.serialize(&mut serde_json::Serializer::new(writer))
}

fn json_erased<T: Serialize>(value: &T, writer: &mut BudgetWriter) -> serde_json::Result<()> {
    erased_serde::serialize(value, &mut serde_json::Serializer::new(writer))
}

fn check_json_error(error: &serde_json::Error) {
    assert_eq!(error.io_error_kind(), Some(io::ErrorKind::Other));
    assert_eq!(error.to_string(), "fixed output budget exhausted");
}

#[test]
fn collect_str_streaming_writer_error() {
    let chunks = Chunks(Cell::new(0));
    let value = Collect(&chunks);
    let mut direct = BudgetWriter::new(3);
    let direct_error = json_direct(&value, &mut direct).unwrap_err();
    let direct_work = chunks.0.replace(0);
    let mut erased = BudgetWriter::new(3);
    let erased_error = json_erased(&value, &mut erased).unwrap_err();
    let erased_work = chunks.0.get();
    assert_eq!(direct.bytes, b"\"ab");
    assert_eq!(erased.bytes, direct.bytes);
    check_json_error(&direct_error);
    check_json_error(&erased_error);
    assert_eq!(direct_work, 2);
    println!("matching partial bytes/errors; direct fragments={direct_work}, erased fragments={erased_work}");
    assert_eq!(
        erased_work, direct_work,
        "erased formatting work must match direct streaming"
    );
}

#[test]
fn collect_str_success_matches_direct() {
    let chunks = Chunks(Cell::new(0));
    let value = Collect(&chunks);
    let mut direct = BudgetWriter::new(usize::MAX);
    json_direct(&value, &mut direct).unwrap();
    assert_eq!(chunks.0.replace(0), 6);
    let mut erased = BudgetWriter::new(usize::MAX);
    json_erased(&value, &mut erased).unwrap();
    assert_eq!(chunks.0.get(), 6);
    assert_eq!(direct.bytes, b"\"abcdefghijkl\"");
    assert_eq!(erased.bytes, direct.bytes);
    let escaped = Collect("a\"\\\né");
    let direct = serde_json::to_string(&escaped).unwrap();
    let erased: &dyn erased_serde::Serialize = &escaped;
    assert_eq!(direct, "\"a\\\"\\\\\\né\"");
    assert_eq!(serde_json::to_string(erased).unwrap(), direct);
}

#[derive(Debug)]
struct TestError {
    marker: usize,
    message: &'static str,
}

impl Display for TestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for TestError {}

impl serde::ser::Error for TestError {
    fn custom<T: Display>(_: T) -> Self {
        Self {
            marker: 0,
            message: "custom error",
        }
    }
}

#[derive(Debug)]
struct Token<'a> {
    borrowed: &'a u8,
    text: String,
}

#[derive(Default)]
struct Calls {
    collect: AtomicUsize,
    string: AtomicUsize,
}

struct RecordingSerializer<'a> {
    calls: &'a Calls,
    borrowed: &'a u8,
    failure: Option<usize>,
    panic: bool,
}

impl<'a> RecordingSerializer<'a> {
    fn new(calls: &'a Calls, borrowed: &'a u8) -> Self {
        Self {
            calls,
            borrowed,
            failure: None,
            panic: false,
        }
    }

    fn finish(self, text: String) -> Result<Token<'a>, TestError> {
        if let Some(marker) = self.failure {
            return Err(TestError {
                marker,
                message: "controlled serializer failure",
            });
        }
        Ok(Token {
            borrowed: self.borrowed,
            text,
        })
    }
}

macro_rules! unsupported_scalar {
    ($($method:ident($value:ident: $ty:ty);)*) => {
        $(fn $method(self, _: $ty) -> Result<Self::Ok, Self::Error> {
            unreachable!()
        })*
    };
}

impl<'a> Serializer for RecordingSerializer<'a> {
    type Ok = Token<'a>;
    type Error = TestError;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    unsupported_scalar! {
        serialize_bool(v: bool);
        serialize_i8(v: i8);
        serialize_i16(v: i16);
        serialize_i32(v: i32);
        serialize_i64(v: i64);
        serialize_u8(v: u8);
        serialize_u16(v: u16);
        serialize_u32(v: u32);
        serialize_u64(v: u64);
        serialize_f32(v: f32);
        serialize_f64(v: f64);
        serialize_char(v: char);
        serialize_bytes(v: &[u8]);
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.calls.string.fetch_add(1, Ordering::Relaxed);
        self.finish(value.to_owned())
    }

    fn collect_str<T: ?Sized + Display>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        self.calls.collect.fetch_add(1, Ordering::Relaxed);
        assert!(!self.panic, "collect override panic");
        self.finish(value.to_string())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unreachable!()
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unreachable!()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unreachable!()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unreachable!()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unreachable!()
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        unreachable!()
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unreachable!()
    }
}

fn check_calls(calls: &Calls, collect: usize, string: usize) {
    assert_eq!(calls.collect.load(Ordering::Relaxed), collect);
    assert_eq!(calls.string.load(Ordering::Relaxed), string);
}

fn check_token(token: &Token<'_>, borrowed: *const u8, text: &str) {
    assert!(std::ptr::eq(token.borrowed, borrowed));
    assert_eq!(token.text, text);
}

#[test]
fn collect_str_custom_serializer() {
    let borrowed = 42;
    let calls = Calls::default();
    let value = Collect("formatted");
    let token = value
        .serialize(RecordingSerializer::new(&calls, &borrowed))
        .unwrap();
    check_token(&token, &borrowed, "formatted");
    check_calls(&calls, 1, 0);
    let calls = Calls::default();
    let token =
        erased_serde::serialize(&value, RecordingSerializer::new(&calls, &borrowed)).unwrap();
    check_token(&token, &borrowed, "formatted");
    check_calls(&calls, 1, 0);
}

#[test]
fn serialize_str_is_unchanged() {
    let calls = Calls::default();
    let borrowed = 42;
    let token =
        erased_serde::serialize(&"ordinary", RecordingSerializer::new(&calls, &borrowed)).unwrap();
    check_token(&token, &borrowed, "ordinary");
    check_calls(&calls, 0, 1);
}

#[test]
fn typed_errors_preserve_fields() {
    let calls = Calls::default();
    let borrowed = 42;
    let mut serializer = RecordingSerializer::new(&calls, &borrowed);
    serializer.failure = Some(739);
    let direct = Collect("text").serialize(serializer).unwrap_err();
    let mut serializer = RecordingSerializer::new(&calls, &borrowed);
    serializer.failure = Some(739);
    let erased = erased_serde::serialize(&Collect("text"), serializer).unwrap_err();
    assert_eq!(direct.marker, 739);
    assert_eq!(erased.marker, direct.marker);
    assert_eq!(erased.message, direct.message);
    assert_eq!(erased.to_string(), "controlled serializer failure");
    let mut serializer = RecordingSerializer::new(&calls, &borrowed);
    serializer.failure = Some(811);
    assert_eq!(
        erased_serde::serialize(&"ordinary", serializer)
            .unwrap_err()
            .marker,
        811
    );
}

#[test]
fn public_erased_entrypoint() {
    let chunks = Chunks(Cell::new(0));
    let value = Collect(&chunks);
    let mut writer = BudgetWriter::new(usize::MAX);
    let mut json = serde_json::Serializer::new(&mut writer);
    let mut serializer = <dyn erased_serde::Serializer>::erase(&mut json);
    let _: () = erased_serde::Serialize::erased_serialize(&value, &mut serializer).unwrap();
    assert_eq!(writer.bytes, b"\"abcdefghijkl\"");
    chunks.0.set(0);
    let mut writer = BudgetWriter::new(4);
    let mut json = serde_json::Serializer::new(&mut writer);
    let mut serializer = <dyn erased_serde::Serializer>::erase(&mut json);
    let error = erased_serde::Serialize::erased_serialize(&[value], &mut serializer).unwrap_err();
    assert_eq!(writer.bytes, b"[\"ab");
    assert_eq!(error.to_string(), "fixed output budget exhausted");
    assert_eq!(chunks.0.get(), 2);
}

struct BorrowedDisplay<'a> {
    text: &'a str,
    calls: Rc<Cell<usize>>,
}

impl Display for BorrowedDisplay<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.calls.set(self.calls.get() + 1);
        formatter.write_str(self.text)
    }
}

#[test]
fn borrowed_unsized_display() {
    let text = String::from("borrowed");
    let display = BorrowedDisplay {
        text: &text,
        calls: Rc::new(Cell::new(0)),
    };
    assert_eq!(
        serde_json::to_string(&Collect(&display) as &dyn erased_serde::Serialize).unwrap(),
        "\"borrowed\""
    );
    let unsized_value = Collect::<str>(&text);
    assert_eq!(
        serde_json::to_string(&unsized_value as &dyn erased_serde::Serialize).unwrap(),
        "\"borrowed\""
    );
    let object: &dyn Display = &display;
    assert_eq!(
        serde_json::to_string(&Collect(object) as &dyn erased_serde::Serialize).unwrap(),
        "\"borrowed\""
    );
    assert_eq!(
        serde_json::to_string(
            &Collect(&format_args!("{}:{}", text, 7)) as &dyn erased_serde::Serialize
        )
        .unwrap(),
        "\"borrowed:7\""
    );
    assert_eq!(display.calls.get(), 2);
}

macro_rules! value_object_test {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            let calls = Calls::default();
            let borrowed = 42;
            let value = Collect("object");
            let object: &$ty = &value;
            let token = object
                .serialize(RecordingSerializer::new(&calls, &borrowed))
                .unwrap();
            check_token(&token, &borrowed, "object");
            check_calls(&calls, 1, 0);
        }
    };
}

value_object_test!(value_plain, dyn erased_serde::Serialize);
value_object_test!(value_send, dyn erased_serde::Serialize + Send);
value_object_test!(value_sync, dyn erased_serde::Serialize + Sync);
value_object_test!(value_send_sync, dyn erased_serde::Serialize + Send + Sync);

macro_rules! serializer_wrapper_test {
    ($reference:ident, $boxed:ident, $ty:ty) => {
        #[test]
        fn $reference() {
            let calls = Calls::default();
            let borrowed = 42;
            let mut serializer =
                <dyn erased_serde::Serializer>::erase(RecordingSerializer::new(&calls, &borrowed));
            let mut wrapper: &mut $ty = &mut serializer;
            erased_serde::Serialize::erased_serialize(&Collect("wrapper"), &mut wrapper).unwrap();
            check_calls(&calls, 1, 0);
        }

        #[test]
        fn $boxed() {
            let calls = Calls::default();
            let borrowed = 42;
            let serializer =
                <dyn erased_serde::Serializer>::erase(RecordingSerializer::new(&calls, &borrowed));
            let mut wrapper: Box<$ty> = Box::new(serializer);
            erased_serde::Serialize::erased_serialize(&Collect("wrapper"), &mut wrapper).unwrap();
            check_calls(&calls, 1, 0);
        }
    };
}

serializer_wrapper_test!(
    serializer_plain_reference,
    serializer_plain_box,
    dyn erased_serde::Serializer + '_
);
serializer_wrapper_test!(
    serializer_send_reference,
    serializer_send_box,
    dyn erased_serde::Serializer + Send + '_
);
serializer_wrapper_test!(
    serializer_sync_reference,
    serializer_sync_box,
    dyn erased_serde::Serializer + Sync + '_
);
serializer_wrapper_test!(
    serializer_send_sync_reference,
    serializer_send_sync_box,
    dyn erased_serde::Serializer + Send + Sync + '_
);

struct KeyMap;

impl Serialize for KeyMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&Collect("key"), &7)?;
        map.end()
    }
}

#[test]
fn nested_collect_str_consumers() {
    #[derive(serde_derive::Serialize)]
    struct Nested<'a> {
        values: [Collect<'a, str>; 1],
        ordinary: &'a str,
        number: u8,
    }
    let value = Nested {
        values: [Collect("child")],
        ordinary: "plain",
        number: 7,
    };
    let direct = serde_json::to_string(&value).unwrap();
    assert_eq!(
        direct,
        r#"{"values":["child"],"ordinary":"plain","number":7}"#
    );
    assert_eq!(
        serde_json::to_string(&value as &dyn erased_serde::Serialize).unwrap(),
        direct
    );
    assert_eq!(
        serde_json::to_string(&KeyMap as &dyn erased_serde::Serialize).unwrap(),
        serde_json::to_string(&KeyMap).unwrap()
    );
    assert_eq!(serde_json::to_string(&KeyMap).unwrap(), r#"{"key":7}"#);
}

struct PanickingDisplay;

impl Display for PanickingDisplay {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("display panic")
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> &str {
    payload.downcast_ref::<&str>().unwrap()
}

#[test]
fn display_panic_propagates() {
    let direct =
        panic::catch_unwind(|| serde_json::to_string(&Collect(&PanickingDisplay))).unwrap_err();
    let erased = panic::catch_unwind(|| {
        serde_json::to_string(&Collect(&PanickingDisplay) as &dyn erased_serde::Serialize)
    })
    .unwrap_err();
    assert_eq!(panic_message(direct.as_ref()), "display panic");
    assert_eq!(panic_message(erased.as_ref()), "display panic");
}

#[test]
fn concrete_collect_panic_propagates() {
    let calls = Calls::default();
    let borrowed = 42;
    let direct = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut serializer = RecordingSerializer::new(&calls, &borrowed);
        serializer.panic = true;
        Collect("panic").serialize(serializer)
    }))
    .unwrap_err();
    let erased = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut serializer = RecordingSerializer::new(&calls, &borrowed);
        serializer.panic = true;
        erased_serde::serialize(&Collect("panic"), serializer)
    }))
    .unwrap_err();
    assert_eq!(panic_message(direct.as_ref()), "collect override panic");
    assert_eq!(panic_message(erased.as_ref()), "collect override panic");
}
