use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::protobuf::v1::rodtkjott::{
    Status as ProtoStatus, StatusBegrunnelse as ProtoStatusBegrunnelse, Vurdering as ProtoVurdering,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vurdering {
    #[serde(rename(deserialize = "fordeltTil"))]
    pub fordelt_til: Option<String>,
    #[serde(rename(deserialize = "sistRedigert"))]
    pub sist_redigert: DateTime<Utc>,
    #[serde(rename(deserialize = "sistRedigertAv"))]
    pub sist_redigert_av: String,
    pub status: Status,
    #[serde(rename(deserialize = "statusBegrunnelse"))]
    pub status_begrunnelse: Option<StatusBegrunnelse>,
    #[serde(rename(deserialize = "tilknyttetSaksnummer"))]
    pub tilknyttet_saksnummer: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Status {
    #[serde(rename(deserialize = "NY"))]
    Ny,
    #[serde(rename(deserialize = "UNDER_VURDERING"))]
    UnderVurdering,
    #[serde(rename(deserialize = "VURDERT"))]
    Vurdert,
    #[serde(rename(deserialize = "DUBLETT"))]
    Dublett,
    #[serde(rename(deserialize = "INSPEKSJON"))]
    Inspeksjon,
    #[serde(rename(deserialize = "SAKSBEHANDLING"))]
    Saksbehandling,
    #[serde(rename(deserialize = "UTKAST"))]
    Utkast,
    #[serde(rename(deserialize = "SENERE_OPPFØLGING"))]
    SenereOppfolging,
}

impl Status {
    pub fn to_i32(&self) -> i32 {
        match *self {
            Status::Ny => 0,
            Status::UnderVurdering => 1,
            Status::Vurdert => 2,
            Status::Dublett => 3,
            Status::Inspeksjon => 4,
            Status::Saksbehandling => 5,
            Status::Utkast => 6,
            Status::SenereOppfolging => 7,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum StatusBegrunnelse {
    #[serde(rename(deserialize = "UNDER_ENDRING"))]
    UnderEndring,
    #[serde(rename(deserialize = "UNDER_VURDERING"))]
    Gjennomfort,
    #[serde(rename(deserialize = "INGEN_BEKYMRING"))]
    IngenBekymring,
    #[serde(rename(deserialize = "RESSURSMANGEL"))]
    Ressursmangel,
    #[serde(rename(deserialize = "SESONGBETONT_PROBLEMSTILLING"))]
    SesongbetontProblemstilling,
    #[serde(rename(deserialize = "MIDLERTIDIG_RESSURSMANGEL"))]
    MidlertidigRessursmangel,
}

impl StatusBegrunnelse {
    pub fn to_i32(&self) -> i32 {
        match *self {
            StatusBegrunnelse::UnderEndring => 0,
            StatusBegrunnelse::Gjennomfort => 1,
            StatusBegrunnelse::IngenBekymring => 2,
            StatusBegrunnelse::Ressursmangel => 3,
            StatusBegrunnelse::SesongbetontProblemstilling => 4,
            StatusBegrunnelse::MidlertidigRessursmangel => 5,
        }
    }
}

impl From<Vurdering> for ProtoVurdering {
    fn from(value: Vurdering) -> Self {
        Self {
            fordelt_til: value.fordelt_til,
            sist_redigert: value.sist_redigert.to_rfc3339(),
            status: ProtoStatus::from(value.status) as i32,
            status_begrunnelse: value.status_begrunnelse.map(|v| {
                let status_begrunnelse = ProtoStatusBegrunnelse::from(v);
                i32::from(status_begrunnelse)
            }),
            tilknyttet_saksnummer: value.tilknyttet_saksnummer,
        }
    }
}

impl From<Status> for ProtoStatus {
    fn from(value: Status) -> Self {
        match value {
            Status::Ny => Self::Ny,
            Status::UnderVurdering => Self::UnderVurdering,
            Status::Vurdert => Self::Vurdert,
            Status::Dublett => Self::Dublett,
            Status::Inspeksjon => Self::Inspeksjon,
            Status::Saksbehandling => Self::Inspeksjon,
            Status::Utkast => Self::Utkast,
            Status::SenereOppfolging => Self::SenereOppfolging,
        }
    }
}

impl From<StatusBegrunnelse> for ProtoStatusBegrunnelse {
    fn from(value: StatusBegrunnelse) -> Self {
        match value {
            StatusBegrunnelse::UnderEndring => Self::UnderEndring,
            StatusBegrunnelse::Gjennomfort => Self::Gjennomfort,
            StatusBegrunnelse::IngenBekymring => Self::IngenBekymring,
            StatusBegrunnelse::Ressursmangel => Self::Ressursmangel,
            StatusBegrunnelse::SesongbetontProblemstilling => Self::SesongbetontProblemstilling,
            StatusBegrunnelse::MidlertidigRessursmangel => Self::MidlertidigRessursmangel,
        }
    }
}
