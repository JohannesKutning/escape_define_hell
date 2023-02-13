use crate::stack_element::StackElement;

pub struct Stack {
    pub entries : Vec< StackElement >,
    pub max_line_number : usize
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
