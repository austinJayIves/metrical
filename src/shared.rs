use crate::collector::Collector;

static mut COLLECTOR: Option<&'static mut dyn Collector> = None;

pub fn collector() -> &'static mut &'static mut dyn Collector {
    // Simply returns a reference to the collector. This is unsafe because the collector
    // can technically be mutated during the holding of the reference. In reality, the Collector
    // can only be set once, before any calls to this function, so this shouldn't ever happen.
    unsafe {
        COLLECTOR.as_mut().unwrap()
    }
}

pub fn set_boxed_collector(collector: Box<dyn Collector>) {
    let static_collector = Box::leak(collector);

    set_collector(static_collector)
}

pub fn set_collector(collector: &'static mut dyn Collector) {
    unsafe {
        COLLECTOR = Some(collector)
    }
}