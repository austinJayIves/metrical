use crate::metric::{MetricGenerator, MetricData, MetricType};
use std::time::SystemTime;
use crate::shared::collector;


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
            value: 0
        }
    }
}


pub struct Gauge {
    name: String,
    namespace: Option<String>,
    value: i32
}

impl Gauge {
    pub fn increment(&mut self, value: i32) {
        self.value += value;
    }

    pub fn decrement(&mut self, value: i32) {
        self.value -= value;
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
            metric: MetricType::Gauge(self.value)
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
    use crate::metric::gauge::GaugeBuilder;
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
            metric: MetricType::Gauge(1)
        });

        gauge.decrement(1);

        let metric = gauge.metric();
        assert_eq!(metric, MetricData {
            namespace: Option::None,
            occurred: metric.occurred,
            name: "HelloGauge".to_owned(),
            metric: MetricType::Gauge(0)
        });

        gauge.decrement(1);

        let metric = gauge.metric();
        assert_eq!(metric, MetricData {
            namespace: Option::None,
            occurred: metric.occurred,
            name: "HelloGauge".to_owned(),
            metric: MetricType::Gauge(-1)
        });

        std::mem::forget(gauge);
    }
}