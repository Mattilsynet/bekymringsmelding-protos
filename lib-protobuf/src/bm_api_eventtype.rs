use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum BmApiEventType {
    AdminJobRepopulateDb,
    AdminJobDeleteBm,
    Unsupported(String),
}

const BM_API_REPOPULATE_DB: &str = "no.mattilsynet.lib-schemas.protos.RepopulateDb";
const BM_API_DELETE_BM_ID: &str = "no.mattilsynet.lib-schemas.protos.DeleteBmId";

impl BmApiEventType {
    pub fn as_str(&self) -> &str {
        match self {
            BmApiEventType::AdminJobRepopulateDb => BM_API_REPOPULATE_DB,
            BmApiEventType::AdminJobDeleteBm => BM_API_DELETE_BM_ID,
            BmApiEventType::Unsupported(_) => "Unsupported",
        }
    }
}

impl From<&str> for BmApiEventType {
    fn from(event_ty: &str) -> Self {
        match event_ty {
            BM_API_REPOPULATE_DB => BmApiEventType::AdminJobRepopulateDb,
            BM_API_DELETE_BM_ID => BmApiEventType::AdminJobDeleteBm,
            other_str => BmApiEventType::Unsupported(other_str.to_string()),
        }
    }
}
