use crate::protobuf::v1::hvittkjott::leveranse::Leveranse as protoLeveranse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Leveranse {
    pub leveranse_id: String,
    pub produksjonstype: String,
    pub hybrid: Option<String>,
    pub bilde_ider: Option<Vec<String>>,
}

impl From<Leveranse> for protoLeveranse {
    fn from(value: Leveranse) -> Self {
        protoLeveranse {
            leveranse_id: value.leveranse_id,
            produksjonstype: value.produksjonstype,
            hybrid: value.hybrid,
            bilde_ider: value.bilde_ider.unwrap_or_default(),
        }
    }
}
