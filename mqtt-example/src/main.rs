use std::{
    env,
    process,
    time::Duration
};

extern crate paho_mqtt as mqtt;

const DFLT_BROKER: &str = "tcp://broker.emqx.io:1883";
const DFLT_CLIENT: &str = "rust_publish";
const DFLT_SERVER: &[&str] = &["rust/mqtt", "rust/test"];

// Define the QOS
const QOS:i32 = 1;

fn main() {
    let host = env::args().nth(1).unwrap_or_else(||
        DFLT_BROKER.to_string()
    );

    dbg!(host);
}
