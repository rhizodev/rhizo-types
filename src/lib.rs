//! # rhizo-types
//!
//! `rhizo-types` is a collection of common Structs and Enums used in the software components
//! comprising the Rhizo network.

use borsh::{BorshSerialize, BorshDeserialize};         
use serde::{Serialize, Deserialize};

/// Representation of RouteData as exists on the Solana blockchain
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteData {
    pub route: String,
    pub module_cid: [u8; 32],
    pub encodings: Vec<Encoding>,
    pub arguments: Vec<(Vec<u8>, ArgumentType)>
}

/// Representation of route deploy data
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteDeploy {
    pub source: Vec<u8>,
    pub metadata: RouteData
}

/// Supported HTTP route encoding types
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Eq, PartialEq, Serialize, Deserialize)]
pub enum Encoding {
    ApplicationJson,
    ApplicationOctetStream,
    TextHtml,
    TextPlain,
}

/// Supported types for route arguments
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Eq, PartialEq, Serialize, Deserialize)]
pub enum ArgumentType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Str,
    Array(CollectionType),
    Map(CollectionType, CollectionType)
}

/// Supported types for top-level collections of route argument types
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Eq, PartialEq, Serialize, Deserialize)]
pub enum CollectionType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Str,
    Array(NestedCollectionType),
    Map(NestedCollectionType, NestedCollectionType)
}

/// Supported types for singly nested collections of route argument types
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Eq, PartialEq, Serialize, Deserialize)]
pub enum NestedCollectionType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Str,
}

