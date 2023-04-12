// Multiple lifetimes are a strange case in rust
pub struct StrSplit<'a,'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// Iterator trait that chops the string finding delimeters and returns the string until the delimeter  
impl<'a, 'b> Iterator for StrSplit<'a, 'b> {

    // Associated type related to return type of next() function (Iterator)
    // Lifetime tied to 'a not to 'b, therefore we can use delimiter outside  
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {

        // We want a mutable ref(&mut) to &str not moving the value itself from self.remainder
        let remainder = self.remainder.as_mut()? /* ? return the actual value inside Some() */;
        // as_mut() function: Returns a mutable ret(&mut or ref mut) to the value inside Option()
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[next_delim + self.delimiter.len()..];
            Some(until_delimiter)
        } else {

            // take() function: In a Option(), if there's Some() takes it and leave None, if None returns None 
            self.remainder.take()
        }
    }
}

// We need different lifetimes because in abscense of other lifetimes, rust compiler enforces the longest one to the rest
fn until_char(s: &str, c: char) -> &str {
    /*  &format!("{}", c) would have s lifetime and therefore constrainted by the function scope */
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}
#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
