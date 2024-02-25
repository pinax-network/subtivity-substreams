// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStats {
    /// number of successfully executed transactions in this block
    #[prost(int64, tag="1")]
    pub transaction_traces: i64,
    /// number of successfully executed actions/trace calls in this block
    #[prost(int64, tag="2")]
    pub trace_calls: i64,
    /// list of unique active wallets in this block
    #[prost(string, repeated, tag="3")]
    pub uaw: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// number of eip-4844 blobs in this block
    #[prost(int64, tag="4")]
    pub blobs: i64,
}
// @@protoc_insertion_point(module)
