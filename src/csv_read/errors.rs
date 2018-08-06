use csv::StringRecord;
use csv_read::{Header, HeaderContainer};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct HeaderAddErr {
    header: Header,
}

impl HeaderAddErr {
    pub fn new(header: Header) -> Self {
        Self { header }
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
pub struct HeaderContainerErr {
    missing_fields: Vec<String>,
    string_record: StringRecord,
}

impl HeaderContainerErr {
    pub fn new(missing_fields: Vec<String>, string_record: StringRecord) -> Self {
        Self {
            missing_fields,
            string_record,
        }
    }
}

impl Display for HeaderContainerErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "String record {:?} did not contain fields {:?}",
            self.string_record, self.missing_fields
        )
    }
}

impl Error for HeaderContainerErr {}

#[derive(Clone, Debug)]
pub struct ColumnMissing {
    col_name: String,
}

impl ColumnMissing {
    pub fn new(col_name: String) -> Self {
        Self { col_name }
    }
}

impl Display for ColumnMissing {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Csv was missing column {}", self.col_name)
    }
}

impl Error for ColumnMissing {}

#[derive(Clone, Debug)]
pub struct IncorrectColumnsErr {
    headers: HeaderContainer,
}

impl IncorrectColumnsErr {
    pub fn new(headers: HeaderContainer) -> Self {
        Self {
            headers
        }
    }
}

impl Display for IncorrectColumnsErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Header {:?} had an incorrect number of columns",
            self.headers
        )
    }
}

impl Error for IncorrectColumnsErr {}
