use metrical::{counter, gauge, timer, from_env, flush};
use std::time::Duration;

#[test]
pub fn it_should_allow_configuration_from_the_environment() -> Result<(), Box<dyn std::error::Error>> {

    from_env(None)?;

    let mut ctr = counter("HelloCounter".to_owned());
    ctr.increment(10);

    let timer = timer("HelloTimer".to_owned());
    std::thread::sleep(Duration::from_secs(1));
    drop(ctr);
    drop(timer);

    let mut gauge = gauge("HelloGauge".to_owned());
    gauge.increment(20);

    drop(gauge);
    flush();
    Ok(())
}
