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

pub const NUM_FLOODFILLS: &str = "floodfill_count";
pub const NUM_CONNECTED_FLOODFILLS: &str = "connected_floodfill_count";
pub const NUM_QUERIES: &str = "num_queries";
pub const NUM_SUCCEEDED_QUERIES: &str = "num_succeeded_queries";
pub const NUM_FAILED_QUERIES: &str = "num_failed_queries";
pub const NUM_ACTIVE_QUERIES: &str = "num_active_queries";
pub const QUERY_DURATION_BUCKET: &str = "query_duration_bucket";
pub const NUM_NETDB_MESSAGES: &str = "netdb_message_count";

/// Register NetDB metrics.
pub fn register_metrics(mut metrics: Vec<MetricType>) -> Vec<MetricType> {
    // counters
    metrics.push(MetricType::Counter {
        name: NUM_FLOODFILLS,
        description: "total number of know floodfills",
    });
    metrics.push(MetricType::Counter {
        name: NUM_QUERIES,
        description: "total number of queries made",
    });
    metrics.push(MetricType::Counter {
        name: NUM_SUCCEEDED_QUERIES,
        description: "total number of succeded queries",
    });
    metrics.push(MetricType::Counter {
        name: NUM_FAILED_QUERIES,
        description: "total number of failed queries",
    });
    metrics.push(MetricType::Counter {
        name: NUM_NETDB_MESSAGES,
        description: "number of i2np messaged received to netdb subsystem",
    });

    // gauges
    metrics.push(MetricType::Gauge {
        name: NUM_CONNECTED_FLOODFILLS,
        description: "number of connected floodfills",
    });
    metrics.push(MetricType::Gauge {
        name: NUM_ACTIVE_QUERIES,
        description: "number of active queries",
    });

    // histograms
    metrics.push(MetricType::Histogram {
        name: QUERY_DURATION_BUCKET,
        description: "how long queries take",
        buckets: vec![1f64, 3f64, 5f64, 8f64, 10f64, 15f64, 30f64, 60f64],
    });

    metrics
}
