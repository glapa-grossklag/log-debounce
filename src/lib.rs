//! # `log-debounce`
//!
//! Debounced logging macros for the `log` crate.
//!
//! ## Usage
//!
//! ```rust
//! use log_debounce::{info_debounce, warn_once};
//! use std::time::Duration;
//!
//! # let salinity = 35.2;
//! info_debounce!(Duration::from_secs(60), "Measured current salinity as {:.1} ppt", salinity);
//!
//! warn_once!("Salinometer disconnected, no measurements are available");
//! ```

/// Logs a message at the info level, debounced by duration per callsite.
///
/// Subsequent calls to the same callsite within the specified duration will be silently dropped.
/// Each unique location in your code where this macro is invoked maintains its own debounce state.
///
/// # Example
///
/// ```rust
/// use log_debounce::info_debounce;
/// use std::time::Duration;
///
/// # let temperature = 23.5;
/// // Will log at most once per 30 seconds from this line
/// info_debounce!(Duration::from_secs(30), "Temperature: {:.1}°C", temperature);
/// ```
#[macro_export]
macro_rules! info_debounce {
    ($duration:expr, $($arg:tt)*) => {{
        use std::sync::LazyLock;
        use std::sync::Mutex;
        use std::time::Instant;

        static LAST: LazyLock<Mutex<Option<Instant>>> =
            LazyLock::new(|| Mutex::new(None));

        if let Ok(mut last) = LAST.lock() {
            let now = Instant::now();
            let should_log = last
                .map(|l| now.duration_since(l) >= $duration)
                .unwrap_or(true);

            if should_log {
                *last = Some(now);
                log::info!($($arg)*);
            }
        }
    }};
}

/// Logs a message at the info level exactly once per callsite.
///
/// The first invocation logs the message; all subsequent calls are silently dropped.
///
/// # Example
///
/// ```rust
/// use log_debounce::info_once;
///
/// // Logs only the first time this line is reached
/// info_once!("Startup complete");
/// ```
#[macro_export]
macro_rules! info_once {
    ($($arg:tt)*) => {
        $crate::info_debounce!(std::time::Duration::MAX, $($arg)*)
    };
}

/// Logs a message at the warn level, debounced by duration per callsite.
///
/// Subsequent calls to the same callsite within the specified duration will be silently dropped.
/// Each unique location in your code where this macro is invoked maintains its own debounce state.
///
/// # Example
///
/// ```rust
/// use log_debounce::warn_debounce;
/// use std::time::Duration;
///
/// # let queue_size = 1500;
/// // Will log at most once per 5 minutes from this line
/// warn_debounce!(Duration::from_secs(300), "Queue size high: {}", queue_size);
/// ```
#[macro_export]
macro_rules! warn_debounce {
    ($duration:expr, $($arg:tt)*) => {{
        use std::sync::LazyLock;
        use std::sync::Mutex;
        use std::time::Instant;

        static LAST: LazyLock<Mutex<Option<Instant>>> =
            LazyLock::new(|| Mutex::new(None));

        if let Ok(mut last) = LAST.lock() {
            let now = Instant::now();
            let should_log = last
                .map(|l| now.duration_since(l) >= $duration)
                .unwrap_or(true);

            if should_log {
                *last = Some(now);
                log::warn!($($arg)*);
            }
        }
    }};
}

/// Logs a message at the warn level exactly once per callsite.
///
/// The first invocation logs the message; all subsequent calls are silently dropped.
///
/// # Example
///
/// ```rust
/// use log_debounce::warn_once;
///
/// // Logs only the first time this line is reached
/// warn_once!("Deprecated configuration detected");
/// ```
#[macro_export]
macro_rules! warn_once {
    ($($arg:tt)*) => {
        $crate::warn_debounce!(std::time::Duration::MAX, $($arg)*)
    };
}

/// Logs a message at the error level, debounced by duration per callsite.
///
/// Subsequent calls to the same callsite within the specified duration will be silently dropped.
/// Each unique location in your code where this macro is invoked maintains its own debounce state.
///
/// # Example
///
/// ```rust
/// use log_debounce::error_debounce;
/// use std::time::Duration;
///
/// # let error = "connection timeout";
/// // Will log at most once per 10 seconds from this line
/// error_debounce!(Duration::from_secs(10), "Database error: {}", error);
/// ```
#[macro_export]
macro_rules! error_debounce {
    ($duration:expr, $($arg:tt)*) => {{
        use std::sync::LazyLock;
        use std::sync::Mutex;
        use std::time::Instant;

        static LAST: LazyLock<Mutex<Option<Instant>>> =
            LazyLock::new(|| Mutex::new(None));

        if let Ok(mut last) = LAST.lock() {
            let now = Instant::now();
            let should_log = last
                .map(|l| now.duration_since(l) >= $duration)
                .unwrap_or(true);

            if should_log {
                *last = Some(now);
                log::error!($($arg)*);
            }
        }
    }};
}

/// Logs a message at the error level exactly once per callsite.
///
/// The first invocation logs the message; all subsequent calls are silently dropped.
///
/// # Example
///
/// ```rust
/// use log_debounce::error_once;
///
/// // Logs only the first time this line is reached
/// error_once!("Failed to load critical configuration");
/// ```
#[macro_export]
macro_rules! error_once {
    ($($arg:tt)*) => {
        $crate::error_debounce!(std::time::Duration::MAX, $($arg)*)
    };
}

/// Logs a message at the debug level, debounced by duration per callsite.
///
/// Subsequent calls to the same callsite within the specified duration will be silently dropped.
/// Each unique location in your code where this macro is invoked maintains its own debounce state.
///
/// # Example
///
/// ```rust
/// use log_debounce::debug_debounce;
/// use std::time::Duration;
///
/// # let cache_hits = 42;
/// // Will log at most once per 60 seconds from this line
/// debug_debounce!(Duration::from_secs(60), "Cache hits: {}", cache_hits);
/// ```
#[macro_export]
macro_rules! debug_debounce {
    ($duration:expr, $($arg:tt)*) => {{
        use std::sync::LazyLock;
        use std::sync::Mutex;
        use std::time::Instant;

        static LAST: LazyLock<Mutex<Option<Instant>>> =
            LazyLock::new(|| Mutex::new(None));

        if let Ok(mut last) = LAST.lock() {
            let now = Instant::now();
            let should_log = last
                .map(|l| now.duration_since(l) >= $duration)
                .unwrap_or(true);

            if should_log {
                *last = Some(now);
                log::debug!($($arg)*);
            }
        }
    }};
}

/// Logs a message at the debug level exactly once per callsite.
///
/// The first invocation logs the message; all subsequent calls are silently dropped.
///
/// # Example
///
/// ```rust
/// use log_debounce::debug_once;
///
/// // Logs only the first time this line is reached
/// debug_once!("Entering hot path");
/// ```
#[macro_export]
macro_rules! debug_once {
    ($($arg:tt)*) => {
        $crate::debug_debounce!(std::time::Duration::MAX, $($arg)*)
    };
}

/// Logs a message at the trace level, debounced by duration per callsite.
///
/// Subsequent calls to the same callsite within the specified duration will be silently dropped.
/// Each unique location in your code where this macro is invoked maintains its own debounce state.
///
/// # Example
///
/// ```rust
/// use log_debounce::trace_debounce;
/// use std::time::Duration;
///
/// # let iteration = 1000;
/// // Will log at most once per 100ms from this line
/// trace_debounce!(Duration::from_millis(100), "Iteration {}", iteration);
/// ```
#[macro_export]
macro_rules! trace_debounce {
    ($duration:expr, $($arg:tt)*) => {{
        use std::sync::LazyLock;
        use std::sync::Mutex;
        use std::time::Instant;

        static LAST: LazyLock<Mutex<Option<Instant>>> =
            LazyLock::new(|| Mutex::new(None));

        if let Ok(mut last) = LAST.lock() {
            let now = Instant::now();
            let should_log = last
                .map(|l| now.duration_since(l) >= $duration)
                .unwrap_or(true);

            if should_log {
                *last = Some(now);
                log::trace!($($arg)*);
            }
        }
    }};
}

/// Logs a message at the trace level exactly once per callsite.
///
/// The first invocation logs the message; all subsequent calls are silently dropped.
///
/// # Example
///
/// ```rust
/// use log_debounce::trace_once;
///
/// // Logs only the first time this line is reached
/// trace_once!("Function called for first time");
/// ```
#[macro_export]
macro_rules! trace_once {
    ($($arg:tt)*) => {
        $crate::trace_debounce!(std::time::Duration::MAX, $($arg)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_debounce_fires_first_time() {
        info_debounce!(Duration::from_secs(1), "test message");
        // if this compiles and runs, it works
    }

    #[test]
    fn test_once_macro() {
        info_once!("only once");
        info_once!("only once");
        // both should compile, second is dropped
    }
}
