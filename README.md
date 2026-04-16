# Sx127x-Common
Common code for the Semtech [FSK](https://crates.io/crates/sx127x-fsk) and [LoRa](https://crates.io/crates/sx127x-lora) modems. The crate isn't terribly useful on its own but provides a
single source of truth in the downstream modem drivers.

### Cargo Features
- `async` (default): async implementation
- `sync`: sync implementation