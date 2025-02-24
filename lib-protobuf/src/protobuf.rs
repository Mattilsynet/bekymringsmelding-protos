pub mod v1 {
    pub mod event {
        include!(concat!(env!("OUT_DIR"), "/v1.event.rs"));
    }

    pub mod person {
        pub mod ansatt {
            include!(concat!(env!("OUT_DIR"), "/v1.person.ansatt.rs"));
        }
    }

    pub mod virksomhet {
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

    pub mod publikum {
        include!(concat!(env!("OUT_DIR"), "/v1.publikum.rs"));
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

    pub mod identifisering {
        include!(concat!(env!("OUT_DIR"), "/v1.identifisering.rs"));
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
}
