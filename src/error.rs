#[derive(Debug)]
pub enum Sx127xError<SPI> {
    InvalidFdev,
    InvalidInput,
    InvalidPayloadLength,
    InvalidPreambleLength,
    InvalidState,
    InvalidSymbolTimeout,
    PacketTermination,
    SPI(SPI),
}