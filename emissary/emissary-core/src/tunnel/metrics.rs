// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::runtime::MetricType;

use alloc::{vec, vec::Vec};

// general
pub const NUM_TUNNEL_MESSAGES: &str = "tunnel_message_count";
pub const NUM_INBOUND_TUNNELS: &str = "inbound_tunnel_count";
pub const NUM_OUTBOUND_TUNNELS: &str = "outbound_tunnel_count";
pub const NUM_FRAGMENTS: &str = "tunnel_num_fragments";

// tunnel building
pub const NUM_PENDING_INBOUND_TUNNELS: &str = "pending_inbound_tunnel_count";
pub const NUM_PENDING_OUTBOUND_TUNNELS: &str = "pending_outbound_tunnel_count";
pub const NUM_BUILD_FAILURES: &str = "tunnel_build_failure_count";
pub const NUM_BUILD_SUCCESSES: &str = "tunnel_build_success_count";

// tunnel tests
pub const NUM_TEST_FAILURES: &str = "tunnel_test_failure_count";
pub const NUM_TEST_SUCCESSES: &str = "tunnel_test_success_count";
pub const TUNNEL_TEST_DURATIONS: &str = "tunnel_test_durations_bucket";

// transit
pub const NUM_TRANSIT_TUNNELS: &str = "transit_tunnels_count";
pub const NUM_TRANSIT_TUNNELS_ACCEPTED: &str = "transit_tunnels_accepted_count";
pub const NUM_TRANSIT_TUNNELS_REJECTED: &str = "transit_tunnels_rejected_count";

/// Register tunnel metrics.
pub fn register_metrics(mut metrics: Vec<MetricType>) -> Vec<MetricType> {
    // counters
    metrics.push(MetricType::Counter {
        name: NUM_TUNNEL_MESSAGES,
        description: "number of i2np messaged received to tunnel subsystem",
    });
    metrics.push(MetricType::Counter {
        name: NUM_BUILD_FAILURES,
        description: "number of tunnel build failures",
    });
    metrics.push(MetricType::Counter {
        name: NUM_BUILD_SUCCESSES,
        description: "number of tunnel build successes",
    });
    metrics.push(MetricType::Counter {
        name: NUM_TEST_FAILURES,
        description: "number of failed tunnel tests",
    });
    metrics.push(MetricType::Counter {
        name: NUM_TEST_SUCCESSES,
        description: "number of succeeded tunnel tests",
    });
    metrics.push(MetricType::Counter {
        name: NUM_TRANSIT_TUNNELS_ACCEPTED,
        description: "number of transit tunnels that were accepted",
    });
    metrics.push(MetricType::Counter {
        name: NUM_TRANSIT_TUNNELS_REJECTED,
        description: "number of transit tunnels that were rejected",
    });

    // gauges
    metrics.push(MetricType::Gauge {
        name: NUM_PENDING_INBOUND_TUNNELS,
        description: "number of pending inbound tunnels",
    });
    metrics.push(MetricType::Gauge {
        name: NUM_PENDING_OUTBOUND_TUNNELS,
        description: "number of pending outbound tunnels",
    });
    metrics.push(MetricType::Gauge {
        name: NUM_INBOUND_TUNNELS,
        description: "number of inbound tunnels",
    });
    metrics.push(MetricType::Gauge {
        name: NUM_OUTBOUND_TUNNELS,
        description: "number of outbound tunnels",
    });
    metrics.push(MetricType::Gauge {
        name: NUM_TRANSIT_TUNNELS,
        description: "number of transit tunnels",
    });

    // histograms
    metrics.push(MetricType::Histogram {
        name: TUNNEL_TEST_DURATIONS,
        description: "tunnel test durations",
        buckets: vec![
            3f64, 5f64, 10f64, 20f64, 40f64, 80f64, 100f64, 150f64, 200f64, 500f64, 1000f64,
        ],
    });
    metrics.push(MetricType::Histogram {
        name: NUM_FRAGMENTS,
        description: "number of fragments per message",
        buckets: vec![
            1f64, 2f64, 5f64, 8f64, 10f64, 15f64, 20f64, 35f64, 30f64, 40f64, 50f64,
        ],
    });

    metrics
}
