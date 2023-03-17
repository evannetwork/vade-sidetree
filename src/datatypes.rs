use std::collections::HashMap;

#[cfg(feature = "sdk")]
use crate::in3_request_list::ResolveHttpRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "sdk")]
use std::os::raw::c_void;

pub use crate::vade_sidetree_client::{did::*, *};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum UpdateType {
    Update,
    Recovery,
    Deactivate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationRequestGenerated {
    pub r#type: String,
    pub suffix_data: SuffixData,
    pub delta: Delta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DIDCreateResult {
    pub operation_request: OperationRequestGenerated,
    pub did_suffix: String,
    pub update_key: JsonWebKey,
    pub recovery_key: JsonWebKey,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignedDataPayload {
    pub update_key: JsonWebKey,
    pub delta_hash: String,
}

pub struct SideTreeConfig {
    #[cfg(feature = "sdk")]
    pub request_id: *const c_void,
    #[cfg(feature = "sdk")]
    pub resolve_http_request: ResolveHttpRequest,
    pub sidetree_rest_api_url: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidetreeDidDocument {
    #[serde(rename = "@context")]
    pub context: String,
    pub did_document: DidDocument,
    pub did_document_metadata: DidDocumentMetadata,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DidDocument {
    pub id: String,
    #[serde(rename = "@context")]
    pub context: Value,
    pub verification_method: Option<Vec<KeyAgreement>>,
    pub service: Option<Vec<Service>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAgreement {
    pub id: String,
    pub controller: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub public_key_jwk: JsonWebKeyPublic,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocumentMetadata {
    pub method: MethodMetadata,
    pub deactivated: Option<bool>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodMetadata {
    pub published: bool,
    pub recovery_commitment: Option<String>,
    pub update_commitment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DidUpdatePayload {
    pub update_type: UpdateType,
    pub update_key: Option<JsonWebKey>,
    pub recovery_key: Option<JsonWebKey>,
    pub next_update_key: Option<JsonWebKeyPublic>,
    pub next_recovery_key: Option<JsonWebKeyPublic>,
    pub patches: Option<Vec<Patch>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DidCreateResponse {
    pub update_key: JsonWebKey,
    pub recovery_key: JsonWebKey,
    pub did: SidetreeDidDocument,
}

/// Message passed to vade containing the desired did implementation.
/// Does not perform action if type does not indicate did type.
#[derive(Serialize, Deserialize)]
pub struct TypeOptions {
    pub r#type: Option<String>,
}
