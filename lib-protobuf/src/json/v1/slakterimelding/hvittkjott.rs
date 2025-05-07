use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::json::v1::slakterimelding::avsender::Avsender;
use crate::json::v2::slakterimelding::slakteri::Slakteri;
use crate::json::v2::slakterimelding::tilsynsobjekt::Tilsynsobjekt;
use crate::protobuf::v1::hvittkjott::hvittkott::Hvittkjott as protoHvittkjott;
use crate::protobuf::v1::person::ansatt::Ansatt;
use crate::protobuf::v1::virksomhet::slakteri::Slakteri as ProtoSlakteri;
use crate::protobuf::v2::virksomhet::tilsynsobjekt::Tilsynsobjekt as ProtoTilsynsobjekt;

use super::leveranse::Leveranse;
use crate::protobuf::v1::hvittkjott::leveranse::Leveranse as protoLeveranse;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hvittkjott {
    pub melding_id: Uuid,
    pub avsender: Avsender,
    pub leveranse: Leveranse,
    pub begrunnelse: String,
    pub slakteri: Slakteri,
    pub tilsynsobjekt: Tilsynsobjekt,
}

impl From<Hvittkjott> for protoHvittkjott {
    fn from(value: Hvittkjott) -> Self {
        protoHvittkjott {
            avsender: Some(Ansatt::from(value.avsender)),
            begrunnelse_for_bekymring: value.begrunnelse,
            slakteri: Some(ProtoSlakteri::from(value.slakteri)),
            tilsynsobjekt: Some(ProtoTilsynsobjekt::from(value.tilsynsobjekt)),
            leveranse: Some(protoLeveranse::from(value.leveranse)),
            innsendt: Utc::now().to_string(),
        }
    }
}
