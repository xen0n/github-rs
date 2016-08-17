//! Definitions of types used throughout github-rs

// -----------------------------------------------------------------------------------------//
//                              REQUEST TYPE ALIASES                                        //
// -----------------------------------------------------------------------------------------//
/// Type used to explicitly define when we're dealing with HTML and not just JSON or any type of
/// string.
pub type HTML = String;

/// Represents a response from the server that has not been deserialized
/// by serde into the appropriate type
pub type RawJSON = String;

// -----------------------------------------------------------------------------------------//
//                              CLIENT TYPE ALIASES                                         //
// -----------------------------------------------------------------------------------------//
/// Used to represent the access token to authenticate with the
/// Github API
pub type AccessToken = String;
