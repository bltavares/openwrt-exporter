// We cannot use rust-prometheus until
// we fix the use of atomics https://github.com/tikv/rust-prometheus/issues/315

use ubus_serde::{Context, UbusExtension};

fn main() {
    let context = Context::connect();
    let wifis = context.list(Some("hostapd.*")).iter().collect::<Vec<_>>();
    let server = tiny_http::Server::http(("0.0.0.0", 1337)).unwrap();

    loop {
        let request = match server.recv() {
            Ok(req) => req,
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        };

        let mut counter_2g = 0;
        let mut counter_5g = 0;
        for wifi in &wifis {
            let data = context
                .call_as::<APClients>(&wifi.path, "get_clients")
                .recv();
            let data = match data {
                Ok(data) => data,
                Err(e) => {
                    println!("error on call: {}", e);
                    continue;
                }
            };

            if data.freq < 5000 {
                counter_2g += data.clients.len()
            } else {
                counter_5g += data.clients.len()
            }
        }

        let payload = format!(
            r#"
# HELP wifi_clients_total The total number connected clients.
# TYPE wifi_clients_total gauge
wifi_clients_total{{freq="2.4g"}} {}
wifi_clients_total{{freq="5.0g"}} {}
"#,
            counter_2g, counter_5g
        );

        let result = request.respond(tiny_http::Response::from_string(payload));
        if result.is_err() {
            println!("Failed to send response");
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct APClients {
    freq: u32,
    /// mac-address indexed
    clients: std::collections::HashMap<String, Client>,
}
#[derive(Debug, serde::Deserialize)]
struct Client {
    authorized: bool,
    auth: bool,
    assoc: bool,
    preauth: bool,
    wds: bool,
    wmm: bool,
    ht: bool,
    vht: bool,
    wps: bool,
    mfp: bool,
    rrm: Vec<u8>,
    aid: u8,
}
