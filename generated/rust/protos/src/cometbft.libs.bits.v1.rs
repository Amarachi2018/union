/// BitArray is an array of bits.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct BitArray {
    #[prost(int64, tag = "1")]
    pub bits: i64,
    #[prost(uint64, repeated, tag = "2")]
    pub elems: ::prost::alloc::vec::Vec<u64>,
}
impl ::prost::Name for BitArray {
    const NAME: &'static str = "BitArray";
    const PACKAGE: &'static str = "cometbft.libs.bits.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cometbft.libs.bits.v1.{}", Self::NAME)
    }
}
