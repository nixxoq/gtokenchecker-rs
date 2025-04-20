use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    #[serde(rename = "type")]
    pub connection_type: String,
    pub name: String,
    pub visibility: u8,
    pub verified: bool,
    pub revoked: bool,
}

impl Connection {
    pub fn show(&self, index: usize, all_connections: usize) {
        println!(
            "
Connection #{} of {}

Connection type: {}
Name: {}
Visible: {}
Verified: {}
Revoked: {}
",
            index,
            all_connections,
            self.connection_type,
            self.name,
            self.visibility != 0,
            self.verified,
            self.revoked
        )
    }
}
