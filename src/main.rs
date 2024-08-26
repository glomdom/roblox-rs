mod cli;
mod indent_manager;
mod transpiler;

use crate::cli::Cli;
use crate::indent_manager::IndentManager;
use crate::transpiler::LuauTranspiler;
use clap::Parser;
use syn::parse_file;
use syn::visit::Visit;

fn main() {
    let cli = Cli::parse();

    let file_contents = std::fs::read_to_string(&cli.file).expect("Failed to read file");
    let syntax_tree = parse_file(&file_contents).expect("Failed to parse file contents");

    
    let mut indent_manager = IndentManager::new("    ");
    let mut transpiler = LuauTranspiler::new(&mut indent_manager);
    transpiler.visit_file(&syntax_tree);
    
    // println!("{:#?}", syntax_tree);
    println!("{}", transpiler.render());
}
