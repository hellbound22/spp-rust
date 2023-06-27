#[derive(Debug)]
pub enum SPPError {
    MaxDataSizeExedded,
    MinDataLen,
    APIDLenMismatch,
    SequenceControlLenMismatch,
    SecondaryHeaderNotPresent,
    MandatoryFieldNotPresent,
}