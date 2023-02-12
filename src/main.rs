use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn main() {
    let filename = "tests/test.c";
    let line_number : usize = 12;
    let stack = match read_and_parse_source_file( filename, line_number ) {
        Err( err ) => panic!( "error: {}", err ),
        Ok( stack ) => stack,
    };
    print!( "{}", stack.to_string() );
}

struct Stack {
    entries : Vec< StackElement >,
    max_line_number : usize
}

impl Stack {
    pub fn new() -> Stack {
        Stack { entries : Vec::new(), max_line_number : 0 }
    }

    pub fn push( & mut self, stack_element : StackElement ) {
        self.max_line_number = std::cmp::max( stack_element.line,
                self.max_line_number );
        self.entries.push( stack_element );
    }

    pub fn pop( & mut self ) {
        while self.entries.last().unwrap().is_else() {
            self.entries.pop();
        }
        self.entries.pop();
    }

    pub fn to_string( & self ) -> String {
        let mut string = String::new();
        let width = f64::ceil( f64::log10( self.max_line_number as f64 ) );
        for entry in & self.entries {
            string.push_str( & format!( "{:width$}: {}\n",
                    entry.line, entry.string, width = width as usize ) );
        }
        return string;
    }
}

struct StackElement {
    line : usize,
    string : String
}

impl StackElement {
    pub fn new( line : usize, string : & String ) -> StackElement {
        StackElement { line : line, string : string.clone() }
    }

    pub fn is_else( & self ) -> bool {
        line_contains_else( & self.string )
    }
}

fn read_and_parse_source_file( filename : & str, line_number : usize )
        -> Result< Stack, std::io::Error > {
    let file = File::open( filename )?;
    let reader = BufReader::new( file );
    let mut stack = Stack::new();
    let mut current_line_number : usize = 1;
    for line in reader.lines() {
        let line_string = match line {
            Ok( s ) => s,
            _ => String::new()
        };
        let stack_element = StackElement::new( current_line_number,
                & line_string );

        if current_line_number == line_number {
            stack.push( stack_element );
            break;
        }
        else if line_contains_ifdef( & line_string ) {
            stack.push( stack_element );
        }
        else if line_contains_else( & line_string ) {
            stack.push( stack_element );
        }
        else if line_contains_endif( & line_string ) {
            stack.pop();
        }
        current_line_number += 1;
    }
    return Ok( stack );
}

fn line_contains_ifdef( line : & str ) -> bool {
    let re = Regex::new(r"^\s*#if.*").unwrap();
    re.is_match( line )
}

fn line_contains_else( line : & str ) -> bool {
    let re = Regex::new(r"^\s*#else.*").unwrap();
    re.is_match( line )
}

fn line_contains_endif( line : & str ) -> bool {
    let re = Regex::new(r"^\s*#endif.*").unwrap();
    re.is_match( line )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_ifdef() {
        assert!( line_contains_ifdef( " #ifdef foobar" ) );
    }

    #[test]
    fn contains_else() {
        assert!( line_contains_else( "#else foobar" ) );
    }

    #[test]
    fn contains_endif() {
        assert!( line_contains_endif( " \t#endif foobar" ) );
    }

    const TEST_C : &'static str = "tests/test.c";

    #[test]
    fn test_file_line_1() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 1 )?;
        assert_eq!( stack.to_string(), "1: // line 1\n" );
        Ok(())
    }

    #[test]
    fn test_file_line_3() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 3 )?;
        let string = stack.to_string();
        assert_eq!( string, "2: #ifdef def0\n3: // line 3\n" );
        Ok(())
    }

    #[test]
    fn test_file_line_5() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 5 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!( "2: #ifdef def0\n",
                    "4: #ifdef def1\n", "5: // line 5\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_7() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 7 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!( "2: #ifdef def0\n",
                    "4: #ifdef def1\n",
                    "6: #ifdef def2\n",
                    "7: // line 7\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_9() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 9 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!( "2: #ifdef def0\n",
                    "4: #ifdef def1\n",
                    "6: #ifdef def2\n",
                    "8: #ifdef def3\n",
                    "9: // line 9\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_11() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 11 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    " 2: #ifdef def0\n",
                    " 4: #ifdef def1\n",
                    " 6: #ifdef def2\n",
                    " 8: #ifdef def3\n",
                    "10: #else\n",
                    "11: // line 11\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_13() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 13 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    " 2: #ifdef def0\n",
                    " 4: #ifdef def1\n",
                    " 6: #ifdef def2\n",
                    " 8: #ifdef def3\n",
                    "10: #else\n",
                    "12: #ifdef def4\n",
                    "13: // line 13\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_15() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 15 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    " 2: #ifdef def0\n",
                    " 4: #ifdef def1\n",
                    " 6: #ifdef def2\n",
                    " 8: #ifdef def3\n",
                    "10: #else\n",
                    "15: // line 15\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_17() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 17 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    " 2: #ifdef def0\n",
                    " 4: #ifdef def1\n",
                    " 6: #ifdef def2\n",
                    "17: // line 17\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_19() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 19 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    " 2: #ifdef def0\n",
                    " 4: #ifdef def1\n",
                    " 6: #ifdef def2\n",
                    "18: #else\n",
                    "19: // line 19\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_21() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 21 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    " 2: #ifdef def0\n",
                    " 4: #ifdef def1\n",
                    "21: // line 21\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_23() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 23 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    " 2: #ifdef def0\n",
                    " 4: #ifdef def1\n",
                    "22: #else\n",
                    "23: // line 23\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_25() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 25 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    " 2: #ifdef def0\n",
                    "25: // line 25\n" ) );
        Ok(())
    }

    #[test]
    fn test_file_line_27() -> Result< (), std::io::Error > {
        let stack = read_and_parse_source_file( TEST_C, 27 )?;
        let string = stack.to_string();
        assert_eq!( string,
                concat!(
                    "27: // line 27\n" ) );
        Ok(())
    }
}
