use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum EventType {
    Rodtkjott,
    Hvittkjott,
    RodtkjottVurdering,
    MaskinIdentifisering,
    ManuellIdentifisering,
    BehandlendeEnhet,
    Maskinvurdering,
    Inspeksjon,
    Telefonsamtale,
    Brev,
    Notat,
    Avsluttet,
    TilOppfolging,
    UnderOppfolging,
    TilVurdering,
    ManuellVurdering,
    Ansvarlig,
    Arkivering,
    Publikum,
    Unsupported(String),
}

const RODTKJOTT_STR: &str = "no.mattilsynet.lib-schemas.protos.Rodtkjott";
const HVITTKJOTT_STR: &str = "no.mattilsynet.lib-schemas.protos.Hvittkjott";
const RODTKJOTT_VURDERING_STR: &str = "no.mattilsynet.lib-schemas.protos.Vurdering";
const MASKIN_IDENTIFISERING_STR: &str = "no.mattilsynet.lib-schemas.protos.MaskinIdentifisering";
const MANUELL_IDENTIFISERING_STR: &str = "no.mattilsynet.lib-schemas.protos.ManuellIdentifisering";
const BEHANDLENDE_ENHET_STR: &str = "no.mattilsynet.lib-schemas.protos.BehandlendeEnhet";
const MASKINVURDERING_STR: &str = "no.mattilsynet.lib-schemas.protos.Maskinvurdering";
const INSPEKSJON_STR: &str = "no.mattilsynet.lib-schemas.protos.Inspeksjon";
const TELEFONSAMTALE_STR: &str = "no.mattilsynet.lib-schemas.protos.Telefonsamtale";
const BREV_STR: &str = "no.mattilsynet.lib-schemas.protos.Brev";
const NOTAT_STR: &str = "no.mattilsynet.lib-schemas.protos.Notat";
const AVSLUTTET_STR: &str = "no.mattilsynet.lib-schemas.protos.Avsluttet";
const TIL_OPPFOLGING_STR: &str = "no.mattilsynet.lib-schemas.protos.TilOppfolging";
const UNDER_OPPFOLGING_STR: &str = "no.mattilsynet.lib-schemas.protos.UnderOppfolging";
const TIL_VURDERING_STR: &str = "no.mattilsynet.lib-schemas.protos.TilVurdering";
const MANUELL_VURDERING_STR: &str = "no.mattilsynet.lib-schemas.protos.ManuellVurdering";
const ANSVARLIG_STR: &str = "no.mattilsynet.lib-schemas.protos.Ansvarlig";
const ARKIVERING_STR: &str = "no.mattilsynet.lib-schemas.protos.Arkivering";
const PUBLIKUM_STR: &str = "no.mattilsynet.lib-schemas.protos.Publikum";

pub trait EventTypeTrait {
    fn as_str(&self) -> &str;
}

impl EventTypeTrait for EventType {
    fn as_str(&self) -> &str {
        match self {
            EventType::Rodtkjott => RODTKJOTT_STR,
            EventType::Hvittkjott => HVITTKJOTT_STR,
            EventType::RodtkjottVurdering => RODTKJOTT_VURDERING_STR,
            EventType::MaskinIdentifisering => MASKIN_IDENTIFISERING_STR,
            EventType::ManuellIdentifisering => MANUELL_IDENTIFISERING_STR,
            EventType::BehandlendeEnhet => BEHANDLENDE_ENHET_STR,
            EventType::Maskinvurdering => MASKINVURDERING_STR,
            EventType::Inspeksjon => INSPEKSJON_STR,
            EventType::Telefonsamtale => TELEFONSAMTALE_STR,
            EventType::Brev => BREV_STR,
            EventType::Notat => NOTAT_STR,
            EventType::Avsluttet => AVSLUTTET_STR,
            EventType::TilOppfolging => TIL_OPPFOLGING_STR,
            EventType::UnderOppfolging => UNDER_OPPFOLGING_STR,
            EventType::TilVurdering => TIL_VURDERING_STR,
            EventType::ManuellVurdering => MANUELL_VURDERING_STR,
            EventType::Ansvarlig => ANSVARLIG_STR,
            EventType::Arkivering => ARKIVERING_STR,
            EventType::Publikum => PUBLIKUM_STR,
            EventType::Unsupported(s) => s,
        }
    }
}

impl From<&str> for EventType {
    fn from(event_ty: &str) -> Self {
        match event_ty {
            RODTKJOTT_STR => EventType::Rodtkjott,
            HVITTKJOTT_STR => EventType::Hvittkjott,
            RODTKJOTT_VURDERING_STR => EventType::RodtkjottVurdering,
            MASKIN_IDENTIFISERING_STR => EventType::MaskinIdentifisering,
            MANUELL_IDENTIFISERING_STR => EventType::ManuellIdentifisering,
            BEHANDLENDE_ENHET_STR => EventType::BehandlendeEnhet,
            MASKINVURDERING_STR => EventType::Maskinvurdering,
            INSPEKSJON_STR => EventType::Inspeksjon,
            TELEFONSAMTALE_STR => EventType::Telefonsamtale,
            BREV_STR => EventType::Brev,
            NOTAT_STR => EventType::Notat,
            AVSLUTTET_STR => EventType::Avsluttet,
            TIL_OPPFOLGING_STR => EventType::TilOppfolging,
            UNDER_OPPFOLGING_STR => EventType::UnderOppfolging,
            TIL_VURDERING_STR => EventType::TilVurdering,
            MANUELL_VURDERING_STR => EventType::ManuellVurdering,
            ANSVARLIG_STR => EventType::Ansvarlig,
            ARKIVERING_STR => EventType::Arkivering,
            PUBLIKUM_STR => EventType::Publikum,
            other => EventType::Unsupported(other.to_string()),
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let event_type_str = match self {
            EventType::Rodtkjott => RODTKJOTT_STR,
            EventType::Hvittkjott => HVITTKJOTT_STR,
            EventType::RodtkjottVurdering => RODTKJOTT_VURDERING_STR,
            EventType::MaskinIdentifisering => MASKIN_IDENTIFISERING_STR,
            EventType::ManuellIdentifisering => MANUELL_IDENTIFISERING_STR,
            EventType::BehandlendeEnhet => BEHANDLENDE_ENHET_STR,
            EventType::Maskinvurdering => MASKINVURDERING_STR,
            EventType::Inspeksjon => INSPEKSJON_STR,
            EventType::Telefonsamtale => TELEFONSAMTALE_STR,
            EventType::Brev => BREV_STR,
            EventType::Notat => NOTAT_STR,
            EventType::Avsluttet => AVSLUTTET_STR,
            EventType::TilOppfolging => TIL_OPPFOLGING_STR,
            EventType::UnderOppfolging => UNDER_OPPFOLGING_STR,
            EventType::TilVurdering => TIL_VURDERING_STR,
            EventType::ManuellVurdering => MANUELL_VURDERING_STR,
            EventType::Ansvarlig => ANSVARLIG_STR,
            EventType::Arkivering => ARKIVERING_STR,
            EventType::Publikum => PUBLIKUM_STR,
            EventType::Unsupported(s) => s,
        };
        write!(f, "{}", event_type_str)
    }
}
