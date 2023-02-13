use crate::line_parser::line_contains_else;

pub struct StackElement {
    pub line : usize,
    pub string : String
}

impl StackElement {
    pub fn new( line : usize, string : & String ) -> StackElement {
        StackElement { line : line, string : string.clone() }
    }

    pub fn is_else( & self ) -> bool {
        line_contains_else( & self.string )
    }
}


