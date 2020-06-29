use crate::configuration::{Configuration, FlushConfigurationOptions};
use crate::metric::{MetricData, Namespace};
use std::time::{SystemTime, Duration};

struct MetricBuffer{
    buffer: Option<Vec<MetricData>>,
    flush_settings: FlushConfigurationOptions,
    last_flush: Option<SystemTime>
}

pub trait Collector {
    fn send(&mut self, data: MetricData);
    fn flush(&mut self);
    fn namespace(&self) -> Option<Namespace>;
}

impl MetricBuffer {
    fn new_buffer(flush_after_amount: Option<usize>) -> Vec<MetricData> {
        match flush_after_amount {
            Some(v) => Vec::with_capacity(v),
            None => Vec::new()
        }
    }

    fn new(flush_settings: FlushConfigurationOptions) -> MetricBuffer {
        MetricBuffer{
            buffer: Some(MetricBuffer::new_buffer(flush_settings.flush_after_amount)),
            flush_settings,
            last_flush: None
        }
    }

    pub fn flush(&mut self) -> Vec<MetricData> {
        let data = match self.buffer.take() {
            Some(buffer) => buffer,
            None => Vec::new()
        };

        self.buffer = Some(Vec::new());
        self.last_flush = Some(SystemTime::now());

        data
    }

    pub fn flush_ready(&self) -> bool {
        (match self.flush_settings.flush_after_amount {
            Some(v) => self.buffer.as_ref().unwrap().len() >= v,
            None => false
        }) || (match self.flush_settings.flush_after_interval {
            Some(interval) => match self.last_flush {
                Some(last_flush) => {
                    let latest_interval = SystemTime::now()
                        .duration_since(last_flush)
                        .unwrap_or(Duration::from_secs(0))
                        .as_secs();

                    latest_interval > interval
                },
                None => false
            },
            None => false
        })
    }

    pub fn submit(&mut self, data: MetricData) {
        self.buffer.as_mut().unwrap().push(data);
    }
}

impl Drop for BufferedCollector {
    fn drop(&mut self) {
        let data = self.buffer.flush();
        self.config.send(data);
    }
}

pub struct BufferedCollector {
    config: Configuration,
    buffer: MetricBuffer,
}

impl BufferedCollector {
    pub fn new(config: Configuration) -> BufferedCollector {
        let flush_settings = config.options.flush.clone();

        BufferedCollector {
            config,
            buffer: MetricBuffer::new(flush_settings)
        }
    }

}

impl Collector for BufferedCollector {
    fn send(&mut self, metric: MetricData) {
        self.buffer.submit(metric);

        if self.buffer.flush_ready() {
            self.flush()
        }
    }

    fn flush(&mut self) {
        let data = self.buffer.flush();
        self.config.send(data);
    }

    fn namespace(&self) -> Option<Namespace> {
        self.config.options.namespace.clone()
    }
}

