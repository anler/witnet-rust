//! Constants used across the Wallet
//!
//! KeyPath constant values are taken from the WIP definition:
//! https://github.com/aesedepece/WIPs/blob/wip-adansdpc-hdwallets/wip-adansdpc-hdwallets.md#path-levels
use super::types;

/// Default offset used when returning paginated results.
pub static DEFAULT_PAGINATION_OFFSET: u32 = 0;

/// Default limit/page-size used when returning paginated results.
pub static DEFAULT_PAGINATION_LIMIT: u32 = 25;

/// Default account index used for the first account of newly-created wallet.
pub static DEFAULT_ACCOUNT_INDEX: u32 = 0;

/// Length of the random salt used when generating a session id.
pub static ID_SALT_LENGTH: usize = 32;

/// Hash iterations to use when deriving the key used for the session
/// id and the session id.
pub static ID_HASH_ITERATIONS: u32 = 4096;

/// Hash used when generating the session id.
pub static ID_HASH_FUNC: &types::HashFunction = &types::HashFunction::Sha256;

/// Maximum limit/page-size that can be used when returning paginated
/// results.
pub static MAX_PAGINATION_LIMIT: u32 = 1000;

/// Purpose section for an account keypath.
pub static KEYPATH_PURPOSE: u32 = 3;

/// Coin-type section for an account keypath.
pub static KEYPATH_COIN_TYPE: u32 = 4919;

/// The value for the 'change' path in an account keypath intended for
/// receiving payments.
pub static EXTERNAL_KEYCHAIN: u32 = 0;

/// The value for the 'change' path in an account keypath intended to
/// be used as a change address.
pub static INTERNAL_KEYCHAIN: u32 = 1;

/// Special key used to check if a decryption key is the correct one
/// for a wallet.
pub static ENCRYPTION_CHECK_KEY: &str = "ENC_KEY";

/// Special value stored with `ENCRYPTION_CHECK_KEY`.
pub static ENCRYPTION_CHECK_VALUE: () = ();
