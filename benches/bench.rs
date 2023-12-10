#![feature(test)]
#![allow(clippy::struct_excessive_bools, clippy::struct_field_names)]

extern crate test;

mod twitter;

use crate::twitter::Twitter;
use std::fs;
use test::Bencher;

#[bench]
fn serialize_twitter_monomorphized(b: &mut Bencher) {
    let bytes = fs::read("benches/twitter.json").unwrap();
    let value: Twitter = serde_json::from_slice(&bytes).unwrap();
    let mut buf = Vec::with_capacity(bytes.len());
    b.iter(|| {
        buf.clear();
        let mut serializer = serde_json::Serializer::new(&mut buf);
        serde::Serialize::serialize(&value, &mut serializer).unwrap();
    });
}

#[bench]
fn serialize_twitter_erased(b: &mut Bencher) {
    let bytes = fs::read("benches/twitter.json").unwrap();
    let value: Twitter = serde_json::from_slice(&bytes).unwrap();
    let mut buf = Vec::with_capacity(bytes.len());
    b.iter(|| {
        buf.clear();
        let erased_value = &value as &dyn erased_serde::Serialize;
        let mut serializer = serde_json::Serializer::new(&mut buf);
        let mut erased_serializer = <dyn erased_serde::Serializer>::erase(&mut serializer);
        erased_value
            .erased_serialize(&mut erased_serializer)
            .unwrap();
    });
}

#[bench]
fn twitter_to_json_value_monomorphized(b: &mut Bencher) {
    let bytes = fs::read("benches/twitter.json").unwrap();
    let value: Twitter = serde_json::from_slice(&bytes).unwrap();
    b.iter(|| -> serde_json::Value {
        let serializer = serde_json::value::Serializer;
        serde::Serialize::serialize(&value, serializer).unwrap()
    });
}

#[bench]
fn twitter_to_json_value_erased(b: &mut Bencher) {
    let bytes = fs::read("benches/twitter.json").unwrap();
    let value: Twitter = serde_json::from_slice(&bytes).unwrap();
    b.iter(|| -> serde_json::Value {
        let erased_value = &value as &dyn erased_serde::Serialize;
        let serializer = serde_json::value::Serializer;
        erased_serde::serialize(erased_value, serializer).unwrap()
    });
}
