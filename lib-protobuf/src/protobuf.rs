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
