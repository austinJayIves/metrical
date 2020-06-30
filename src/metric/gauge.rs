use crate::metric::{MetricGenerator, MetricData, MetricType};
use std::time::SystemTime;
use crate::shared::collector;
use std::ops::{Add, AddAssign};
use std::fmt::Display;
use std::fmt::Formatter;


pub struct GaugeBuilder {
    name: String,
    namespace: Option<String>
}

impl GaugeBuilder {
    pub fn new(name: String) -> Self {
        GaugeBuilder{
            name,
            namespace: Option::None
        }
    }

    pub fn namespace(&mut self, namespace: Option<String>) -> &Self {
        self.namespace = namespace;
        self
    }

    pub fn build(&self) -> Gauge {
        Gauge{
            name: self.name.clone(),
            namespace: self.namespace.clone(),
            value: GaugeOptions::Increase(0)
        }
    }
}


/// A metric used to measure both increasing and decreasing resources.
///
/// # Examples:
/// - A measure of the number of CPUs dedicated to a task
/// - The number of machines running a particular script
/// - The amount of memory in use by a function
pub struct Gauge {
    name: String,
    namespace: Option<String>,
    value: GaugeOptions
}

#[derive(Clone, PartialEq, Debug)]
pub enum GaugeOptions {
    Increase(u32),
    Decrease(u32),
    Set(u32)
}

impl Add for GaugeOptions {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            GaugeOptions::Increase(v) => {
                match rhs {
                    GaugeOptions::Increase(v2) => GaugeOptions::Increase(v + v2),
                    GaugeOptions::Decrease(v2) => if v2 > v {
                        GaugeOptions::Decrease(v2 - v)
                    } else {
                        GaugeOptions::Increase(v - v2)
                    },
                    GaugeOptions::Set(v2) => GaugeOptions::Set(v2)
                }
            },
            GaugeOptions::Decrease(v) => {
                match rhs {
                    GaugeOptions::Increase(v2) => if v2 > v {
                        GaugeOptions::Increase(v2 - v)
                    } else {
                        GaugeOptions::Decrease(v - v2)
                    },
                    GaugeOptions::Decrease(v2) => GaugeOptions::Decrease(v2 + v),
                    GaugeOptions::Set(v2) => GaugeOptions::Set(v2)
                }
            },
            GaugeOptions::Set(v) => {
                match rhs {
                    GaugeOptions::Increase(v2) => GaugeOptions::Set(v + v2),
                    GaugeOptions::Decrease(v2) => if v2 > v {
                        GaugeOptions::Set(0)
                    } else {
                        GaugeOptions::Set(v - v2)
                    },
                    GaugeOptions::Set(v2) => GaugeOptions::Set(v2)
                }
            }
        }
    }
}

impl AddAssign for GaugeOptions {
    fn add_assign(&mut self, rhs: Self) {
        let x = self.clone();
        *self = x + rhs
    }
}

impl Display for GaugeOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GaugeOptions::Increase(v) => f.write_str(format!("+{}", v).as_str()),
            GaugeOptions::Decrease(v) => f.write_str(format!("-{}", v).as_str()),
            GaugeOptions::Set(v) => f.write_str(v.to_string().as_str())
        }
    }
}

impl Gauge {
    /// Increment the value of a gauge
    pub fn increment(&mut self, value: u32) {
        self.value += GaugeOptions::Increase(value);
    }

    /// Decrement the value of a gauge
    pub fn decrement(&mut self, value: u32) {
        self.value += GaugeOptions::Decrease(value);
    }

    /// Set the value of a gauge to a specific value
    pub fn set(&mut self, value: u32) {
        self.value = GaugeOptions::Set(value);

        // Set values are sent right away
        collector().send(self.metric());
        self.value = GaugeOptions::Increase(0);
    }
}

impl MetricGenerator for Gauge {
    fn metric(&self) -> MetricData {
        let occurred = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        MetricData {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
            occurred,
            metric: MetricType::Gauge(self.value.clone())
        }
    }
}

impl Drop for Gauge {
    fn drop(&mut self) {
        collector().send(self.metric());
    }
}

#[cfg(test)]
mod test {
    use crate::metric::gauge::{GaugeBuilder, GaugeOptions};
    use crate::metric::{MetricGenerator, MetricData, MetricType};

    #[test]
    pub fn it_should_go_up_and_down() {
        let mut gauge = GaugeBuilder::new("HelloGauge".to_owned())
            .namespace(Option::None)
            .build();

        gauge.increment(1);

        let metric = gauge.metric();
        assert_eq!(metric, MetricData {
            namespace: Option::None,
            occurred: metric.occurred,
            name: "HelloGauge".to_owned(),
            metric: MetricType::Gauge(GaugeOptions::Increase(1))
        });

        gauge.decrement(1);

        let metric = gauge.metric();
        assert_eq!(metric, MetricData {
            namespace: Option::None,
            occurred: metric.occurred,
            name: "HelloGauge".to_owned(),
            metric: MetricType::Gauge(GaugeOptions::Increase(0))
        });

        gauge.decrement(1);

        let metric = gauge.metric();
        assert_eq!(metric, MetricData {
            namespace: Option::None,
            occurred: metric.occurred,
            name: "HelloGauge".to_owned(),
            metric: MetricType::Gauge(GaugeOptions::Decrease(1))
        });

        std::mem::forget(gauge);
    }
}