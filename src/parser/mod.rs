// This include forces recompiling this source file if the grammar file changes.
const _GRAMMAR: &'static str = include_str!("tera.pest");


#[derive(Parser)]
#[grammar = "parser/tera.pest"]
pub struct TeraParser;

pub mod ast;


#[cfg(test)]
mod tests;
