use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::json::v1::slakterimelding::avsender::Avsender;
use crate::json::v1::slakterimelding::kategori::Kategori;
use crate::json::v1::slakterimelding::slakteri::Slakteri;
use crate::json::v2::slakterimelding::funn::Funn;
use crate::json::v2::slakterimelding::tilsynsobjekt::Tilsynsobjekt;
use crate::protobuf;
use crate::protobuf::v1::person::ansatt::Ansatt;
use crate::protobuf::v1::rodtkjott::Kategori as ProtoKategori;
use crate::protobuf::v1::virksomhet::slakteri::Slakteri as ProtoSlakteri;
use crate::protobuf::v2::rodtkjott::Funn as ProtoFunn;
use crate::protobuf::v2::virksomhet::tilsynsobjekt::Tilsynsobjekt as ProtoTilsynsobjekt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rodtkjott {
    pub melding_id: Uuid,
    pub(crate) avsender: Option<Avsender>,
    pub(crate) begrunnelse: String,
    pub(crate) funn: Vec<Option<Funn>>,
    pub(crate) kategorier: Vec<Kategori>,
    pub(crate) slakteri: Option<Slakteri>,
    pub(crate) tilsynsobjekt: Option<Tilsynsobjekt>,
}

impl From<Rodtkjott> for protobuf::v2::rodtkjott::Rodtkjott {
    fn from(value: Rodtkjott) -> Self {
        protobuf::v2::rodtkjott::Rodtkjott {
            avsender: value.avsender.map(|a| Ansatt::from(a)),
            begrunnelse_for_bekymring: value.begrunnelse,
            funn: value
                .funn
                .into_iter()
                .filter_map(|opt_funn| opt_funn.map(ProtoFunn::from))
                .collect(),
            kategorier: value
                .kategorier
                .into_iter()
                .map(|k| {
                    let kategori = ProtoKategori::from(k);
                    i32::from(kategori)
                })
                .collect(),
            slakteri: value.slakteri.map(ProtoSlakteri::from),
            tilsynsobjekt: value.tilsynsobjekt.map(ProtoTilsynsobjekt::from),
            innsendt: Utc::now().to_string(),
        }
    }
}
