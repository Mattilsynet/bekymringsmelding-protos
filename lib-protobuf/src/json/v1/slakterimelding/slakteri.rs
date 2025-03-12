use serde::{Deserialize, Serialize};

use crate::protobuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Slakteri {
    pub eftanummer: u32,
    pub navn: String,
}

impl From<Slakteri> for protobuf::v1::virksomhet::slakteri::Slakteri {
    fn from(value: Slakteri) -> Self {
        Self {
            eftanummer: value.eftanummer.to_string(),
            navn: value.navn,
        }
    }
}
