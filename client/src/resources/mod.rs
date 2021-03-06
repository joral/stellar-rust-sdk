//! Defines the basic resources of stellar's horizon end points and
//! implements their deserialization from JSON into rust types.

mod account;
mod amount;
mod asset;
mod base64string;
mod datum;
mod deserialize;

/// An effect represents specific changes that occur in the ledger resulting from operations.
pub mod effect;
mod ledger;
mod offer;
/// An operation is an individual command that mutates the ledger.
pub mod operation;
mod orderbook;
/// A payment path is a payment route from a source asset to a destination asset.
pub mod payment_path;
mod trade;
mod transaction;

/// # Stellar Resources
///
/// A collection of data types and resources used within the stellar api.
/// All the derives for XDR and JSON are implemented for the resources so that
/// they can be used with a client. Either for reading or for writing.
pub use self::account::Account;
pub use self::amount::{Amount, ParseAmountError};
pub use self::asset::{Asset, AssetIdentifier, Flags, ParseAssetIdentifierError};
pub use self::datum::Datum;
pub use self::effect::Effect;
pub use self::ledger::Ledger;
pub use self::offer::Offer;
pub use self::operation::{Operation, OperationKind};
pub use self::orderbook::Orderbook;
pub use self::payment_path::PaymentPath;
pub use self::trade::{Seller as TradeSeller, Trade, TradeAggregation};
pub use self::transaction::Memo;
pub use self::transaction::Transaction;
