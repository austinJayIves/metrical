use crate::metric::{MetricData, MetricGenerator, MetricType};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::shared::collector;

pub struct CounterBuilder {
    name: String,
    namespace: Option<String>
}

impl CounterBuilder {
    pub fn new(name: String) -> CounterBuilder {
        CounterBuilder{ name, namespace: Option::None }
    }

    pub fn namespace(&mut self, namespace: Option<String>) -> &Self {
        self.namespace = namespace;
        self
    }

    pub fn build(&self) -> Counter {
        Counter {
            count: 0,
            name: self.name.clone(),
            namespace: self.namespace.clone()
        }
    }
}

#[derive(Debug)]
pub struct Counter {
    count: u32,
    name: String,
    namespace: Option<String>
}

impl Counter {
    pub fn increment(&mut self, amount: u32) -> &Self {
        self.count += amount;
        self
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        collector().send(self.metric());
    }
}

impl MetricGenerator for Counter {
    fn metric(&self) -> MetricData {
        let namespace = self.namespace.clone().take();
        let name = self.name.clone();

        let occurred = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        MetricData{
            namespace,
            name,
            occurred,
            metric: MetricType::Counter(self.count)
        }
    }
}


#[cfg(test)]
mod test {
    mod counter {
        use crate::metric::{MetricGenerator, MetricData, MetricType};
        use crate::metric::counter::CounterBuilder;

        #[test]
        pub fn test_increment() {
            let mut counter = CounterBuilder::new("HelloCounter".to_owned())
                .namespace(Option::None)
                .build();

            counter.increment(1);
            counter.increment(1);

            let result = counter.metric();

            assert_eq!(
                result,
                MetricData {
                    namespace: Option::None,
                    occurred: result.occurred,
                    name: "HelloCounter".to_owned(),
                    metric: MetricType::Counter(2)
                }
            );

            std::mem::forget(counter);
        }

        #[test]
        pub fn it_should_work_after_many_additions() {
            let mut counter = CounterBuilder::new("HelloCounter".to_owned())
                .namespace(Option::None)
                .build();

            for i in 1..100 {
                counter.increment(i);
            }

            let result = counter.metric();
            assert_eq!(
                result,
                MetricData{
                    namespace: Option::None,
                    occurred: result.occurred,
                    name: "HelloCounter".to_owned(),
                    metric: MetricType::Counter(4950)
                }
            );

            std::mem::forget(counter);
        }
    }
}
