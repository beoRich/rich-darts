#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema::schema::dartmatch))]
pub struct DartsMatch {
    pub id: u16,
    pub status: String
}

impl DartsMatch {}