# `log-debounce`

[Debounced](https://developer.mozilla.org/en-US/docs/Glossary/Debounce) logging macros for the `log` crate.

## Usage

```rust
use log_debounce::{info_debounce, warn_once};
use std::time::Duration;

# let salinity = 35.2;
info_debounce!(Duration::from_secs(60), "Measured current salinity as {:.1} ppt", salinity);

warn_once!("Salinometer disconnected, no measurements are available");
```