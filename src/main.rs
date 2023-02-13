use clap::{arg, command, Command, value_parser};

use crate::line_parser::read_and_parse_source_file;

mod stack;
mod stack_element;
mod line_parser;

fn cli() -> Command {
    command!()
        .arg( arg!( -i --input <FILE> "The source file to analyze." )
                .required( true ) )
        .arg( arg!( -l --line <INTEGER> "The line the ifdef report is generated for." )
                .required( true )
                .value_parser( value_parser!( usize ) ) )
}

fn main() {
    let matches = cli().get_matches();
    let filename = matches.get_one::< String >( "input" ).expect( "required" );
    let line_number : usize = * matches.get_one::< usize >( "line" ).expect( "required" );
    let stack = match read_and_parse_source_file( filename, line_number ) {
        Err( err ) => panic!( "error: {}", err ),
        Ok( stack ) => stack,
    };
    print!( "{}", stack.to_string() );
}
