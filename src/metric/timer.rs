use crate::metric::{MetricGenerator, MetricData, MetricType};
use std::time::{Instant, SystemTime};
use crate::shared::collector;

pub struct TimerBuilder {
    name: String,
    namespace: Option<String>
}

impl TimerBuilder {
    pub fn new(name: String) -> TimerBuilder {
        TimerBuilder{
            name,
            namespace: Option::None
        }
    }

    pub fn namespace(&mut self, namespace: Option<String>) -> &Self {
        self.namespace = namespace;
        self
    }

    pub fn build(&self) -> Timer {
        Timer {
            start: Instant::now(),
            stop: Option::None,
            name: self.name.clone(),
            namespace: self.namespace.clone()
        }
    }
}

/// A metric used to measure the time elapsed during a task or process.
///
/// # Examples:
/// - Timing a critical query to a database
/// - Timing how long it takes to return a message to a user once an HTTP request has been received
///
pub struct Timer {
    start: Instant,
    stop: Option<Instant>,
    name: String,
    namespace: Option<String>
}

impl Drop for Timer {
    fn drop(&mut self) {
        collector().send(self.metric());
    }
}

impl MetricGenerator for Timer {
    fn metric(&self) -> MetricData {
        let elapsed = match self.stop {
            Some(instant) => {
                instant.duration_since(self.start).as_millis()
            }
            None => {
                Instant::now().duration_since(self.start).as_millis()
            }
        };

        let namespace = self.namespace.clone().take();
        let name = self.name.clone();

        let occurred = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        MetricData{
            namespace,
            name,
            occurred,
            metric: MetricType::Timer(elapsed)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::metric::timer::TimerBuilder;
    use crate::metric::{MetricGenerator, MetricType};

    #[test]
    pub fn it_should_correctly_give_time() {
        let timer = TimerBuilder::new("HelloTimer".to_owned())
            .namespace(Option::None)
            .build();

        std::thread::sleep(std::time::Duration::from_secs(1));
        let top_bound = 1010;
        let lower_bound = 990;

        let metric = timer.metric();

        let namespace = metric.namespace.clone();
        let name = metric.name.clone();

        assert_eq!(Option::None, namespace);
        assert_eq!("HelloTimer".to_owned(), name);
        if let MetricType::Timer(instant) = metric.metric {
            assert!(instant < top_bound);
            assert!(instant > lower_bound);
        } else {
            assert!(false, "Timer did not return MetricType Timer")
        }

        std::mem::forget(timer);
    }
}