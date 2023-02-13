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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_else() {
        let stack_element = StackElement::new( 5, & "\t #else foobar".to_string() );
        assert!( stack_element.is_else() );
    }

    #[test]
    fn is_not_else() {
        let stack_element = StackElement::new( 5, & "\t #if foobar".to_string() );
        assert!( ! stack_element.is_else() );
    }
}

