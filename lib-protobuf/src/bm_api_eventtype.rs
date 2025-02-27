use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum BmApiEventType {
    AdminJobRepopulateDb,
    Unsupported(String),
}

const BM_API_REPOPULATE_DB: &str = "no.mattilsynet.lib-schemas.protos.RepopulateDb";

impl BmApiEventType {
    pub fn as_str(&self) -> &str {
        match self {
            BmApiEventType::AdminJobRepopulateDb => BM_API_REPOPULATE_DB,
            BmApiEventType::Unsupported(_) => "Unsupported",
        }
    }
}

impl From<&str> for BmApiEventType {
    fn from(event_ty: &str) -> Self {
        match event_ty {
            BM_API_REPOPULATE_DB => BmApiEventType::AdminJobRepopulateDb,
            other_str => BmApiEventType::Unsupported(other_str.to_string()),
        }
    }
}
