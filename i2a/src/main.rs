use clap::Parser;
use colored::*;
use futures_util::TryStreamExt;
use reqwest::Client;
use std::io::{self, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use warp::Filter;


// --- CLI ARGUMENTS STRUCT ---
#[derive(Parser, Debug)]
#[command(name = "i2a")]
#[command(author = "BlackTechX011")]
#[command(version = "1.0")]
#[command(about = "I2P to API Bridge", long_about = None)]
struct Args {
    /// The Target I2P URL (e.g., http://i2p-projekt.i2p)
    #[arg(short, long, default_value = "http://i2p-projekt.i2p")]
    target: String,

    /// Local port to host the API/Proxy on
    #[arg(short, long, default_value_t = 8790)]
    port: u16,

    /// The upstream I2P HTTP Proxy port (Emissary default: 4444)
    #[arg(long, default_value_t = 4444)]
    upstream: u16,

    /// Path to the I2P binary (Emissary)
    #[arg(long, default_value = "emissary-cli.exe")]
    bin: String,
}

#[tokio::main]
async fn main() {
    // 1. Parse Arguments
    let args = Args::parse();

    // 2. Print Banner
    print_banner();

    println!(
        "{} {} -> {}",
        "[CONFIG]".bold().blue(),
        "Target".yellow(),
        args.target.cyan()
    );
    println!(
        "{} {} -> 127.0.0.1:{}",
        "[CONFIG]".bold().blue(),
        "Local API".yellow(),
        args.port.to_string().cyan()
    );

    // 3. Launch/Check Emissary
    check_or_launch_router(&args.bin);

    // 4. Wait for Upstream Proxy
    if !wait_for_upstream(args.upstream) {
        println!(
            "\n{}",
            "[FATAL] Could not connect to I2P Router.".red().bold()
        );
        return;
    }

    // 5. Build Client
    let proxy_url = format!("http://127.0.0.1:{}", args.upstream);
    let client = Client::builder()
        .proxy(reqwest::Proxy::http(&proxy_url).expect("Invalid proxy URL"))
        .build()
        .expect("Failed to build HTTP client");

    // 6. Setup Warp Server
    let proxy_route = warp::path::full()
        .and(warp::method())
        .and(warp::header::headers_cloned())
        .and(warp::body::stream())
        .and_then(move |path: warp::path::FullPath, method, headers, body| {
            let client = client.clone();
            let target = args.target.clone();
            async move { handle_request(client, target, path, method, headers, body).await }
        });

    println!(
        "\n{} Bridge is active. Access your API at:",
        "[SUCCESS]".bold().green()
    );
    println!("      {}", format!("http://127.0.0.1:{}", args.port).underline().white());
    println!("{}", "Press CTRL+C to stop.".dimmed());

    warp::serve(proxy_route)
        .run(([127, 0, 0, 1], args.port))
        .await;
}

fn print_banner() {
    let art = r#"
   _  _____           
  (_)/ __  \   __ _   
  | |`' / /'  / _` |  
  | |  / /   | (_| |  
  |_|./ /___  \__,_|  
     \_____/          
    "#;
    println!("{}", art.magenta().bold());
    println!("  I2P to API Bridge | v1.0");
    println!("  --------------------------\n");
}

fn check_or_launch_router(bin_name: &str) {
    print!("{} Checking for I2P Router process...", "[INIT]".bold().blue());
    io::stdout().flush().unwrap();

    // Attempt to spawn. If it fails, we assume it's missing or running.
    match Command::new(bin_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(_) => println!(" {}", "Started internal router.".green()),
        Err(_) => println!(
            " {}",
            "Not found (Assuming external router is running).".yellow()
        ),
    }
}

fn wait_for_upstream(port: u16) -> bool {
    print!("{} Connecting to upstream (Port {})...", "[NET]".bold().blue(), port);
    io::stdout().flush().unwrap();

    for _ in 0..15 {
        if TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
            println!(" {}", "Connected.".green());
            return true;
        }
        thread::sleep(Duration::from_secs(1));
        print!(".");
        io::stdout().flush().unwrap();
    }
    println!(" {}", "Timeout.".red());
    false
}

async fn handle_request(
    client: Client,
    base_url: String,
    path: warp::path::FullPath,
    method: warp::http::Method,
    _headers: warp::http::HeaderMap,
    body: impl futures_util::Stream<Item = Result<impl bytes::Buf, warp::Error>> + Send + Sync + 'static,
) -> Result<impl warp::Reply, warp::Rejection> {
    
    // Construct target URL
    let url = format!("{}{}", base_url, path.as_str());

    // Transform stream (Buf -> Bytes)
    let reqwest_body_stream = body.map_ok(|mut buf| {
        buf.copy_to_bytes(buf.remaining())
    }).map_err(|e| {
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    });

    let req_body = reqwest::Body::wrap_stream(reqwest_body_stream);

    let resp = client.request(method, &url).body(req_body).send().await;

    match resp {
        Ok(response) => {
            let status = response.status();
            let body = response.bytes_stream();
            Ok(warp::reply::with_status(
                warp::reply::html(warp::hyper::Body::wrap_stream(body)),
                status,
            ))
        }
        Err(_) => Ok(warp::reply::with_status(
            warp::reply::html("<h1>i2a Error</h1><p>Upstream I2P connection failed.</p>".into()),
            warp::http::StatusCode::BAD_GATEWAY,
        )),
    }
}