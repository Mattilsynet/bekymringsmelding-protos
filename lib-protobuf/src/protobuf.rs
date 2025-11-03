pub mod v1 {
    pub mod event {
        include!(concat!(env!("OUT_DIR"), "/v1.event.rs"));
    }

    pub mod person {
        pub mod ansatt {
            include!(concat!(env!("OUT_DIR"), "/v1.person.ansatt.rs"));
        }
        pub mod generell_person {
            include!(concat!(env!("OUT_DIR"), "/v1.person.generell_person.rs"));
        }
    }

    pub mod virksomhet {
        pub mod organisasjon {
            include!(concat!(env!("OUT_DIR"), "/v1.virksomhet.organisasjon.rs"));
        }
        pub mod tilsynsobjekt {
            include!(concat!(env!("OUT_DIR"), "/v1.virksomhet.tilsynsobjekt.rs"));
        }
        pub mod slakteri {
            include!(concat!(env!("OUT_DIR"), "/v1.virksomhet.slakteri.rs"));
        }
    }

    pub mod rodtkjott {
        include!(concat!(env!("OUT_DIR"), "/v1.rodtkjott.rs"));
    }

    pub mod hvittkjott {
        pub mod hvittkott {
            include!(concat!(env!("OUT_DIR"), "/v1.hvittkjott.hvittkjott.rs"));
        }

        pub mod leveranse {
            include!(concat!(env!("OUT_DIR"), "/v1.hvittkjott.leveranse.rs"));
        }
    }

    pub mod publikum {
        include!(concat!(env!("OUT_DIR"), "/v1.publikum.rs"));
    }

    pub mod ulovlig_import_av_dyr {
        include!(concat!(env!("OUT_DIR"), "/v1.ulovlig_import_av_dyr.rs"));
    }

    pub mod identifisering {

        pub mod identifisering {
            include!(concat!(env!("OUT_DIR"), "/v1.identifisering.rs"));
        }

        pub mod behandlende_enhet {
            include!(concat!(
                env!("OUT_DIR"),
                "/v1.identifisering.behandlende_enhet.rs"
            ));
        }
        pub mod maskin_identifisering {
            include!(concat!(
                env!("OUT_DIR"),
                "/v1.identifisering.maskin_identifisering.rs"
            ));
        }
        pub mod manuell_identifisering {
            include!(concat!(
                env!("OUT_DIR"),
                "/v1.identifisering.manuell_identifisering.rs"
            ));
        }
    }

    pub mod vurdering {
        pub mod maskinvurdering {
            include!(concat!(env!("OUT_DIR"), "/v1.vurdering.maskinvurdering.rs"));
        }

        pub mod manuell_vurdering {
            include!(concat!(
                env!("OUT_DIR"),
                "/v1.vurdering.manuell_vurdering.rs"
            ));
        }
    }

    pub mod arkivering {
        pub mod arkiv_job {
            include!(concat!(env!("OUT_DIR"), "/v1.arkivering.arkiv_job.rs"));
        }

        pub mod arkivering_status {
            include!(concat!(
                env!("OUT_DIR"),
                "/v1.arkivering.arkivering_status.rs"
            ));
        }
    }

    pub mod oppfolging {
        include!(concat!(env!("OUT_DIR"), "/v1.oppfolging.rs"));
    }

    pub mod status {
        include!(concat!(env!("OUT_DIR"), "/v1.status.rs"));
    }
}

pub mod v2 {

    pub mod virksomhet {
        pub mod tilsynsobjekt {
            include!(concat!(env!("OUT_DIR"), "/v2.virksomhet.tilsynsobjekt.rs"));
        }
    }

    pub mod rodtkjott {
        include!(concat!(env!("OUT_DIR"), "/v2.rodtkjott.rs"));
    }

    pub mod publikum {
        include!(concat!(env!("OUT_DIR"), "/v2.publikum.rs"));
    }
}
