use csv::{ReaderBuilder, Reader};
use csv::StringRecord;
use std::marker::PhantomData;
use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{Formatter, Display};
use std::fmt;

#[derive(Debug, Clone)]
pub struct HeaderAddErr {
    header: Header
}

impl HeaderAddErr {
    pub fn new(header: Header) -> Self {
        Self {
            header
        }
    }
}

impl Display for HeaderAddErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Header Add Error")
    }
}

impl Error for HeaderAddErr {
    fn description(&self) -> &str {
        "Tried to add header \"x\" when header container already contained \"x\""
    }
    fn cause(&self) -> Option<&Error> {
        Some(&self.header)
    }
}
#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub column: usize,
}
impl Header {
    pub fn new(name: String, column: usize) -> Self {
        Self {
            name,
            column,
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Header ( Name: {}, Column: {} )", self.name, self.column)
    }
}

impl Error for Header {
    fn description(&self) -> &str {
        "Data column header"
    }
}

impl PartialEq for Header {
    fn eq(&self, other: &Header) -> bool {
        self.name == other.name || self.column == other.column
    }
}

pub struct HeaderContainer(Vec<Header>);

impl HeaderContainer {
    pub fn add_header(&mut self, header: Header) -> Result<(), HeaderAddErr> {
        if self.0.contains(&header) {
            Err(HeaderAddErr::new(header))
        }
        else {
            self.0.push(header);
            Ok(())
        }
    }
}

pub fn header_contains(headers: &StringRecord, name: &str) -> Option<usize> {
    for (i, header) in headers.iter().enumerate() {
        if header == name {
            return Some(i);
        }
    }
    None
}

pub fn header_from_record(headers: &StringRecord, names: impl Iterator<Item = &str>) ->