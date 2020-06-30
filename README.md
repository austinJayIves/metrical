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
