# bekymringsmelding-protos
Protobuf definisjoner for bekymringsmeldinger

## Hvordan ta ibruk
For og ta ibruk protos i Rust må du legge til følgende linje i [dependecies] Cargo.toml filen din:
```toml
# Erstatt "pakkenavn" med navnet du vil gi pakken i prosjektet ditt
pakkenavn = { git = "ssh://git@github.com/Mattilsynet/bekymringsmelding-protos" }
```
pakkenavn er navnet du vil gi pakken i ditt prosjekt, freks protobuf.

I tillegg må du legge til følgende linje i .zshrc filen din:
```bash
export CARGO_NET_GIT_FETCH_WITH_CLI=true
```

## Hendvendelser

Spørsmål knyttet til koden eller prosjektet kan stilles som issues her på GitHub

# For Mattilsynet-ansatte
Interne hendvendelser kan sendes via slack i kanalen #landdyrtilsyn
