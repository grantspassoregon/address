use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

/// The `StreetNamePostType` represents the street name post type of an address.  Acceptable post
/// types include the list of recognized street suffix names in Appendix C1 of the United States
/// Postal Service (USPS) Publication 28 - Postal Addressing Standards.
#[allow(missing_docs)]
#[derive(
    Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default, PartialOrd, Ord, Hash,
)]
pub enum StreetNamePostType {
    ALLEY,
    ANEX,
    ARCADE,
    AVENUE,
    BAYOU,
    BEACH,
    BEND,
    BLUFF,
    BLUFFS,
    BOTTOM,
    BOULEVARD,
    BRANCH,
    BRIDGE,
    BROOK,
    BROOKS,
    BURG,
    BURGS,
    BYPASS,
    CAMP,
    CANYON,
    CAPE,
    CAUSEWAY,
    CENTER,
    CENTERS,
    CIRCLE,
    CIRCLES,
    CLIFF,
    CLIFFS,
    CLUB,
    COMMON,
    COMMONS,
    CORNER,
    CORNERS,
    COURSE,
    COURT,
    COURTS,
    COVE,
    COVES,
    CREEK,
    CRESCENT,
    CREST,
    CROSSING,
    CROSSROAD,
    CROSSROADS,
    CURVE,
    CUTOFF,
    DALE,
    DAM,
    DIVIDE,
    DRIVE,
    /// Drive Cutoff, added to accommodate Azalea Dr Cutoff.  As Azalea Dr exists, and Drive is the
    /// post type, the Cutoff from Azalea Dr receives a second post type Cutoff.  The FGDC standard
    /// notes that the complete street name can take multiple post types, but to accommodate this in
    /// our data model, we have added a compound post type to the enumeration of valid types.
    #[serde(rename(deserialize = "DRIVE CUTOFF"))]
    DriveCutoff,
    DRIVES,
    ESTATE,
    ESTATES,
    EXPRESSWAY,
    EXTENSION,
    EXTENSIONS,
    FALL,
    FALLS,
    FERRY,
    FIELD,
    FIELDS,
    FLAT,
    FLATS,
    FORD,
    FORDS,
    FOREST,
    FORGE,
    FORGES,
    FORK,
    FORKS,
    FORT,
    FREEWAY,
    GARDEN,
    GARDENS,
    GATEWAY,
    GLEN,
    GLENS,
    GREEN,
    GREENS,
    GROVE,
    GROVES,
    HARBOR,
    HARBORS,
    HAVEN,
    HEIGHTS,
    HIGHWAY,
    HILL,
    HILLS,
    HOLLOW,
    INLET,
    ISLAND,
    ISLANDS,
    ISLE,
    JUNCTION,
    JUNCTIONS,
    KEY,
    KEYS,
    KNOLL,
    KNOLLS,
    LAKE,
    LAKES,
    LAND,
    LANDING,
    LANE,
    LIGHT,
    LIGHTS,
    LOAF,
    LOCK,
    LOCKS,
    LODGE,
    LOOP,
    MALL,
    MANOR,
    MANORS,
    MEADOW,
    MEADOWS,
    MEWS,
    MILL,
    MILLS,
    MISSION,
    MOTORWAY,
    MOUNT,
    MOUNTAIN,
    MOUNTAINS,
    NECK,
    ORCHARD,
    OVAL,
    OVERPASS,
    PARK,
    PARKWAY,
    PASS,
    PASSAGE,
    PATH,
    PIKE,
    PINE,
    PINES,
    PLACE,
    PLAIN,
    PLAINS,
    PLAZA,
    POINT,
    POINTS,
    PORT,
    PORTS,
    PRAIRIE,
    RADIAL,
    RAMP,
    RANCH,
    RAPID,
    RAPIDS,
    REST,
    RIDGE,
    RIDGES,
    RIVER,
    ROAD,
    ROADS,
    ROUTE,
    ROW,
    RUE,
    RUN,
    SHOAL,
    SHOALS,
    SHORE,
    SHORES,
    SKYWAY,
    SPRING,
    SPRINGS,
    SPUR,
    SQUARE,
    SQUARES,
    STATION,
    STRAVENUE,
    STREAM,
    #[default]
    STREET,
    STREETS,
    SUMMIT,
    TERRACE,
    THROUGHWAY,
    TRACE,
    TRACK,
    TRAFFICWAY,
    TRAIL,
    TRAILER,
    TUNNEL,
    TURNPIKE,
    UNDERPASS,
    UNION,
    UNIONS,
    VALLEY,
    VALLEYS,
    VIADUCT,
    VIEW,
    VIEWS,
    VILLAGE,
    VILLAGES,
    VILLE,
    VISTA,
    WALK,
    WALL,
    WAY,
    WAYS,
    WELL,
    WELLS,
}

impl StreetNamePostType {
    /// The `abbreviate` method returns the standard postal abbreviation for a street name post
    /// type.
    pub fn abbreviate(&self) -> String {
        match self {
            Self::ALLEY => "ALY".to_string(),
            Self::ANEX => "ANX".to_string(),
            Self::ARCADE => "ARC".to_string(),
            Self::AVENUE => "AVE".to_string(),
            Self::BAYOU => "BYU".to_string(),
            Self::BEACH => "BCH".to_string(),
            Self::BEND => "BND".to_string(),
            Self::BLUFF => "BLF".to_string(),
            Self::BLUFFS => "BLFS".to_string(),
            Self::BOTTOM => "BTM".to_string(),
            Self::BOULEVARD => "BLVD".to_string(),
            Self::BRANCH => "BR".to_string(),
            Self::BRIDGE => "BRG".to_string(),
            Self::BROOK => "BRK".to_string(),
            Self::BROOKS => "BRKS".to_string(),
            Self::BURG => "BG".to_string(),
            Self::BURGS => "BGS".to_string(),
            Self::BYPASS => "BYP".to_string(),
            Self::CAMP => "CP".to_string(),
            Self::CANYON => "CYN".to_string(),
            Self::CAPE => "CPE".to_string(),
            Self::CAUSEWAY => "CSWY".to_string(),
            Self::CENTER => "CTR".to_string(),
            Self::CENTERS => "CTRS".to_string(),
            Self::CIRCLE => "CIR".to_string(),
            Self::CIRCLES => "CIRS".to_string(),
            Self::CLIFF => "CLF".to_string(),
            Self::CLIFFS => "CLFS".to_string(),
            Self::CLUB => "CLB".to_string(),
            Self::COMMON => "CMN".to_string(),
            Self::COMMONS => "CMNS".to_string(),
            Self::CORNER => "COR".to_string(),
            Self::CORNERS => "CORS".to_string(),
            Self::COURSE => "CRSE".to_string(),
            Self::COURT => "CT".to_string(),
            Self::COURTS => "CTS".to_string(),
            Self::COVE => "CV".to_string(),
            Self::COVES => "CVS".to_string(),
            Self::CREEK => "CRK".to_string(),
            Self::CRESCENT => "CRES".to_string(),
            Self::CREST => "CRST".to_string(),
            Self::CROSSING => "XING".to_string(),
            Self::CROSSROAD => "XRD".to_string(),
            Self::CROSSROADS => "XRDS".to_string(),
            Self::CURVE => "CURV".to_string(),
            Self::CUTOFF => "CTOFF".to_string(),
            Self::DALE => "DL".to_string(),
            Self::DAM => "DM".to_string(),
            Self::DIVIDE => "DV".to_string(),
            Self::DRIVE => "DR".to_string(),
            Self::DRIVES => "DRS".to_string(),
            Self::DriveCutoff => "DRCTOFF".to_string(),
            Self::ESTATE => "EST".to_string(),
            Self::ESTATES => "ESTS".to_string(),
            Self::EXPRESSWAY => "EXPY".to_string(),
            Self::EXTENSION => "EXT".to_string(),
            Self::EXTENSIONS => "EXTS".to_string(),
            Self::FALL => "FALL".to_string(),
            Self::FALLS => "FLS".to_string(),
            Self::FERRY => "FRY".to_string(),
            Self::FIELD => "FLD".to_string(),
            Self::FIELDS => "FLDS".to_string(),
            Self::FLAT => "FLT".to_string(),
            Self::FLATS => "FLTS".to_string(),
            Self::FORD => "FRD".to_string(),
            Self::FORDS => "FRDS".to_string(),
            Self::FOREST => "FRST".to_string(),
            Self::FORGE => "FRG".to_string(),
            Self::FORGES => "FRGS".to_string(),
            Self::FORK => "FRK".to_string(),
            Self::FORKS => "FRKS".to_string(),
            Self::FORT => "FT".to_string(),
            Self::FREEWAY => "FWY".to_string(),
            Self::GARDEN => "GDN".to_string(),
            Self::GARDENS => "GDNS".to_string(),
            Self::GATEWAY => "GTWY".to_string(),
            Self::GLEN => "GLN".to_string(),
            Self::GLENS => "GLNS".to_string(),
            Self::GREEN => "GRN".to_string(),
            Self::GREENS => "GRNS".to_string(),
            Self::GROVE => "GRV".to_string(),
            Self::GROVES => "GRVS".to_string(),
            Self::HARBOR => "HBR".to_string(),
            Self::HARBORS => "HBRS".to_string(),
            Self::HAVEN => "HVN".to_string(),
            Self::HEIGHTS => "HTS".to_string(),
            Self::HIGHWAY => "HWY".to_string(),
            Self::HILL => "HL".to_string(),
            Self::HILLS => "HLS".to_string(),
            Self::HOLLOW => "HOLW".to_string(),
            Self::INLET => "INLT".to_string(),
            Self::ISLAND => "IS".to_string(),
            Self::ISLANDS => "ISS".to_string(),
            Self::ISLE => "ISLE".to_string(),
            Self::JUNCTION => "JCT".to_string(),
            Self::JUNCTIONS => "JCTS".to_string(),
            Self::KEY => "KY".to_string(),
            Self::KEYS => "KYS".to_string(),
            Self::KNOLL => "KNL".to_string(),
            Self::KNOLLS => "KNLS".to_string(),
            Self::LAKE => "LK".to_string(),
            Self::LAKES => "LKS".to_string(),
            Self::LAND => "LAND".to_string(),
            Self::LANDING => "LNDG".to_string(),
            Self::LANE => "LN".to_string(),
            Self::LIGHT => "LGT".to_string(),
            Self::LIGHTS => "LGTS".to_string(),
            Self::LOAF => "LF".to_string(),
            Self::LOCK => "LCK".to_string(),
            Self::LOCKS => "LCKS".to_string(),
            Self::LODGE => "LDG".to_string(),
            Self::LOOP => "LOOP".to_string(),
            Self::MALL => "MALL".to_string(),
            Self::MANOR => "MNR".to_string(),
            Self::MANORS => "MNRS".to_string(),
            Self::MEADOW => "MDW".to_string(),
            Self::MEADOWS => "MDWS".to_string(),
            Self::MEWS => "MEWS".to_string(),
            Self::MILL => "ML".to_string(),
            Self::MILLS => "MLS".to_string(),
            Self::MISSION => "MSN".to_string(),
            Self::MOTORWAY => "MTWY".to_string(),
            Self::MOUNT => "MT".to_string(),
            Self::MOUNTAIN => "MTN".to_string(),
            Self::MOUNTAINS => "MTNS".to_string(),
            Self::NECK => "NCK".to_string(),
            Self::ORCHARD => "ORCH".to_string(),
            Self::OVAL => "OVAL".to_string(),
            Self::OVERPASS => "OPAS".to_string(),
            Self::PARK => "PARK".to_string(),
            Self::PARKWAY => "PKWY".to_string(),
            Self::PASS => "PASS".to_string(),
            Self::PASSAGE => "PSGE".to_string(),
            Self::PATH => "PATH".to_string(),
            Self::PIKE => "PIKE".to_string(),
            Self::PINE => "PNE".to_string(),
            Self::PINES => "PNES".to_string(),
            Self::PLACE => "PL".to_string(),
            Self::PLAIN => "PLN".to_string(),
            Self::PLAINS => "PLNS".to_string(),
            Self::PLAZA => "PLZ".to_string(),
            Self::POINT => "PT".to_string(),
            Self::POINTS => "PTS".to_string(),
            Self::PORT => "PRT".to_string(),
            Self::PORTS => "PRTS".to_string(),
            Self::PRAIRIE => "PR".to_string(),
            Self::RADIAL => "RADL".to_string(),
            Self::RAMP => "RAMP".to_string(),
            Self::RANCH => "RNCH".to_string(),
            Self::RAPID => "RPD".to_string(),
            Self::RAPIDS => "RPDS".to_string(),
            Self::REST => "RST".to_string(),
            Self::RIDGE => "RDG".to_string(),
            Self::RIDGES => "RDGS".to_string(),
            Self::RIVER => "RIV".to_string(),
            Self::ROAD => "RD".to_string(),
            Self::ROADS => "RDS".to_string(),
            Self::ROUTE => "RTE".to_string(),
            Self::ROW => "ROW".to_string(),
            Self::RUE => "RUE".to_string(),
            Self::RUN => "RUN".to_string(),
            Self::SHOAL => "SHL".to_string(),
            Self::SHOALS => "SHLS".to_string(),
            Self::SHORE => "SHR".to_string(),
            Self::SHORES => "SHRS".to_string(),
            Self::SKYWAY => "SKWY".to_string(),
            Self::SPRING => "SPG".to_string(),
            Self::SPRINGS => "SPGS".to_string(),
            Self::SPUR => "SPUR".to_string(),
            Self::SQUARE => "SQ".to_string(),
            Self::SQUARES => "SQS".to_string(),
            Self::STATION => "STA".to_string(),
            Self::STRAVENUE => "STRA".to_string(),
            Self::STREAM => "STRM".to_string(),
            Self::STREET => "ST".to_string(),
            Self::STREETS => "STS".to_string(),
            Self::SUMMIT => "SMT".to_string(),
            Self::TERRACE => "TER".to_string(),
            Self::THROUGHWAY => "TRWY".to_string(),
            Self::TRACE => "TRCE".to_string(),
            Self::TRACK => "TRAK".to_string(),
            Self::TRAFFICWAY => "TRFY".to_string(),
            Self::TRAIL => "TRL".to_string(),
            Self::TRAILER => "TRLR".to_string(),
            Self::TUNNEL => "TUNL".to_string(),
            Self::TURNPIKE => "TPKE".to_string(),
            Self::UNDERPASS => "UPAS".to_string(),
            Self::UNION => "UN".to_string(),
            Self::UNIONS => "UNS".to_string(),
            Self::VALLEY => "VLY".to_string(),
            Self::VALLEYS => "VLYS".to_string(),
            Self::VIADUCT => "VIA".to_string(),
            Self::VIEW => "VW".to_string(),
            Self::VIEWS => "VWS".to_string(),
            Self::VILLAGE => "VLG".to_string(),
            Self::VILLAGES => "VLGS".to_string(),
            Self::VILLE => "VL".to_string(),
            Self::VISTA => "VIS".to_string(),
            Self::WALK => "WALK".to_string(),
            Self::WALL => "WALL".to_string(),
            Self::WAY => "WAY".to_string(),
            Self::WAYS => "WAYS".to_string(),
            Self::WELL => "WL".to_string(),
            Self::WELLS => "WLS".to_string(),
        }
    }
}

/// Deserialization function for post type abbreviations.  This works if all the post types in the
/// data observe the official postal contraction.  For post types with a mix of abbreviations and
/// alternative spellings, [`deserialize_mixed_post_type()`] will work better.
pub fn deserialize_abbreviated_post_type<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<StreetNamePostType>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        "ALY" => Ok(Some(StreetNamePostType::ALLEY)),
        "ANX" => Ok(Some(StreetNamePostType::ANEX)),
        "ARC" => Ok(Some(StreetNamePostType::ARCADE)),
        "AVE" => Ok(Some(StreetNamePostType::AVENUE)),
        "BYU" => Ok(Some(StreetNamePostType::BAYOU)),
        "BCH" => Ok(Some(StreetNamePostType::BEACH)),
        "BND" => Ok(Some(StreetNamePostType::BEND)),
        "BLF" => Ok(Some(StreetNamePostType::BLUFF)),
        "BLFS" => Ok(Some(StreetNamePostType::BLUFFS)),
        "BTM" => Ok(Some(StreetNamePostType::BOTTOM)),
        "BLVD" => Ok(Some(StreetNamePostType::BOULEVARD)),
        "BR" => Ok(Some(StreetNamePostType::BRANCH)),
        "BRG" => Ok(Some(StreetNamePostType::BRIDGE)),
        "BRK" => Ok(Some(StreetNamePostType::BROOK)),
        "BRKS" => Ok(Some(StreetNamePostType::BROOKS)),
        "BG" => Ok(Some(StreetNamePostType::BURG)),
        "BGS" => Ok(Some(StreetNamePostType::BURGS)),
        "BYP" => Ok(Some(StreetNamePostType::BYPASS)),
        "CP" => Ok(Some(StreetNamePostType::CAMP)),
        "CYN" => Ok(Some(StreetNamePostType::CANYON)),
        "CPE" => Ok(Some(StreetNamePostType::CAPE)),
        "CSWY" => Ok(Some(StreetNamePostType::CAUSEWAY)),
        "CTR" => Ok(Some(StreetNamePostType::CENTER)),
        "CTRS" => Ok(Some(StreetNamePostType::CENTERS)),
        "CIR" => Ok(Some(StreetNamePostType::CIRCLE)),
        "CIRS" => Ok(Some(StreetNamePostType::CIRCLES)),
        "CLF" => Ok(Some(StreetNamePostType::CLIFF)),
        "CLFS" => Ok(Some(StreetNamePostType::CLIFFS)),
        "CLB" => Ok(Some(StreetNamePostType::CLUB)),
        "CMN" => Ok(Some(StreetNamePostType::COMMON)),
        "CMNS" => Ok(Some(StreetNamePostType::COMMONS)),
        "COR" => Ok(Some(StreetNamePostType::CORNER)),
        "CORS" => Ok(Some(StreetNamePostType::CORNERS)),
        "CRSE" => Ok(Some(StreetNamePostType::COURSE)),
        "CT" => Ok(Some(StreetNamePostType::COURT)),
        "CTS" => Ok(Some(StreetNamePostType::COURTS)),
        "CV" => Ok(Some(StreetNamePostType::COVE)),
        "CVS" => Ok(Some(StreetNamePostType::COVES)),
        "CRK" => Ok(Some(StreetNamePostType::CREEK)),
        "CRES" => Ok(Some(StreetNamePostType::CRESCENT)),
        "CRST" => Ok(Some(StreetNamePostType::CREST)),
        "XING" => Ok(Some(StreetNamePostType::CROSSING)),
        "XRD" => Ok(Some(StreetNamePostType::CROSSROAD)),
        "XRDS" => Ok(Some(StreetNamePostType::CROSSROADS)),
        "CURV" => Ok(Some(StreetNamePostType::CURVE)),
        "CUTOFF" => Ok(Some(StreetNamePostType::CUTOFF)),
        "DL" => Ok(Some(StreetNamePostType::DALE)),
        "DM" => Ok(Some(StreetNamePostType::DAM)),
        "DV" => Ok(Some(StreetNamePostType::DIVIDE)),
        "DR" => Ok(Some(StreetNamePostType::DRIVE)),
        "DR CUTOFF" => Ok(Some(StreetNamePostType::DriveCutoff)),
        "DRS" => Ok(Some(StreetNamePostType::DRIVES)),
        "EST" => Ok(Some(StreetNamePostType::ESTATE)),
        "ESTS" => Ok(Some(StreetNamePostType::ESTATES)),
        "EXPY" => Ok(Some(StreetNamePostType::EXPRESSWAY)),
        "EXT" => Ok(Some(StreetNamePostType::EXTENSION)),
        "EXTS" => Ok(Some(StreetNamePostType::EXTENSIONS)),
        "FALL" => Ok(Some(StreetNamePostType::FALL)),
        "FLS" => Ok(Some(StreetNamePostType::FALLS)),
        "FRY" => Ok(Some(StreetNamePostType::FERRY)),
        "FLD" => Ok(Some(StreetNamePostType::FIELD)),
        "FLDS" => Ok(Some(StreetNamePostType::FIELDS)),
        "FLT" => Ok(Some(StreetNamePostType::FLAT)),
        "FLTS" => Ok(Some(StreetNamePostType::FLATS)),
        "FRD" => Ok(Some(StreetNamePostType::FORD)),
        "FRDS" => Ok(Some(StreetNamePostType::FORDS)),
        "FRST" => Ok(Some(StreetNamePostType::FOREST)),
        "FRG" => Ok(Some(StreetNamePostType::FORGE)),
        "FRGS" => Ok(Some(StreetNamePostType::FORGES)),
        "FRK" => Ok(Some(StreetNamePostType::FORK)),
        "FRKS" => Ok(Some(StreetNamePostType::FORKS)),
        "FT" => Ok(Some(StreetNamePostType::FORT)),
        "FWY" => Ok(Some(StreetNamePostType::FREEWAY)),
        "GDN" => Ok(Some(StreetNamePostType::GARDEN)),
        "GDNS" => Ok(Some(StreetNamePostType::GARDENS)),
        "GTWY" => Ok(Some(StreetNamePostType::GATEWAY)),
        "GLN" => Ok(Some(StreetNamePostType::GLEN)),
        "GLNS" => Ok(Some(StreetNamePostType::GLENS)),
        "GRN" => Ok(Some(StreetNamePostType::GREEN)),
        "GRNS" => Ok(Some(StreetNamePostType::GREENS)),
        "GRV" => Ok(Some(StreetNamePostType::GROVE)),
        "GRVS" => Ok(Some(StreetNamePostType::GROVES)),
        "HBR" => Ok(Some(StreetNamePostType::HARBOR)),
        "HBRS" => Ok(Some(StreetNamePostType::HARBORS)),
        "HVN" => Ok(Some(StreetNamePostType::HAVEN)),
        "HTS" => Ok(Some(StreetNamePostType::HEIGHTS)),
        "HWY" => Ok(Some(StreetNamePostType::HIGHWAY)),
        "HL" => Ok(Some(StreetNamePostType::HILL)),
        "HLS" => Ok(Some(StreetNamePostType::HILLS)),
        "HOLW" => Ok(Some(StreetNamePostType::HOLLOW)),
        "INLT" => Ok(Some(StreetNamePostType::INLET)),
        "IS" => Ok(Some(StreetNamePostType::ISLAND)),
        "ISS" => Ok(Some(StreetNamePostType::ISLANDS)),
        "ISLE" => Ok(Some(StreetNamePostType::ISLE)),
        "JCT" => Ok(Some(StreetNamePostType::JUNCTION)),
        "JCTS" => Ok(Some(StreetNamePostType::JUNCTIONS)),
        "KY" => Ok(Some(StreetNamePostType::KEY)),
        "KYS" => Ok(Some(StreetNamePostType::KEYS)),
        "KNL" => Ok(Some(StreetNamePostType::KNOLL)),
        "KNLS" => Ok(Some(StreetNamePostType::KNOLLS)),
        "LK" => Ok(Some(StreetNamePostType::LAKE)),
        "LKS" => Ok(Some(StreetNamePostType::LAKES)),
        "LAND" => Ok(Some(StreetNamePostType::LAND)),
        "LNDG" => Ok(Some(StreetNamePostType::LANDING)),
        "LN" => Ok(Some(StreetNamePostType::LANE)),
        "LGT" => Ok(Some(StreetNamePostType::LIGHT)),
        "LGTS" => Ok(Some(StreetNamePostType::LIGHTS)),
        "LF" => Ok(Some(StreetNamePostType::LOAF)),
        "LCK" => Ok(Some(StreetNamePostType::LOCK)),
        "LCKS" => Ok(Some(StreetNamePostType::LOCKS)),
        "LDG" => Ok(Some(StreetNamePostType::LODGE)),
        "LOOP" => Ok(Some(StreetNamePostType::LOOP)),
        "MALL" => Ok(Some(StreetNamePostType::MALL)),
        "MNR" => Ok(Some(StreetNamePostType::MANOR)),
        "MNRS" => Ok(Some(StreetNamePostType::MANORS)),
        "MDW" => Ok(Some(StreetNamePostType::MEADOW)),
        "MDWS" => Ok(Some(StreetNamePostType::MEADOWS)),
        "MEWS" => Ok(Some(StreetNamePostType::MEWS)),
        "ML" => Ok(Some(StreetNamePostType::MILL)),
        "MLS" => Ok(Some(StreetNamePostType::MILLS)),
        "MSN" => Ok(Some(StreetNamePostType::MISSION)),
        "MTWY" => Ok(Some(StreetNamePostType::MOTORWAY)),
        "MT" => Ok(Some(StreetNamePostType::MOUNT)),
        "MTN" => Ok(Some(StreetNamePostType::MOUNTAIN)),
        "MTNS" => Ok(Some(StreetNamePostType::MOUNTAINS)),
        "NCK" => Ok(Some(StreetNamePostType::NECK)),
        "ORCH" => Ok(Some(StreetNamePostType::ORCHARD)),
        "OVAL" => Ok(Some(StreetNamePostType::OVAL)),
        "OPAS" => Ok(Some(StreetNamePostType::OVERPASS)),
        "PARK" => Ok(Some(StreetNamePostType::PARK)),
        "PKWY" => Ok(Some(StreetNamePostType::PARKWAY)),
        "PASS" => Ok(Some(StreetNamePostType::PASS)),
        "PSGE" => Ok(Some(StreetNamePostType::PASSAGE)),
        "PATH" => Ok(Some(StreetNamePostType::PATH)),
        "PIKE" => Ok(Some(StreetNamePostType::PIKE)),
        "PNE" => Ok(Some(StreetNamePostType::PINE)),
        "PNES" => Ok(Some(StreetNamePostType::PINES)),
        "PL" => Ok(Some(StreetNamePostType::PLACE)),
        "PLN" => Ok(Some(StreetNamePostType::PLAIN)),
        "PLNS" => Ok(Some(StreetNamePostType::PLAINS)),
        "PLZ" => Ok(Some(StreetNamePostType::PLAZA)),
        "PT" => Ok(Some(StreetNamePostType::POINT)),
        "PTS" => Ok(Some(StreetNamePostType::POINTS)),
        "PRT" => Ok(Some(StreetNamePostType::PORT)),
        "PRTS" => Ok(Some(StreetNamePostType::PORTS)),
        "PR" => Ok(Some(StreetNamePostType::PRAIRIE)),
        "RADL" => Ok(Some(StreetNamePostType::RADIAL)),
        "RAMP" => Ok(Some(StreetNamePostType::RAMP)),
        "RNCH" => Ok(Some(StreetNamePostType::RANCH)),
        "RPD" => Ok(Some(StreetNamePostType::RAPID)),
        "RPDS" => Ok(Some(StreetNamePostType::RAPIDS)),
        "RST" => Ok(Some(StreetNamePostType::REST)),
        "RDG" => Ok(Some(StreetNamePostType::RIDGE)),
        "RDGS" => Ok(Some(StreetNamePostType::RIDGES)),
        "RIV" => Ok(Some(StreetNamePostType::RIVER)),
        "RD" => Ok(Some(StreetNamePostType::ROAD)),
        "RDS" => Ok(Some(StreetNamePostType::ROADS)),
        "RTE" => Ok(Some(StreetNamePostType::ROUTE)),
        "ROW" => Ok(Some(StreetNamePostType::ROW)),
        "RUE" => Ok(Some(StreetNamePostType::RUE)),
        "RUN" => Ok(Some(StreetNamePostType::RUN)),
        "SHL" => Ok(Some(StreetNamePostType::SHOAL)),
        "SHLS" => Ok(Some(StreetNamePostType::SHOALS)),
        "SHR" => Ok(Some(StreetNamePostType::SHORE)),
        "SHRS" => Ok(Some(StreetNamePostType::SHORES)),
        "SKWY" => Ok(Some(StreetNamePostType::SKYWAY)),
        "SPG" => Ok(Some(StreetNamePostType::SPRING)),
        "SPGS" => Ok(Some(StreetNamePostType::SPRINGS)),
        "SPUR" => Ok(Some(StreetNamePostType::SPUR)),
        "SQ" => Ok(Some(StreetNamePostType::SQUARE)),
        "SQS" => Ok(Some(StreetNamePostType::SQUARES)),
        "STA" => Ok(Some(StreetNamePostType::STATION)),
        "STRA" => Ok(Some(StreetNamePostType::STRAVENUE)),
        "STRM" => Ok(Some(StreetNamePostType::STREAM)),
        "ST" => Ok(Some(StreetNamePostType::STREET)),
        "STS" => Ok(Some(StreetNamePostType::STREETS)),
        "SMT" => Ok(Some(StreetNamePostType::SUMMIT)),
        "TER" => Ok(Some(StreetNamePostType::TERRACE)),
        "TRWY" => Ok(Some(StreetNamePostType::THROUGHWAY)),
        "TRCE" => Ok(Some(StreetNamePostType::TRACE)),
        "TRAK" => Ok(Some(StreetNamePostType::TRACK)),
        "TRFY" => Ok(Some(StreetNamePostType::TRAFFICWAY)),
        "TRL" => Ok(Some(StreetNamePostType::TRAIL)),
        "TRLR" => Ok(Some(StreetNamePostType::TRAILER)),
        "TUNL" => Ok(Some(StreetNamePostType::TUNNEL)),
        "TPKE" => Ok(Some(StreetNamePostType::TURNPIKE)),
        "UPAS" => Ok(Some(StreetNamePostType::UNDERPASS)),
        "UN" => Ok(Some(StreetNamePostType::UNION)),
        "UNS" => Ok(Some(StreetNamePostType::UNIONS)),
        "VLY" => Ok(Some(StreetNamePostType::VALLEY)),
        "VLYS" => Ok(Some(StreetNamePostType::VALLEYS)),
        "VIA" => Ok(Some(StreetNamePostType::VIADUCT)),
        "VW" => Ok(Some(StreetNamePostType::VIEW)),
        "VWS" => Ok(Some(StreetNamePostType::VIEWS)),
        "VLG" => Ok(Some(StreetNamePostType::VILLAGE)),
        "VLGS" => Ok(Some(StreetNamePostType::VILLAGES)),
        "VL" => Ok(Some(StreetNamePostType::VILLE)),
        "VIS" => Ok(Some(StreetNamePostType::VISTA)),
        "WALK" => Ok(Some(StreetNamePostType::WALK)),
        "WALL" => Ok(Some(StreetNamePostType::WALL)),
        "WAY" => Ok(Some(StreetNamePostType::WAY)),
        "WAYS" => Ok(Some(StreetNamePostType::WAYS)),
        "WL" => Ok(Some(StreetNamePostType::WELL)),
        "WLS" => Ok(Some(StreetNamePostType::WELLS)),
        _ => Ok(None),
    }
}

/// Deserialization function for post type abbreviations.
/// Calls [`match_mixed_post_type()`].
pub fn deserialize_mixed_post_type<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<StreetNamePostType>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    let result = match_mixed_post_type(intermediate);
    Ok(result)
}

/// Matches the target data against novel spellings of valid post types.  Add any missing spelling
/// variants to the match statement.
pub fn match_mixed_post_type(input: &str) -> Option<StreetNamePostType> {
    match input {
        "ALY" => Some(StreetNamePostType::ALLEY),
        "ANX" => Some(StreetNamePostType::ANEX),
        "ARC" => Some(StreetNamePostType::ARCADE),
        "AVE" => Some(StreetNamePostType::AVENUE),
        "Ave" => Some(StreetNamePostType::AVENUE),
        "AVENUE" => Some(StreetNamePostType::AVENUE),
        "BYU" => Some(StreetNamePostType::BAYOU),
        "BCH" => Some(StreetNamePostType::BEACH),
        "BND" => Some(StreetNamePostType::BEND),
        "BLF" => Some(StreetNamePostType::BLUFF),
        "BLFS" => Some(StreetNamePostType::BLUFFS),
        "BTM" => Some(StreetNamePostType::BOTTOM),
        "BLVD" => Some(StreetNamePostType::BOULEVARD),
        "Blvd" => Some(StreetNamePostType::BOULEVARD),
        "BOULEVARD" => Some(StreetNamePostType::BOULEVARD),
        "BR" => Some(StreetNamePostType::BRANCH),
        "BRG" => Some(StreetNamePostType::BRIDGE),
        "BRK" => Some(StreetNamePostType::BROOK),
        "BRKS" => Some(StreetNamePostType::BROOKS),
        "BG" => Some(StreetNamePostType::BURG),
        "BGS" => Some(StreetNamePostType::BURGS),
        "BYP" => Some(StreetNamePostType::BYPASS),
        "CP" => Some(StreetNamePostType::CAMP),
        "CYN" => Some(StreetNamePostType::CANYON),
        "CPE" => Some(StreetNamePostType::CAPE),
        "CSWY" => Some(StreetNamePostType::CAUSEWAY),
        "CTR" => Some(StreetNamePostType::CENTER),
        "CTRS" => Some(StreetNamePostType::CENTERS),
        "CIR" => Some(StreetNamePostType::CIRCLE),
        "CIRCLE" => Some(StreetNamePostType::CIRCLE),
        "Circle" => Some(StreetNamePostType::CIRCLE),
        "CIRS" => Some(StreetNamePostType::CIRCLES),
        "CLF" => Some(StreetNamePostType::CLIFF),
        "CLFS" => Some(StreetNamePostType::CLIFFS),
        "CLB" => Some(StreetNamePostType::CLUB),
        "CMN" => Some(StreetNamePostType::COMMON),
        "CMNS" => Some(StreetNamePostType::COMMONS),
        "COR" => Some(StreetNamePostType::CORNER),
        "CORS" => Some(StreetNamePostType::CORNERS),
        "CRSE" => Some(StreetNamePostType::COURSE),
        "CT" => Some(StreetNamePostType::COURT),
        "COURT" => Some(StreetNamePostType::COURT),
        "Ct" => Some(StreetNamePostType::COURT),
        "CTS" => Some(StreetNamePostType::COURTS),
        "CV" => Some(StreetNamePostType::COVE),
        "CVS" => Some(StreetNamePostType::COVES),
        "CRK" => Some(StreetNamePostType::CREEK),
        "CRES" => Some(StreetNamePostType::CRESCENT),
        "CRST" => Some(StreetNamePostType::CREST),
        "CREST" => Some(StreetNamePostType::CREST),
        "XING" => Some(StreetNamePostType::CROSSING),
        "XRD" => Some(StreetNamePostType::CROSSROAD),
        "XRDS" => Some(StreetNamePostType::CROSSROADS),
        "CURV" => Some(StreetNamePostType::CURVE),
        "CUTOFF" => Some(StreetNamePostType::CUTOFF),
        "DL" => Some(StreetNamePostType::DALE),
        "DM" => Some(StreetNamePostType::DAM),
        "DV" => Some(StreetNamePostType::DIVIDE),
        "DR" => Some(StreetNamePostType::DRIVE),
        "DRIVE" => Some(StreetNamePostType::DRIVE),
        "Dr" => Some(StreetNamePostType::DRIVE),
        "DR CUTOFF" => Some(StreetNamePostType::DriveCutoff),
        "DRS" => Some(StreetNamePostType::DRIVES),
        "EST" => Some(StreetNamePostType::ESTATE),
        "ESTS" => Some(StreetNamePostType::ESTATES),
        "EXPY" => Some(StreetNamePostType::EXPRESSWAY),
        "EXT" => Some(StreetNamePostType::EXTENSION),
        "EXTS" => Some(StreetNamePostType::EXTENSIONS),
        "FALL" => Some(StreetNamePostType::FALL),
        "FLS" => Some(StreetNamePostType::FALLS),
        "FRY" => Some(StreetNamePostType::FERRY),
        "FLD" => Some(StreetNamePostType::FIELD),
        "FLDS" => Some(StreetNamePostType::FIELDS),
        "FLT" => Some(StreetNamePostType::FLAT),
        "FLTS" => Some(StreetNamePostType::FLATS),
        "FRD" => Some(StreetNamePostType::FORD),
        "FRDS" => Some(StreetNamePostType::FORDS),
        "FRST" => Some(StreetNamePostType::FOREST),
        "FRG" => Some(StreetNamePostType::FORGE),
        "FRGS" => Some(StreetNamePostType::FORGES),
        "FRK" => Some(StreetNamePostType::FORK),
        "FRKS" => Some(StreetNamePostType::FORKS),
        "FT" => Some(StreetNamePostType::FORT),
        "FWY" => Some(StreetNamePostType::FREEWAY),
        "GDN" => Some(StreetNamePostType::GARDEN),
        "GARDEN" => Some(StreetNamePostType::GARDEN),
        "GDNS" => Some(StreetNamePostType::GARDENS),
        "GTWY" => Some(StreetNamePostType::GATEWAY),
        "GLN" => Some(StreetNamePostType::GLEN),
        "GLEN" => Some(StreetNamePostType::GLEN),
        "Glen" => Some(StreetNamePostType::GLEN),
        "glen" => Some(StreetNamePostType::GLEN),
        "GLNS" => Some(StreetNamePostType::GLENS),
        "GRN" => Some(StreetNamePostType::GREEN),
        "GRNS" => Some(StreetNamePostType::GREENS),
        "GRV" => Some(StreetNamePostType::GROVE),
        "GRVS" => Some(StreetNamePostType::GROVES),
        "HBR" => Some(StreetNamePostType::HARBOR),
        "HBRS" => Some(StreetNamePostType::HARBORS),
        "HVN" => Some(StreetNamePostType::HAVEN),
        "HTS" => Some(StreetNamePostType::HEIGHTS),
        "HWY" => Some(StreetNamePostType::HIGHWAY),
        "Hwy" => Some(StreetNamePostType::HIGHWAY),
        "HIGHWAY" => Some(StreetNamePostType::HIGHWAY),
        "HL" => Some(StreetNamePostType::HILL),
        "HLS" => Some(StreetNamePostType::HILLS),
        "HOLW" => Some(StreetNamePostType::HOLLOW),
        "INLT" => Some(StreetNamePostType::INLET),
        "IS" => Some(StreetNamePostType::ISLAND),
        "ISS" => Some(StreetNamePostType::ISLANDS),
        "ISLE" => Some(StreetNamePostType::ISLE),
        "JCT" => Some(StreetNamePostType::JUNCTION),
        "JCTS" => Some(StreetNamePostType::JUNCTIONS),
        "KY" => Some(StreetNamePostType::KEY),
        "KYS" => Some(StreetNamePostType::KEYS),
        "KNL" => Some(StreetNamePostType::KNOLL),
        "KNLS" => Some(StreetNamePostType::KNOLLS),
        "LK" => Some(StreetNamePostType::LAKE),
        "LKS" => Some(StreetNamePostType::LAKES),
        "LAND" => Some(StreetNamePostType::LAND),
        "LNDG" => Some(StreetNamePostType::LANDING),
        "LN" => Some(StreetNamePostType::LANE),
        "Ln" => Some(StreetNamePostType::LANE),
        "LANE" => Some(StreetNamePostType::LANE),
        "LGT" => Some(StreetNamePostType::LIGHT),
        "LGTS" => Some(StreetNamePostType::LIGHTS),
        "LF" => Some(StreetNamePostType::LOAF),
        "LCK" => Some(StreetNamePostType::LOCK),
        "LCKS" => Some(StreetNamePostType::LOCKS),
        "LDG" => Some(StreetNamePostType::LODGE),
        "LOOP" => Some(StreetNamePostType::LOOP),
        "MALL" => Some(StreetNamePostType::MALL),
        "MNR" => Some(StreetNamePostType::MANOR),
        "MNRS" => Some(StreetNamePostType::MANORS),
        "MDW" => Some(StreetNamePostType::MEADOW),
        "MDWS" => Some(StreetNamePostType::MEADOWS),
        "MEWS" => Some(StreetNamePostType::MEWS),
        "ML" => Some(StreetNamePostType::MILL),
        "MLS" => Some(StreetNamePostType::MILLS),
        "MSN" => Some(StreetNamePostType::MISSION),
        "MTWY" => Some(StreetNamePostType::MOTORWAY),
        "MT" => Some(StreetNamePostType::MOUNT),
        "MTN" => Some(StreetNamePostType::MOUNTAIN),
        "MTNS" => Some(StreetNamePostType::MOUNTAINS),
        "NCK" => Some(StreetNamePostType::NECK),
        "ORCH" => Some(StreetNamePostType::ORCHARD),
        "OVAL" => Some(StreetNamePostType::OVAL),
        "OPAS" => Some(StreetNamePostType::OVERPASS),
        "PARK" => Some(StreetNamePostType::PARK),
        "PKWY" => Some(StreetNamePostType::PARKWAY),
        "Pkwy" => Some(StreetNamePostType::PARKWAY),
        "PASS" => Some(StreetNamePostType::PASS),
        "PSGE" => Some(StreetNamePostType::PASSAGE),
        "PATH" => Some(StreetNamePostType::PATH),
        "PIKE" => Some(StreetNamePostType::PIKE),
        "PNE" => Some(StreetNamePostType::PINE),
        "PNES" => Some(StreetNamePostType::PINES),
        "PL" => Some(StreetNamePostType::PLACE),
        "PLACE" => Some(StreetNamePostType::PLACE),
        "PLN" => Some(StreetNamePostType::PLAIN),
        "PLNS" => Some(StreetNamePostType::PLAINS),
        "PLZ" => Some(StreetNamePostType::PLAZA),
        "PT" => Some(StreetNamePostType::POINT),
        "PTS" => Some(StreetNamePostType::POINTS),
        "PRT" => Some(StreetNamePostType::PORT),
        "PRTS" => Some(StreetNamePostType::PORTS),
        "PR" => Some(StreetNamePostType::PRAIRIE),
        "RADL" => Some(StreetNamePostType::RADIAL),
        "RAMP" => Some(StreetNamePostType::RAMP),
        "RNCH" => Some(StreetNamePostType::RANCH),
        "RPD" => Some(StreetNamePostType::RAPID),
        "RPDS" => Some(StreetNamePostType::RAPIDS),
        "RST" => Some(StreetNamePostType::REST),
        "RDG" => Some(StreetNamePostType::RIDGE),
        "RDGS" => Some(StreetNamePostType::RIDGES),
        "RIV" => Some(StreetNamePostType::RIVER),
        "RD" => Some(StreetNamePostType::ROAD),
        "ROAD" => Some(StreetNamePostType::ROAD),
        "Rd" => Some(StreetNamePostType::ROAD),
        "RDS" => Some(StreetNamePostType::ROADS),
        "RTE" => Some(StreetNamePostType::ROUTE),
        "ROW" => Some(StreetNamePostType::ROW),
        "RUE" => Some(StreetNamePostType::RUE),
        "RUN" => Some(StreetNamePostType::RUN),
        "SHL" => Some(StreetNamePostType::SHOAL),
        "SHLS" => Some(StreetNamePostType::SHOALS),
        "SHR" => Some(StreetNamePostType::SHORE),
        "SHRS" => Some(StreetNamePostType::SHORES),
        "SKWY" => Some(StreetNamePostType::SKYWAY),
        "SPG" => Some(StreetNamePostType::SPRING),
        "SPGS" => Some(StreetNamePostType::SPRINGS),
        "SPUR" => Some(StreetNamePostType::SPUR),
        "SQ" => Some(StreetNamePostType::SQUARE),
        "SQS" => Some(StreetNamePostType::SQUARES),
        "STA" => Some(StreetNamePostType::STATION),
        "STRA" => Some(StreetNamePostType::STRAVENUE),
        "STRM" => Some(StreetNamePostType::STREAM),
        "ST" => Some(StreetNamePostType::STREET),
        "St" => Some(StreetNamePostType::STREET),
        "Street" => Some(StreetNamePostType::STREET),
        "STREET" => Some(StreetNamePostType::STREET),
        "STS" => Some(StreetNamePostType::STREETS),
        "SMT" => Some(StreetNamePostType::SUMMIT),
        "TER" => Some(StreetNamePostType::TERRACE),
        "TRWY" => Some(StreetNamePostType::THROUGHWAY),
        "TRCE" => Some(StreetNamePostType::TRACE),
        "TRAK" => Some(StreetNamePostType::TRACK),
        "TRFY" => Some(StreetNamePostType::TRAFFICWAY),
        "TRL" => Some(StreetNamePostType::TRAIL),
        "TRLR" => Some(StreetNamePostType::TRAILER),
        "TUNL" => Some(StreetNamePostType::TUNNEL),
        "TPKE" => Some(StreetNamePostType::TURNPIKE),
        "UPAS" => Some(StreetNamePostType::UNDERPASS),
        "UN" => Some(StreetNamePostType::UNION),
        "UNS" => Some(StreetNamePostType::UNIONS),
        "VLY" => Some(StreetNamePostType::VALLEY),
        "VLYS" => Some(StreetNamePostType::VALLEYS),
        "VIA" => Some(StreetNamePostType::VIADUCT),
        "VW" => Some(StreetNamePostType::VIEW),
        "VIEW" => Some(StreetNamePostType::VIEW),
        "VWS" => Some(StreetNamePostType::VIEWS),
        "VLG" => Some(StreetNamePostType::VILLAGE),
        "VLGS" => Some(StreetNamePostType::VILLAGES),
        "VL" => Some(StreetNamePostType::VILLE),
        "VIS" => Some(StreetNamePostType::VISTA),
        "WALK" => Some(StreetNamePostType::WALK),
        "WALL" => Some(StreetNamePostType::WALL),
        "WAY" => Some(StreetNamePostType::WAY),
        "Way" => Some(StreetNamePostType::WAY),
        "WAYS" => Some(StreetNamePostType::WAYS),
        "WL" => Some(StreetNamePostType::WELL),
        "WLS" => Some(StreetNamePostType::WELLS),
        _ => None,
    }
}
