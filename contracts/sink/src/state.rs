use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_storey::containers::Column;

#[cw_serde]
pub struct Ash {
    pub burner: Option<Addr>,
    pub amount: Coin,
    /// Point in time (block time) when the Ash was created
    pub time: Timestamp,
}

pub const ASHES: Column<Ash> = Column::new(7);
