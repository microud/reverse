use reverse_storage::application::AppStorage;
use anyhow::Result;

struct Local {
    command: String,
    args: Vec<String>,
    labels: Vec<String>,
}

enum PrivateKey {
    Path(String),
    Cert(String),
}

struct SSH {
    host: String,
    port: u32,
    username: String,
    password: String,
    private_key: PrivateKey,
    labels: Vec<String>
}

enum Terminal {
    LocalTerminal(Local),
    SSHTerminal(SSH),
}

pub struct TerminalManagement<'a> {
    storage: &'a AppStorage,
}

impl<'a> TerminalManagement<'a> {
    pub fn new(storage: &'a AppStorage) -> Self {
        TerminalManagement {
            storage
        }
    }

    pub async fn list_terminals(&self) -> Result<Vec<Terminal>> {
        todo!("list all terminals in storage")
    }

    pub async fn add_terminal(&self, terminal: Terminal) -> Result<()> {
        todo!("add terminal")
    }

    pub async fn update_terminal(&self, terminal: Terminal) -> Result<()> {
        todo!("update terminal")
    }

    pub async fn delete_terminal(&self, terminal: Terminal) -> Result<()> {
        todo!("delete_terminal")
    }
}