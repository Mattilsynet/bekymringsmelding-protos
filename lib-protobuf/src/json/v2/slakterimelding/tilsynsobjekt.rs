use serde::{Deserialize, Serialize};

use crate::protobuf::v2::virksomhet::tilsynsobjekt::{
    Adresse as ProtoAdresse, Tilsynsobjekt as ProtoTilsynsobjekt,
};

pub const AVDELING_LANDDYR: &str = "M42200";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tilsynsobjekt {
    pub aktivitet_id: String,
    pub adresse: Adresse,
    pub mt_enhet: Option<String>,
    pub navn: Option<String>,
    #[serde(rename = "orgNr")]
    pub orgnr: String,
    pub tilsynsobjekt_id: String,
    #[serde(rename = "virksomhetsNavn")]
    pub virksomhetsnavn: Option<String>,
    pub produsentnummer: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Adresse {
    pub gateadresse: Option<String>,
    #[serde(rename(deserialize = "postNr"))]
    pub post_nr: Option<String>,
    pub poststed: Option<String>,
}

impl From<Tilsynsobjekt> for ProtoTilsynsobjekt {
    fn from(value: Tilsynsobjekt) -> Self {
        ProtoTilsynsobjekt {
            adresse: Some(ProtoAdresse::from(value.adresse)),
            mt_enhet: value.mt_enhet.unwrap_or(AVDELING_LANDDYR.to_string()),
            orgnr: value.orgnr,
            produsentnummer: value.produsentnummer,
            tilsynsobjekt_id: value.tilsynsobjekt_id,
            tilsynsobjekt_navn: value.navn,
            virksomhetsnavn: value.virksomhetsnavn,
            aktivitet_id: value.aktivitet_id,
        }
    }
}

impl From<Adresse> for ProtoAdresse {
    fn from(value: Adresse) -> Self {
        Self {
            gateadresse: value.gateadresse,
            postnummer: value.post_nr,
            poststed: value.poststed,
        }
    }
}
