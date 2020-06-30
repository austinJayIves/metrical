# Metrical

## A Graphite and Statsd metric aggregation client for Rust

Metrical makes it easy to add and track metrics from Rust services.

Metrical supports the three following protocols
- Statsd (text)
- Graphite (uncompressed)
- Graphite Compressed (pickled)

Additionally, Metrical can send metrics via either UDP or TCP.

## Getting Started
Metrical is fairly easy to get going. To configure Metrical, you
can either use the helper function `metrical::from_env`, or 
use create a configuration directly via a `ConfigurationBuilder` and
invoke `metrical::from_config` with the configuration.

After that, you can easily create a counter, gauge or timer with
`metrical::counter`, `metrical::gauge` and `metrical::timer` 
respectively.

## Getting Help
Feel free to email me at austin.jay.ives+metrical@gmail.com.

## License
This project is licensed under the MIT open source license

## Environment Variables

To configure the library via environment variables the following environment variables are used:

- `METRICAL_NETWORK_PROTOCOL` - [UDP|TCP]
- `METRICAL_NETWORK_DESTINATION` - (ip\_address:port) or (ip\_address)
- `METRICAL_SEND_METHOD` - [StatsD|Graphite|Graphite\_pickle]
- `METRICAL_NAMESPACE` - A path to put all created metrics underneath. Of the form `foo.myBar.baz`.
- `METRICAL_FLUSH_INTERVAL` - An interval of time (in seconds) before flushing the metrics buffer. 
- `METRICAL_FLUSH_AMOUNT` - The maximum amount of records to store in the metrics buffer at any one time.

Additionally, a prefix can be used with the environment variables. For example, if the prefix `MY_PROJ` was used, the `METRICAL_NETWORK_PROTOCOL` environment variable
will be looked up under `MY_PROJ_METRICAL_NETWORK_PROTOCOL`. 

