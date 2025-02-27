use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum MaskinvurderingEventType {
    AdminJobRevurderAlleMeldinger,
    Unsupported(String),
}

const BM_API_REVURDER_ALLE_MELDINGER: &str =
    "no.mattilsynet.lib-schemas.protos.RevurderAlleMeldinger";

impl MaskinvurderingEventType {
    pub fn as_str(&self) -> &str {
        match self {
            MaskinvurderingEventType::AdminJobRevurderAlleMeldinger => {
                BM_API_REVURDER_ALLE_MELDINGER
            }
            MaskinvurderingEventType::Unsupported(_) => "Unsupported",
        }
    }
}

impl From<&str> for MaskinvurderingEventType {
    fn from(event_ty: &str) -> Self {
        match event_ty {
            BM_API_REVURDER_ALLE_MELDINGER => {
                MaskinvurderingEventType::AdminJobRevurderAlleMeldinger
            }
            other_str => MaskinvurderingEventType::Unsupported(other_str.to_string()),
        }
    }
}
