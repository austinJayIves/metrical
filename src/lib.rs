//! Metrical is a library for quickly exporting metrics to Graphite or StatsD.
//!
//! Metrical supports 3 different protocols for sending data:
//! - Statsd
//! - Graphite Uncompressed
//! - Graphite Compressed (Pickled)
//!
//! Additionally, it supports sending via TCP or UDP.
//!
//! # Features
//! Metrical exports the following features:
//! - **pickle** - This feature is required to use the graphite pickled protocol.
//! This features is not on by default. The protocol will pickle using
//! pickle version 3.
mod metric;
pub use metric::{Counter, Timer, Gauge};
use crate::metric::Namespace;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

/// Describes an error associated with the metrical library.
///
/// Currently the only metrical error that can occur is `ConfigurationInvalid`
#[derive(Debug)]
pub enum MetricalError {
    ConfigurationInvalid(&'static str)
}

impl Display for MetricalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricalError::ConfigurationInvalid(msg) => {
                f.write_str(format!("Configuration Error: {}", msg).as_str())
            }
        }
    }
}

impl Error for MetricalError {}

mod connection;
mod protocol;
mod configuration;
pub use configuration::{Configuration, ConfigurationBuilder};
mod collector;
mod shared;
mod init;
pub use init::{from_env, from_config};

/// Create a counter to count a metric that always increases.
///
/// # Example
/// ```
/// use metrical::counter;
///
/// pub fn do_something() {
/// // Count the number of times this function is called
/// let mut cnt = counter("do_something".to_owned());
/// cnt.increment(1);
///
/// //...
/// }
/// ```
pub fn counter(name: String) -> metric::Counter {
    let mut ctor = metric::CounterBuilder::new(name);

    match shared::collector().namespace() {
        Some(Namespace(path)) => {
            ctor.namespace(Some(path)).build()
        },
        None => {
            ctor.build()
        }
    }
}


/// Create a gauge to monitor both incrementing and decrementing of a statistic
///
/// # Example
/// ```
/// use metrical::gauge;
/// use std::thread::JoinHandle;
///
/// struct Threader{
///     threads: Vec<JoinHandle<()>>
/// }
///
/// impl Threader {
///     pub fn add_resource(&mut self) {
///         let mut gauge = gauge("NumThreadsInUse".to_owned());
///         gauge.increment(1);
///
///         self.threads.push(std::thread::spawn(||
///             println!("This does nothing...")
///         ));
///     }
///
///     pub fn terminate_resource(&mut self) {
///         let mut gauge = gauge("NumThreadsInUse".to_owned());
///         gauge.decrement(1);
///
///         self.threads.pop().map(|thread| thread.join());
///     }
/// }
/// ```
pub fn gauge(name: String) -> metric::Gauge {
    let mut ctor = metric::GaugeBuilder::new(name);

    match shared::collector().namespace() {
        Some(Namespace(path)) => {
            ctor.namespace(Some(path)).build()
        },
        None => {
            ctor.build()
        }
    }
}

/// Create a timer to monitor time needed to perform a specific action
///
/// # Example
/// ```
/// use metrical::timer;
///
/// pub fn do_task() {
///     let timer = timer("Perform long task".to_owned());
///
///     // Perform some task
///
///     // You can drop the timer manually to force the end of the timer.
///     drop(timer);
/// }
/// ```
pub fn timer(name: String) -> metric::Timer {
    let mut ctor = metric::TimerBuilder::new(name);

    match shared::collector().namespace() {
        Some(Namespace(path)) => {
            ctor.namespace(Some(path)).build()
        },
        None => {
            ctor.build()
        }
    }
}

///  Extend the namespace used by created metrics
///
/// # Example:
/// Suppose the default namespace was 'foo'
///
/// ```
/// use metrical::{namespace, counter};
///
/// pub fn namespaced_task() {
///     let namespace = namespace("bar".to_owned());
///
///     let counter = counter("baz".to_owned()); // Path: foo.baz
///     let counter = namespace.counter("baz".to_owned()); // Path: foo.bar.baz
/// }
/// ```
///
pub fn namespace(namespace: String) -> metric::Namespace {
    let global_namespace = shared::collector().namespace();
    match global_namespace {
        Some(Namespace(path)) => {
            Namespace::new(format!("{}.{}", path, namespace))
        },
        None => {
            Namespace::new(namespace)
        }
    }
}

/// Force any buffered metrics to be published.
///
/// It is a good idea to execute this at the end of your program.
///
/// # Example
/// ```
/// use metrical::{from_env, counter, flush};
///
/// pub fn main() {
///     from_env(None).expect("Unable to configure metrical");
///
///     let counter = counter("runs".to_owned());
///
///     // Ensure counter is dropped before the flush
///     drop(counter);
///
///     // Ensure the counter is published
///     flush();
/// }
/// ```
pub fn flush() {
    shared::collector().flush();
}
