use serde::{Deserialize, Serialize};

use crate::protobuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Avsender {
    #[serde(rename(deserialize = "avdelingId"))]
    pub avdeling_id: Option<String>,
    pub brukernavn: String,
    pub navn: String,
    #[serde(rename(deserialize = "regionId"))]
    pub region_id: Option<String>,
    pub tittel: Option<String>,
}

impl From<Avsender> for protobuf::v1::person::ansatt::Ansatt {
    fn from(value: Avsender) -> Self {
        Self {
            avdeling_id: value.avdeling_id,
            epost: Some(String::default()),
            brukernavn: value.brukernavn,
            navn: value.navn,
            region_id: value.region_id,
            tittel: value.tittel,
        }
    }
}
