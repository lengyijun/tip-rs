extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

mod ast_parser;
mod declaration_analysis;
mod dfs;
mod term;
mod union_find;
