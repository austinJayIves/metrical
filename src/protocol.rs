use crate::metric::{MetricData, MetricType};
use serde_pickle::ser;

pub enum Protocol {
    StatsD,
    Graphite(Compression)
}

type PickleRecord = (String, (String, String));

impl Protocol {
    pub fn serialized_statsd_record(metric: MetricData) -> Vec<u8> {
        let path = metric.path();
        match metric.metric() {
            MetricType::Counter(count) => format!("{}:{}|c\n", path, count).into_bytes(),
            MetricType::Timer(elapsed) => format!("{}:{}|ms\n", path, elapsed).into_bytes(),
            MetricType::Gauge(gauge) => format!("{}:{}|g\n", path, gauge).into_bytes()
        }
    }

    pub fn serialize_statsd<I>(metrics: I) -> Vec<u8> where
    I: IntoIterator<Item=MetricData>
    {
        let mut data: Vec<u8> = metrics
            .into_iter()
            .map(Protocol::serialized_statsd_record)
            .flatten()
            .collect();
        data.pop();
        data
    }

    pub fn serialized_graphite_record(metric: MetricData) -> Vec<u8> {
        let occurred = metric.occurred();
        let path = metric.path();
        match metric.metric() {
            MetricType::Counter(count) => format!("{} {} {}", path, count, occurred).into_bytes(),
            MetricType::Timer(elapsed) => format!("{} {} {}", path, elapsed, occurred).into_bytes(),
            MetricType::Gauge(gauge) => format!("{} {} {}", path, gauge, occurred).into_bytes()
        }
    }

    pub fn serialize_graphite_uncompressed<I>(metrics: I) -> Vec<u8> where
        I: IntoIterator<Item=MetricData>
    {
        metrics
            .into_iter()
            .flat_map(Protocol::serialized_graphite_record)
            .collect()
    }

    pub fn pickle_tuple(metric: MetricData) -> PickleRecord {
        let occurred = metric.occurred().to_string();
        let path = metric.path();
        let value = match metric.metric() {
            MetricType::Counter(count) => count.to_string(),
            MetricType::Timer(elapsed) => elapsed.to_string(),
            MetricType::Gauge(gauge) => gauge.to_string()
        };

        return (path, (value, occurred))
    }

    pub fn serialize_graphite_pickled<I>(metrics: I) -> Vec<u8> where
        I: IntoIterator<Item=MetricData>
    {
        use byteorder::{ByteOrder, BigEndian};

        let data: Vec<PickleRecord> = metrics.into_iter().map(
            Protocol::pickle_tuple
        ).collect();

        let data = ser::to_vec(&data, true).unwrap_or(Vec::new());

        let size = data.len() as u32;
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, size);
        buf.to_vec().into_iter().chain(data.into_iter()).collect::<Vec<_>>()
    }

    pub fn serialize_data<I>(&self, metrics: I) -> Vec<u8>
        where I: IntoIterator<Item=MetricData>
    {
        match self {
            Protocol::StatsD => Protocol::serialize_statsd(metrics),
            Protocol::Graphite(Compression::Pickled) => Protocol::serialize_graphite_pickled(metrics),
            Protocol::Graphite(Compression::Uncompressed) =>
                Protocol::serialize_graphite_uncompressed(metrics)
        }
    }
}

impl Protocol {
    pub fn serialize<I>(&self, data: I) -> Vec<Vec<u8>> where I: IntoIterator<Item=MetricData> {
        let mut data = data.into_iter().peekable();

        let num_in_packet = match self {
            Protocol::StatsD => 14,
            Protocol::Graphite(Compression::Uncompressed) => 1,
            Protocol::Graphite(Compression::Pickled) => 14
        };

        let mut packet_bodies = Vec::new();
        while data.peek().is_some() {
            let chunk: Vec<_> = data.by_ref().take(num_in_packet).collect();

            packet_bodies.push(self.serialize_data(chunk));
        }

        packet_bodies
    }
}

pub enum NetworkProtocol {
    UDP,
    TCP
}

pub enum Compression {
    Pickled,
    Uncompressed,
}

#[cfg(test)]
mod test {
    mod uncompressed {
        use crate::metric::{MetricData, metric_test_data};
        use crate::protocol::{Protocol, Compression};

        #[test]
        pub fn it_should_handle_one_metric_per_entry() {
            let data = metric_test_data().to_vec();
            let protocol = Protocol::Graphite(Compression::Uncompressed);
            let result: Vec<Vec<u8>> = protocol.serialize(data);

            assert_eq!(result.len(), 4);

            let payload1 = result[0].clone();
            assert_eq!(payload1, b"test.HelloTimer 1005 1".to_vec());

            let payload2 = result[1].clone();
            assert_eq!(payload2, b"test.HelloCounter 12 2".to_vec());

            let payload3 = result[2].clone();
            assert_eq!(payload3, b"test.HelloGauge 13 3".to_vec());

            let payload4 = result[3].clone();
            assert_eq!(payload4, b"test.HelloGauge -2 4".to_vec());
        }
    }

    mod compressed {
        use crate::metric::metric_test_data;
        use crate::protocol::{Protocol, Compression};
        use std::io::Write;

        #[test]
        pub fn it_should_pickle_properly() -> Result<(), Box<dyn std::error::Error>> {
            let data = metric_test_data().to_vec();
            let protocol = Protocol::Graphite(Compression::Pickled);

            let pickle_data: Vec<Vec<u8>> = protocol.serialize(data);
            let data_point = &pickle_data[0][4..];

            assert_eq!(pickle_data.len(), 1);
            assert_eq!(data_point.len(), 158);

            Ok(())
        }
    }

    mod statsd {
        use crate::metric::metric_test_data;
        use crate::protocol::Protocol;

        #[test]
        pub fn it_should_construct_properly() {
            let data = metric_test_data().to_vec();
            let protocol = Protocol::StatsD;

            let data = protocol.serialize(data);

            assert_eq!(data.len(), 1);
            assert_eq!(data[0].to_vec(), b"test.HelloTimer:1005|ms\ntest.HelloCounter:12|c\ntest.HelloGauge:13|g\ntest.HelloGauge:-2|g".to_vec());
        }
    }
}