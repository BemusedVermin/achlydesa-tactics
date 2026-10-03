//! Logistics: stocks, service jobs, transport, repair. See Execution Plan §4.1.
//! This slice implements physical stocks and exact consumption accounting
//! (High-Level Design §11.3, Simulation §16.1 and §16.2).
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod consumption;
pub mod error;
pub mod stock;

pub use consumption::{Activity, ConsumptionTable, consumed_between, cumulative, time_to_consume};
pub use error::LogisticsError;
pub use stock::{Shortfall, StockKind, StockSet};
