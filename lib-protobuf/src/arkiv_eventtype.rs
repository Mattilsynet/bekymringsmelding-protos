use crate::eventtype::EventTypeTrait;

#[derive(Debug)]
pub enum ArkivEventType {
    RodtkjottAnkommet,
    RodtkjottAvsluttet,
    ArkivJobbAvsluttetDokument,
    ArkivJobbAvsluttSak,
    ArkivJobbJournalfoer,
    ArkivJobbDokument,
    ArkivJobbBilder,
    ArkivJobbBilderJournalpost,
    ArkivJobbSettSaksansvarlig,
    Unsupported(String),
}

const RODKJOTT_ANKOMMET: &str = "no.mattilsynet.lib-schemas.protos.Rodtkjott.Ankommet";
const RODKJOTT_AVSLUTTET: &str = "no.mattilsynet.lib-schemas.protos.Avsluttet";
const ARKIVJOBB_AVSLUTTET_DOKUMENT: &str =
    "no.mattilsynet.lib-schemas.protos.ArkivJobbAvsluttetDokument";
const ARKIVJOBB_AVSLUTT_SAK: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobbAvsluttSak";
const ARKIVJOBB_JOURNALPOST_FERDIG: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobbJournalfoer";
const ARKIVJOBB_DOKUMENT: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobDokument";
const ARKIVJOBB_BILDE: &str = "no.mattilsynet.lib-schemas.protos.ArkivJobbBilde";
const ARKIVJOBB_BILDE_JOURNALPOST: &str =
    "no.mattilsynet.lib-schemas.protos.ArkivJobbBildeJournalpost";
const ARKIVJOBB_SETT_SAKSANSVARLIG: &str =
    "no.mattilsynet.lib-schemas.protos.ArkivJobbSettSaksansvarlig";

impl EventTypeTrait for ArkivEventType {
    fn as_str(&self) -> &str {
        match self {
            ArkivEventType::RodtkjottAnkommet => RODKJOTT_ANKOMMET,
            ArkivEventType::RodtkjottAvsluttet => RODKJOTT_AVSLUTTET,
            ArkivEventType::ArkivJobbAvsluttetDokument => ARKIVJOBB_AVSLUTTET_DOKUMENT,
            ArkivEventType::ArkivJobbDokument => ARKIVJOBB_DOKUMENT,
            ArkivEventType::ArkivJobbBilder => ARKIVJOBB_BILDE,
            ArkivEventType::ArkivJobbAvsluttSak => ARKIVJOBB_AVSLUTT_SAK,
            ArkivEventType::ArkivJobbJournalfoer => ARKIVJOBB_JOURNALPOST_FERDIG,
            ArkivEventType::ArkivJobbBilderJournalpost => ARKIVJOBB_BILDE_JOURNALPOST,
            ArkivEventType::ArkivJobbSettSaksansvarlig => ARKIVJOBB_SETT_SAKSANSVARLIG,
            ArkivEventType::Unsupported(_) => "Unsupported",
        }
    }
}

impl From<&str> for ArkivEventType {
    fn from(event_ty: &str) -> Self {
        match event_ty {
            RODKJOTT_ANKOMMET => ArkivEventType::RodtkjottAnkommet,
            RODKJOTT_AVSLUTTET => ArkivEventType::RodtkjottAvsluttet,
            ARKIVJOBB_AVSLUTTET_DOKUMENT => ArkivEventType::ArkivJobbAvsluttetDokument,
            ARKIVJOBB_AVSLUTT_SAK => ArkivEventType::ArkivJobbAvsluttSak,
            ARKIVJOBB_JOURNALPOST_FERDIG => ArkivEventType::ArkivJobbJournalfoer,
            ARKIVJOBB_DOKUMENT => ArkivEventType::ArkivJobbDokument,
            ARKIVJOBB_BILDE => ArkivEventType::ArkivJobbBilder,
            ARKIVJOBB_BILDE_JOURNALPOST => ArkivEventType::ArkivJobbBilderJournalpost,
            ARKIVJOBB_SETT_SAKSANSVARLIG => ArkivEventType::ArkivJobbSettSaksansvarlig,
            other_str => ArkivEventType::Unsupported(other_str.to_string()),
        }
    }
}
