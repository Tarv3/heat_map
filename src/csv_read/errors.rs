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