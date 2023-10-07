use std::net::SocketAddr;
use std::time::Duration;

use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

pub fn config() {
    let builder = PrometheusBuilder::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], 2000));
    tracing::info!("prometheus listening on {}/metrics", addr);
    builder
        .with_http_listener(addr)
        .idle_timeout(
            MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
            Some(Duration::from_secs(10)),
        )
        .install()
        .expect("failed to install Prometheus recorder");
    metrics::increment_counter!("idle_metric");
}