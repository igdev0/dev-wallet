pub enum AccountType {
    Receiving,
    Spending,
}

pub struct Account {
    // "path" is a string that contains the path for the derived key e.g: m/44/0'/0'/0'/1
    path: String,
    index: u32,
    address: String,
}

impl Account {
    fn new() {}
    fn create_next(&self) -> Account {
        Account {
            address: "".to_string(),
            path: "".to_string(),
            index: 0,
        }
    }
}

pub struct AccountBuilder {
    path: Option<String>,
    index: u32,
    address: String,
}

impl AccountBuilder {
    pub fn build() -> Account {
        Account {
            address: "".to_string(),
            path: "".to_string(),
            index: 0,
        }
    }
}
