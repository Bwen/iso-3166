use std::fmt;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, EnumIter)]
pub enum Alpha2 {
    None,
    // ENUM START
    AA,
    AF,
    AK,
    AM,
    AR,
    AS,
    AV,
    AY,
    AZ,
    BA,
    BE,
    BG,
    BM,
    BN,
    BR,
    BS,
    CA,
    CE,
    CO,
    CS,
    CU,
    CV,
    CY,
    DA,
    DE,
    DZ,
    EE,
    EL,
    EN,
    ES,
    ET,
    EU,
    FA,
    FF,
    FI,
    FO,
    FR,
    FY,
    GA,
    GD,
    GL,
    GN,
    GU,
    HA,
    HE,
    HI,
    HO,
    HR,
    HT,
    HU,
    HY,
    HZ,
    ID,
    IG,
    IT,
    IU,
    JV,
    KA,
    KG,
    KK,
    KM,
    KN,
    KO,
    KR,
    KS,
    KU,
    KV,
    KY,
    LA,
    LB,
    LG,
    LN,
    LO,
    LT,
    LV,
    MK,
    ML,
    MN,
    MR,
    MS,
    MY,
    NB,
    ND,
    NE,
    NL,
    NN,
    NO,
    NR,
    NY,
    OC,
    OM,
    OR,
    PA,
    PL,
    PS,
    PT,
    QU,
    RM,
    RN,
    RO,
    RU,
    RW,
    SA,
    SC,
    SD,
    SE,
    SG,
    SH,
    SK,
    SL,
    SN,
    SO,
    SQ,
    SR,
    SS,
    ST,
    SV,
    SW,
    TA,
    TE,
    TG,
    TH,
    TI,
    TK,
    TN,
    TR,
    TS,
    TT,
    TW,
    UG,
    UK,
    UR,
    UZ,
    VE,
    VI,
    WO,
    XH,
    YO,
    ZA,
    ZH,
    ZU,
    // ENUM END
}

impl Alpha2 {
    pub fn is_none(&self) -> bool {
        match *self {
            Alpha2::None => true,
            _ => false,
        }
    }
}

impl fmt::Display for Alpha2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
