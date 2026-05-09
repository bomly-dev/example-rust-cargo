use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    value: i32,
}

fn main() {
    // Reachable: chrono::Utc::now() internally uses time 0.1.44
    // time 0.1.44 is affected by RUSTSEC-2020-0071 (unsafe local_offset)
    // and RUSTSEC-2020-0159 (potential segfault in multithreaded programs)
    let now = Utc::now();
    println!("Current time: {}", now);

    let config = Config {
        name: "example".to_string(),
        value: 42,
    };
    let json_str = serde_json::to_string(&config).unwrap();
    println!("Config: {}", json_str);
}
