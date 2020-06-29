mod metric;
pub use metric::{Counter, Timer, Gauge};
use crate::metric::Namespace;
use std::error::Error;
use std::fmt::Display;
use serde::export::Formatter;

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

pub fn flush() {
    shared::collector().flush();
}
