use crate::protobuf::v1::hvittkjott::leveranse::Leveranse as protoLeveranse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Leveranse {
    pub leveranse_id: String,
    pub produksjon_type: String,
    pub hybrid: String,
}

impl From<Leveranse> for protoLeveranse {
    fn from(value: Leveranse) -> Self {
        protoLeveranse {
            leveranse_id: value.leveranse_id,
            produksjon_type: value.produksjon_type,
            hybrid: value.hybrid,
        }
    }
}
