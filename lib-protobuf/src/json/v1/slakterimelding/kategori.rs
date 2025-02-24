use serde::{Deserialize, Serialize};

use crate::protobuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Kategori {
    Dyrevelferd,
    MerkeavvikDyr,
}

impl From<Kategori> for protobuf::v1::rodtkjott::Kategori {
    fn from(value: Kategori) -> Self {
        match value {
            Kategori::Dyrevelferd => Self::Dyrevelferd,
            Kategori::MerkeavvikDyr => Self::MerkeavvikDyr,
        }
    }
}
