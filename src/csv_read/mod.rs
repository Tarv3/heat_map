use self::errors::{HeaderAddErr, HeaderContainerErr, ColumnMissing};
use csv::{StringRecord, StringRecordsIter};
use std::cmp::PartialEq;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub mod errors;
pub mod read;

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub column: usize,
}
impl Header {
    pub fn new(name: String, column: usize) -> Self {
        Self { name, column }
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

#[derive(Clone, Debug)]
pub struct HeaderContainer {
    headers: Vec<Header>,
}

impl HeaderContainer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            headers: Vec::with_capacity(capacity),
        }
    }
    pub fn from_record<'a>(
        headers: &'a StringRecord,
        names: &[&'a str],
        reusable_vec: &mut Vec<String>,
    ) -> Result<HeaderContainer, HeaderContainerErr> {
        reusable_vec.clear();
        let mut container = HeaderContainer::with_capacity(names.len());
        for &name in names {
            if let Some(column) = header_contains(headers, name) {
                let header = Header::new(String::from(name), column);
                container.add_header(header).expect("Repeated header");
            } else {
                reusable_vec.push(String::from(name));
            }
        }
        if reusable_vec.is_empty() {
            return Ok(container);
        } else {
            return Err(HeaderContainerErr::new(
                reusable_vec.clone(),
                headers.clone(),
            ));
        }
    }
    pub fn add_header(&mut self, header: Header) -> Result<(), HeaderAddErr> {
        if self.headers.contains(&header) {
            Err(HeaderAddErr::new(header))
        } else {
            self.headers.push(header);
            Ok(())
        }
    }
    pub fn header_count(&self) -> usize {
        self.headers.len()
    }
    pub fn column_with_name(&self, name: &str) -> Result<usize, ColumnMissing> {
        for header in &self.headers {
            if header.name.as_str() == name {
                return Ok(header.column);
            }
        }
        Err(ColumnMissing::new(String::from(name)))
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

pub trait FromStringRecord: Sized {
    fn field_count() -> usize;
    fn correct_num_headers(headers: &HeaderContainer) -> bool {
        Self::field_count() == headers.header_count()
    }
    fn from_string_records<T: Read>(
        headers: &HeaderContainer,
        records: &mut StringRecordsIter<T>,
    ) -> Result<Vec<Self>, Box<Error>>;
}
