use std::{
    env,
    process,
    thread,
    time::Duration
};

extern crate paho_mqtt as mqtt;

const DFLT_BROKER: &str = "tcp://broker.emqx.io:1883";
const DFLT_CLIENT: &str = "rust_subscribe";
const DFLT_TOPICS: &[&str] = &["rust/mqtt", "rust/test"];

// The QOS list that matches topics above

const DFLT_QOS: &[i32] = &[0, 1];

// Reconnect ot the broker when connection is lost.

fn try_reconnect(cli: &mqtt::Client) -> bool {
    println!("Connection lost. Waiting to retry connection");
    for _ in 0..12 {
        thread::sleep(Duration::from_millis(5000));
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            return true;
    }
}
    println!("Unable to reconnect after several attempts");
    false
}

// Subscribes to multiple topics.

fn subscribe_topics(cli: &mqtt::Client) {
    
}

fn main() {
    
}