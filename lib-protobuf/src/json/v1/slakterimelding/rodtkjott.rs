use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::json::v1::slakterimelding::avsender::Avsender;
use crate::json::v1::slakterimelding::funn::Funn;
use crate::json::v1::slakterimelding::kategori::Kategori;
use crate::json::v1::slakterimelding::slakteri::Slakteri;
use crate::json::v1::slakterimelding::tilsynsobjekt::Tilsynsobjekt;
use crate::protobuf;
use crate::protobuf::v1::person::ansatt::Ansatt;
use crate::protobuf::v1::rodtkjott::Funn as ProtoFunn;
use crate::protobuf::v1::rodtkjott::Kategori as ProtoKategori;
use crate::protobuf::v1::virksomhet::slakteri::Slakteri as ProtoSlakteri;
use crate::protobuf::v1::virksomhet::tilsynsobjekt::Tilsynsobjekt as ProtoTilsynsobjekt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rodtkjott {
    pub(crate) aktivitet_id: Option<String>,
    pub(crate) avsender: Option<Avsender>,
    pub(crate) avsender_epost: Option<String>,
    pub(crate) begrunnelse: String,
    pub(crate) eftanummer: String,
    pub(crate) funn: Vec<Option<Funn>>,
    pub(crate) innsendt_av: Option<String>,
    pub(crate) journalpost_id: Option<String>,
    pub(crate) kategorier: Vec<Kategori>,
    pub melding_id: Uuid,
    pub(crate) mottaker_id: Option<String>,
    pub(crate) produsent_tilsynsobjekt_id: String,
    pub(crate) saksnummer: Option<String>,
    pub(crate) sist_redigert: DateTime<Utc>,
    pub(crate) sist_redigert_av: String,
    pub(crate) slakteri: Option<Slakteri>,
    pub(crate) tilsynsobjekt: Option<Tilsynsobjekt>,
    // pub vurdering: Option<Vurdering>,
}

impl From<Rodtkjott> for protobuf::v1::rodtkjott::Rodtkjott {
    fn from(value: Rodtkjott) -> Self {
        protobuf::v1::rodtkjott::Rodtkjott {
            aktivitet_id: value.aktivitet_id.unwrap_or_default(),
            avsender: value.avsender.map(|a| {
                let mut avsender = Ansatt::from(a);
                avsender.avsender_epost = value.avsender_epost;
                avsender
            }),
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
            mottaker_id: value.mottaker_id,
            saksnummer: value.saksnummer,
            sist_redigert: value.sist_redigert.to_rfc3339().to_string(),
            sist_redigert_av: value.sist_redigert_av,
            slakteri: value.slakteri.map(ProtoSlakteri::from),
            tilsynsobjekt: value.tilsynsobjekt.map(ProtoTilsynsobjekt::from),
        }
    }
}
