#![deny(clippy::all)]

//use std::env;
use napi_derive::napi;
//use librespot::core::{authentication::Credentials, config::SessionConfig, session::Session};
//use serde::Serialize;
//use serde_json::to_string;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}