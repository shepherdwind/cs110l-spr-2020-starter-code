use http::StatusCode;
use rand::{Rng, SeedableRng};
use std::{sync::Arc, time::Duration};
use tokio::{io, net::TcpStream, sync::RwLock, time};

use crate::{request, response};

pub struct ConnectionConfig {
    /// How frequently we check whether upstream servers are alive (Milestone 4)
    #[allow(dead_code)]
    active_health_check_interval: usize,
    /// Where we should send requests when doing active health checks (Milestone 4)
    #[allow(dead_code)]
    active_health_check_path: String,
    /// Maximum number of requests an individual IP can make in a minute (Milestone 5)
    #[allow(dead_code)]
    max_requests_per_minute: usize,
    /// Addresses of servers that we are proxying to
    upstream_addresses: Vec<String>,
    success_addresses: Vec<String>,
}

impl ConnectionConfig {
    pub fn new(
        active_health_check_interval: usize,
        active_health_check_path: String,
        max_requests_per_minute: usize,
        upstream: Vec<String>,
    ) -> ConnectionConfig {
        let success_list = upstream.clone();
        ConnectionConfig {
            active_health_check_interval,
            active_health_check_path,
            max_requests_per_minute,
            upstream_addresses: upstream,
            success_addresses: success_list,
        }
    }
}

pub async fn health_check(config: Arc<RwLock<ConnectionConfig>>) {
    let read = config.read().await;
    let seconds = read.active_health_check_interval as u64;
    let mut interval = time::interval(Duration::from_secs(seconds));
    log::debug!("health check status, interval: {}", seconds);
    drop(read);
    interval.tick().await;
    loop {
        interval.tick().await;
        check_all(&config).await;
    }
}

pub async fn check_all(config: &Arc<RwLock<ConnectionConfig>>) {
    let read = config.read().await;
    let mut success_list = Vec::new();
    log::info!("start check all ips");
    for ip in &read.upstream_addresses {
        let check = check_ip(ip, &read.active_health_check_path).await;
        if check.is_some() {
            log::info!("check ok ip {}", &ip);
            success_list.push(ip.to_string());
        }
    }
    drop(read);
    if success_list.len() == 0 {
        return;
    }

    let mut write = config.write().await;
    write.success_addresses = success_list;
}

async fn check_ip(ip: &String, path: &String) -> Option<()> {
    match TcpStream::connect(ip).await {
        Ok(mut stream) => {
            let request = http::Request::builder()
                .method(http::Method::GET)
                .uri(format!("http://{}{}", ip, path))
                .header("Host", ip)
                .body(Vec::new())
                .unwrap();
            let req = request::write_to_stream(&request, &mut stream).await;
            log::info!("check_ip: {}, request result error {}", ip, req.is_err());
            if req.is_err() {
                log::error!("error happen {:?}", req.err());
                return None;
            }

            let response = response::read_from_stream(&mut stream, request.method()).await;
            log::info!("check_ip: response result error {}", req.is_err());
            if response.is_err() {
                return None;
            }

            let status = response.unwrap().status();
            log::info!("check_ip: response status {:?}", status);
            if status == StatusCode::OK {
                return Some(());
            }

            None
        }

        Err(err) => {
            println!("error happen {:?}, remove from config", err);
            None
        }
    }
}

pub async fn connect_to_upstream(
    config: &Arc<RwLock<ConnectionConfig>>,
) -> Result<TcpStream, std::io::Error> {
    let mut rng = rand::rngs::StdRng::from_entropy();
    loop {
        let s = config.read().await;
        let len = s.success_addresses.len();
        if len == 0 {
            break Err(io::Error::new(io::ErrorKind::Other, "no upstream can use"));
        }

        println!("success addresses list: {:?}", s.success_addresses);
        let upstream_idx = rng.gen_range(0, len);
        let upstream_ip = &s.success_addresses[upstream_idx].to_owned();
        // we must drop here
        drop(s);

        match TcpStream::connect(upstream_ip).await {
            Ok(stream) => {
                break Ok(stream);
            }
            Err(err) => {
                println!("error happen {:?}, remove from config", err);
                let r = &mut config.write().await;
                r.success_addresses.remove(upstream_idx);
            }
        }
    }
}
