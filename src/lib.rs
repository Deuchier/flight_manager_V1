#![allow(unused_imports)] // TODO: clear this
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

pub(crate) mod domain;
pub(crate) mod foundation;

pub mod ui;
