pub enum RegularExpression {
    CharBegin,
    CharEnd,
    CharBetween,
    CharNotBetween,
    CharAny
}


pub struct SpecialCharacters {
    pub regex_type : RegularExpression,
    pub string : String
}

pub struct QueryDetails {
    pub query: String,
    pub file_path: String,
    pub regex: Option<SpecialCharacters>,
}

impl QueryDetails {
    pub fn data_cleaning(&mut self) {
        self.query = String::from(self.query.trim());
        self.file_path = String::from(self.file_path.trim());
    }

    pub fn regex_extraction(&mut self) -> Option<QueryDetails> {
        None
    }

}


