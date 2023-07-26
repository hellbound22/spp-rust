#[derive(Debug)]
pub enum SPPError {
    MaxDataSizeExedded,
    MinDataLen,
    APIDLenMismatch(usize),
    SequenceControlLenMismatch,
    SecondaryHeaderNotPresent,
    MandatoryFieldNotPresent,
}