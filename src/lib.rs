//! # ttrackr
//!
//! `ttrackr` is a command line app for tracking time spent on working on tasks.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod cli;
pub mod config;
pub mod db;
pub mod utils;
