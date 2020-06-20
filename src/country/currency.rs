use std::fmt;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, EnumIter)]
pub enum CurrencyCode {
    None,
    // ENUM START
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AZN,
    BAM,
    BDT,
    BGN,
    BIF,
    BND,
    BOB,
    BRL,
    BTN,
    BWP,
    BYN,
    BZD,
    CAD,
    CDF,
    CHF,
    CLP,
    CNY,
    COP,
    CRC,
    CUP,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ERN,
    ETB,
    EUR,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    IRR,
    JOD,
    KES,
    KGS,
    KHR,
    KPW,
    KRW,
    KWD,
    KZT,
    LAK,
    LBP,
    LRD,
    LSL,
    LYD,
    MAD,
    MDL,
    MKD,
    MMK,
    MNT,
    MRU,
    MWK,
    MXN,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    OMR,
    PAB,
    PEN,
    PGK,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SDG,
    SEK,
    SLL,
    SOS,
    SRD,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TRY,
    TZS,
    UAH,
    UGX,
    USD,
    UYU,
    UZS,
    VES,
    VND,
    XAF,
    XOF,
    YER,
    ZAR,
    ZMW,
    ZWL,
    // ENUM END
}

impl CurrencyCode {
    pub fn is_none(&self) -> bool {
        match *self {
            CurrencyCode::None => true,
            _ => false,
        }
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
