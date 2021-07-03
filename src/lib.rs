#![feature(once_cell)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

mod ast_parser;
mod declaration_analysis;
mod dfs;
mod field_collector;
mod sign_lattice;
mod term;
mod type_analysis;
mod union_find;
