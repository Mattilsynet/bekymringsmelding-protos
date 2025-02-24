use serde::{Deserialize, Serialize};

use crate::protobuf::v1::rodtkjott::funn::Type as ProtoType;
use crate::protobuf::v1::rodtkjott::observasjon::Diagnose as ProtoDiagnose;
use crate::protobuf::v1::rodtkjott::Funn as ProtoFunn;
use crate::protobuf::v1::rodtkjott::Observasjon as ProtoObservasjon;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Funn {
    pub eftanummer: String,
    pub funn_id: String,
    pub observasjoner: Option<Vec<Observasjon>>,
    produsent_tilsynsobjekt_id: String,
    #[serde(rename = "type")]
    funn_type: Type,
    pub kontroll_dato: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Observasjon {
    beskrivelse: Option<String>,
    bilde_ids: Option<Vec<String>>,
    diagnose: Option<Kode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Kode {
    #[serde(rename = "type")]
    kode_type: String,
    kode: String,
    beskrivelse: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Type {
    Am,
    Pm,
}

impl From<Type> for ProtoType {
    fn from(value: Type) -> Self {
        match value {
            Type::Am => Self::Am,
            Type::Pm => Self::Pm,
        }
    }
}

impl From<Kode> for ProtoDiagnose {
    fn from(value: Kode) -> Self {
        ProtoDiagnose {
            beskrivelse: value.beskrivelse,
        }
    }
}

impl From<Observasjon> for ProtoObservasjon {
    fn from(value: Observasjon) -> Self {
        ProtoObservasjon {
            beskrivelse: value.beskrivelse,
            bilde_ids: value.bilde_ids.unwrap_or_default(),
            diagnose: value.diagnose.map(ProtoDiagnose::from),
        }
    }
}

impl From<Funn> for ProtoFunn {
    fn from(value: Funn) -> Self {
        ProtoFunn {
            bilde_ids: Vec::default(),
            funn_id: value.funn_id,
            observasjon: value.observasjoner.map_or(Vec::default(), |obs_vec| {
                obs_vec.into_iter().map(ProtoObservasjon::from).collect()
            }),
            funn_type: ProtoType::from(value.funn_type) as i32,
            kontroll_dato: value.kontroll_dato,
        }
    }
}
