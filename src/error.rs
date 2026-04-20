#[derive(Debug)]
pub enum Sx127xError<SPI> {
    // TODO remove InvalidX in favor of InvalidInput
    InvalidFdev,
    InvalidInput,
    InvalidPayloadLength,
    InvalidPreambleLength,
    InvalidState,
    InvalidSymbolTimeout,
    PacketTermination,
    SPI(SPI),
}