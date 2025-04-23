use crate::eventtype::EventTypeTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum ArkivEventType {
    ArkivJobbAvsluttetDokument,
    ArkivJobbAvsluttSak,
    ArkivJobbJournalfoer,
    ArkivJobbDokument,
    ArkivJobbVedlegg,
    ArkivJobbBilderJournalpost,
    ArkivJobbSettSaksansvarlig,
    Unsupported(String),
    ArkivJobbAvskriv,
    BekymringsmeldingAvsluttet,
}

const ARKIVJOBB_AVSLUTTET_DOKUMENT: &str =
    "no.mattilsynet.lib-schemas.protos.ArkivJobbAvsluttetDokument";
const ARKIVJOBB_AVSLUTT_SAK: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobbAvsluttSak";
const ARKIVJOBB_JOURNALFOER: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobbJournalfoer";
const ARKIVJOBB_AVSKRIV: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobbAvskriv";
const ARKIVJOBB_DOKUMENT: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobDokument";
const ARKIVJOBB_VEDLEGG: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobbVedlegg";
const ARKIVJOBB_BILDE_JOURNALPOST: &str =
    "no.mattilsynet.lib-schemas.protos.ArkivJobbBildeJournalpost";
const ARKIVJOBB_SETT_SAKSANSVARLIG: &str =
    "no.mattilsynet.lib-schemas.protos.ArkivJobbSettSaksansvarlig";
const BEKYMRINGSMELDING_AVSLUTTET: &str = "no.mattilsynet.lib-schemas.protos.Avsluttet";

impl EventTypeTrait for ArkivEventType {
    fn as_str(&self) -> &str {
        match self {
            ArkivEventType::ArkivJobbAvsluttetDokument => ARKIVJOBB_AVSLUTTET_DOKUMENT,
            ArkivEventType::ArkivJobbDokument => ARKIVJOBB_DOKUMENT,
            ArkivEventType::ArkivJobbVedlegg => ARKIVJOBB_VEDLEGG,
            ArkivEventType::ArkivJobbAvsluttSak => ARKIVJOBB_AVSLUTT_SAK,
            ArkivEventType::ArkivJobbJournalfoer => ARKIVJOBB_JOURNALFOER,
            ArkivEventType::ArkivJobbAvskriv => ARKIVJOBB_AVSKRIV,
            ArkivEventType::ArkivJobbBilderJournalpost => ARKIVJOBB_BILDE_JOURNALPOST,
            ArkivEventType::ArkivJobbSettSaksansvarlig => ARKIVJOBB_SETT_SAKSANSVARLIG,
            ArkivEventType::BekymringsmeldingAvsluttet => BEKYMRINGSMELDING_AVSLUTTET,
            ArkivEventType::Unsupported(_) => "Unsupported",
        }
    }
}

impl From<&str> for ArkivEventType {
    fn from(event_ty: &str) -> Self {
        match event_ty {
            ARKIVJOBB_AVSLUTTET_DOKUMENT => ArkivEventType::ArkivJobbAvsluttetDokument,
            ARKIVJOBB_AVSLUTT_SAK => ArkivEventType::ArkivJobbAvsluttSak,
            ARKIVJOBB_JOURNALFOER => ArkivEventType::ArkivJobbJournalfoer,
            ARKIVJOBB_AVSKRIV => ArkivEventType::ArkivJobbAvskriv,
            ARKIVJOBB_DOKUMENT => ArkivEventType::ArkivJobbDokument,
            ARKIVJOBB_VEDLEGG => ArkivEventType::ArkivJobbVedlegg,
            ARKIVJOBB_BILDE_JOURNALPOST => ArkivEventType::ArkivJobbBilderJournalpost,
            ARKIVJOBB_SETT_SAKSANSVARLIG => ArkivEventType::ArkivJobbSettSaksansvarlig,
            BEKYMRINGSMELDING_AVSLUTTET => ArkivEventType::BekymringsmeldingAvsluttet,
            other_str => ArkivEventType::Unsupported(other_str.to_string()),
        }
    }
}
