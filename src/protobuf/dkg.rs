// This file is @generated by prost-build.
/// Packet is a wrapper around the three different types of DKG messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<super::common::Metadata>,
    #[prost(oneof = "packet::Bundle", tags = "1, 2, 3")]
    pub bundle: ::core::option::Option<packet::Bundle>,
}
/// Nested message and enum types in `Packet`.
pub mod packet {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Bundle {
        #[prost(message, tag = "1")]
        Deal(super::DealBundle),
        #[prost(message, tag = "2")]
        Response(super::ResponseBundle),
        #[prost(message, tag = "3")]
        Justification(super::JustificationBundle),
    }
}
/// DealBundle is a packet issued by a dealer that contains each individual
/// deals, as well as the coefficients of the public polynomial he used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DealBundle {
    /// Index of the dealer that issues these deals
    #[prost(uint32, tag = "1")]
    pub dealer_index: u32,
    /// Coefficients of the public polynomial that is created from the
    /// private polynomial from which the shares are derived.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub commits: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// list of deals for each individual share holders.
    #[prost(message, repeated, tag = "3")]
    pub deals: ::prost::alloc::vec::Vec<Deal>,
    /// session identifier of the protocol run
    #[prost(bytes = "vec", tag = "4")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// signature over the hash of the deal
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Deal contains a share for a participant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deal {
    #[prost(uint32, tag = "1")]
    pub share_index: u32,
    /// encryption of the share using ECIES
    #[prost(bytes = "vec", tag = "2")]
    pub encrypted_share: ::prost::alloc::vec::Vec<u8>,
}
/// ResponseBundle is a packet issued by a share holder that contains all the
/// responses (complaint and/or success) to broadcast.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseBundle {
    #[prost(uint32, tag = "1")]
    pub share_index: u32,
    #[prost(message, repeated, tag = "2")]
    pub responses: ::prost::alloc::vec::Vec<Response>,
    /// session identifier of the protocol run
    #[prost(bytes = "vec", tag = "3")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// signature over the hash of the response
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Response holds the response that a participant broadcast after having
/// received a deal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// index of the dealer for which this response is for
    #[prost(uint32, tag = "1")]
    pub dealer_index: u32,
    /// Status represents a complaint if set to false, a success if set to
    /// true.
    #[prost(bool, tag = "2")]
    pub status: bool,
}
/// JustificationBundle is a packet that holds all justifications a dealer must
/// produce
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JustificationBundle {
    #[prost(uint32, tag = "1")]
    pub dealer_index: u32,
    #[prost(message, repeated, tag = "2")]
    pub justifications: ::prost::alloc::vec::Vec<Justification>,
    /// session identifier of the protocol run
    #[prost(bytes = "vec", tag = "3")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// signature over the hash of the justification
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Justification holds the justification from a dealer after a participant
/// issued a complaint response because of a supposedly invalid deal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Justification {
    /// represents for who share holder this justification is
    #[prost(uint32, tag = "1")]
    pub share_index: u32,
    /// plaintext share so everyone can see it correct
    #[prost(bytes = "vec", tag = "2")]
    pub share: ::prost::alloc::vec::Vec<u8>,
}