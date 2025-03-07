use serde::{Deserialize, Serialize};

use crate::protobuf::v1::virksomhet::tilsynsobjekt::{
    Adresse as ProtoAdresse, Tilsynsobjekt as ProtoTilsynsobjekt,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tilsynsobjekt {
    pub adresse: Adresse,
    pub identiteter: Vec<Identitet>,
    pub mt_enhet: String,
    pub navn: Option<String>,
    #[serde(rename = "orgNr")]
    pub orgnr: Option<String>,
    pub tilsynsobjekt_id: String,
    #[serde(rename = "virksomhetsNavn")]
    pub virksomhetsnavn: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Adresse {
    pub gateadresse: Option<String>,
    #[serde(rename(deserialize = "postNr"))]
    pub post_nr: Option<String>,
    pub poststed: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identitet {
    pub identitetstype: Identitetstype,
    pub verdi: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identitetstype {
    pub id: String,
    pub beskrivelse: String,
}

impl From<Tilsynsobjekt> for ProtoTilsynsobjekt {
    fn from(value: Tilsynsobjekt) -> Self {
        let mut dyreholds_id = None;
        let mut produsentnummer = None;

        for identitet in value.identiteter {
            match identitet.identitetstype.id.as_str() {
                "IDENTITETSTYPE$DYREHOLDID" => dyreholds_id = Some(identitet.verdi),
                "IDENTITETSTYPE$PRODUSENTNUMMER" => produsentnummer = Some(identitet.verdi),
                _ => {}
            }
        }

        ProtoTilsynsobjekt {
            adresse: Some(ProtoAdresse::from(value.adresse)),
            dyreholds_id,
            mt_enhet: value.mt_enhet,
            orgnr: value.orgnr,
            produsentnummer,
            tilsynsobjekt_id: value.tilsynsobjekt_id,
            tilsynsobjekt_navn: value.navn,
            virksomhetsnavn: value.virksomhetsnavn,
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
