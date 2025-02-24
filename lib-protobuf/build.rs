use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "src/protos/v1/event.proto",
            "src/protos/v1/oppfolging.proto",
            "src/protos/v1/status.proto",
            "src/protos/v1/identifisering.proto",
            "src/protos/v1/rodtkjott/rodtkjott.proto",
            "src/protos/v1/rodtkjott/observasjon.proto",
            "src/protos/v1/rodtkjott/funn.proto",
            "src/protos/v1/person/ansatt.proto",
            "src/protos/v1/virksomhet/tilsynsobjekt.proto",
            "src/protos/v1/virksomhet/slakteri.proto",
            "src/protos/v1/publikum/publikum.proto",
            "src/protos/v1/vurdering/maskinvurdering.proto",
            "src/protos/v1/vurdering/manuell_vurdering.proto",
            "src/protos/v2/virksomhet/tilsynsobjekt.proto",
            "src/protos/v2/rodtkjott/funn.proto",
            "src/protos/v2/rodtkjott/rodtkjott.proto",
        ],
        &["src/"],
    )?;
    Ok(())
}
