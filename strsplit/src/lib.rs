#![warn(rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;
    /* version 1 begin
    fn next(&mut self) -> Option<Self::Item> {
        // &mut &'a str ----- Option<&'a str>
        if let Some(ref mut remainder) = self.remainder {
            // if let Some(remainder) = &mut self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                // left without *  - &mut &'a str
                // right - &'a str
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                // https://doc.rust-lang.org/std/option/enum.Option.html#method.take
                // impl<T> Option<T> { fn take(&mut self) -> Option<T> }
                self.remainder.take()
            }
        } else {
            None
        }
    }
    version 1 end */

    fn next(&mut self) -> Option<Self::Item> {
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut
        // impl<T> Option<T> { fn as_mut(&mut self) -> Option<&mut T> }
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            // left without *  - &mut &'a str
            // right - &'a str
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            // https://doc.rust-lang.org/std/option/enum.Option.html#method.take
            // impl<T> Option<T> { fn take(&mut self) -> Option<T> }
            self.remainder.take()
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    let delim = format!("{}", c);
    StrSplit::new(s, &delim).next().expect("StrSplit should have at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello, world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    // let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    // assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
