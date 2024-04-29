// #![warn(missing_docs)]
#![doc(
    html_logo_url = "https://www.grantspassoregon.gov/DocumentCenter/View/31368/GPLogo_450W-PNG"
)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
pub mod address;
pub mod address_components;
pub mod business;
pub mod compare;
pub mod geo;
pub mod import;
pub mod lexisnexis;
pub mod parser;
pub mod utils;

pub mod prelude {
    pub use crate::address::{
        Address, AddressDelta, AddressDeltas, Addresses, CommonAddress, CommonAddresses,
        PartialAddress, PartialAddresses,
    };
    pub use crate::address_components::{
        deserialize_mixed_post_type, deserialize_mixed_pre_directional, AddressStatus,
        StreetNamePostType, StreetNamePreDirectional, SubaddressType,
    };
    pub use crate::business::{BusinessLicenses, BusinessMatchRecords};
    pub use crate::compare::{
        AddressMatch, FireInspectionMatchRecords, FireInspectionMatches, MatchPartialRecord,
        MatchPartialRecords, MatchRecord, MatchRecords, MatchStatus, Mismatch,
    };
    pub use crate::geo::{GeoAddress, GeoAddresses, Point, SpatialAddress, SpatialAddresses};
    pub use crate::import::{
        Businesses, CityAddress, CityAddresses, CountyAddress, CountyAddresses, FireInspection,
        FireInspections, GrantsPass2022Address, GrantsPass2022Addresses, GrantsPassAddress,
        GrantsPassAddresses, GrantsPassSpatialAddress, GrantsPassSpatialAddresses,
        JosephineCountyAddress, JosephineCountyAddresses, JosephineCountySpatialAddress,
        JosephineCountySpatialAddresses,
    };
    pub use crate::lexisnexis::LexisNexis;
    pub use crate::parser::{
        multi_word, parse_address, parse_address_number, parse_address_number_suffix,
        parse_complete_street_name, parse_post_type, parse_pre_directional, parse_street_name,
        parse_subaddress_element, parse_subaddress_elements, parse_subaddress_identifiers,
        parse_subaddress_type, recursive_post_type,
    };
    pub use crate::utils::{from_csv, load_bin, save, to_csv, Portable, Vectorized};
}
