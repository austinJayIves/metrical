pub trait MetricGenerator {
    fn metric(&self) -> MetricData;
}

#[derive(Debug, PartialEq, Clone)]
pub struct MetricData {
    namespace: Option<String>,
    name: String,
    occurred: u64,
    metric: MetricType
}

impl MetricData {
    pub fn path(&self) -> String {
        self.namespace.clone().map_or_else(
            || self.name.clone(),
            |namespace| format!("{}.{}", namespace, self.name)
        )
    }

    pub fn metric(&self) -> &MetricType {
        return &self.metric
    }

    pub fn occurred(&self) -> u64 {
        return self.occurred
    }
}

#[cfg(test)]
pub fn metric_test_data() -> [MetricData; 4] {
    [
        MetricData{name: "HelloTimer".to_owned(), occurred: 1,
            namespace: Option::Some("test".to_owned()), metric: MetricType::Timer(1005) },
        MetricData{name: "HelloCounter".to_owned(), occurred: 2,
            namespace: Option::Some("test".to_owned()), metric: MetricType::Counter(12) },
        MetricData{name: "HelloGauge".to_owned(), occurred: 3,
            namespace: Option::Some("test".to_owned()), metric: MetricType::Gauge(GaugeOptions::Increase(13))},
        MetricData{name: "HelloGauge".to_owned(), occurred: 4,
            namespace: Option::Some("test".to_owned()), metric: MetricType::Gauge(GaugeOptions::Decrease(2)) },
    ]
}

#[derive(Debug, PartialEq, Clone)]
pub enum MetricType {
    Counter(u32),
    Timer(u128),
    Gauge(GaugeOptions)
}

#[derive(Clone)]
pub struct Namespace(pub String);

impl Namespace {
    pub fn new(namespace: String) -> Self {
        Namespace(namespace)
    }

    pub fn namespace(&self, namespace: String) -> Self {
        Namespace::new(format!("{}.{}", self.0, namespace))
    }

    pub fn counter(&self, name: String) -> Counter {
        CounterBuilder::new(name).namespace(Option::Some(self.0.clone())).build()
    }

    pub fn timer(&self, name: String) -> Timer {
        TimerBuilder::new(name).namespace(Option::Some(self.0.clone())).build()
    }

    pub fn gauge(&self, name: String) -> Gauge {
        GaugeBuilder::new(name).namespace(Option::Some(self.0.clone())).build()
    }
}

mod counter;
mod timer;
mod gauge;

pub use counter::{Counter, CounterBuilder};
pub use timer::{Timer, TimerBuilder};
pub use gauge::{Gauge, GaugeBuilder};
use crate::metric::gauge::GaugeOptions;
