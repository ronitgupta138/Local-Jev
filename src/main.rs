//! local-jev CLI: 100% Offline, Zero-Cost Drop-In Alternative to TypeSafe Jev.

use clap::{Parser, Subcommand};
use colored::Colorize;
use local_jev::domain::{ChoiceRequest, RouteRequest, ScoreRequest};
use local_jev::engine::JevDecisionEngine;
use local_jev::server::create_router;
use std::net::SocketAddr;
use std::time::Instant;

#[derive(Parser)]
#[command(
    name = "local-jev",
    author = "Ronit Gupta <ronitgupta138@gmail.com>",
    version = "1.0.0",
    about = "100% Offline, Zero-Cost Drop-In Alternative to TypeSafe Jev System One Decisions"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the TypeSafe-compatible local HTTP server
    Serve {
        /// Port to bind the server on (default: 8080)
        #[arg(long, default_value_t = 8080)]
        port: u16,

        /// Host address to bind (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },

    /// Run single-pass choice selection on candidate options
    Choice {
        /// Context state / diff
        #[arg(long)]
        state: String,

        /// Decision query
        #[arg(long)]
        question: String,

        /// Comma-separated list of candidate options
        #[arg(long, value_delimiter = ',')]
        options: Vec<String>,

        /// Temperature scaling (default: 0.1)
        #[arg(long, default_value_t = 0.1)]
        temperature: f32,

        /// Output as raw JSON
        #[arg(long)]
        json: bool,
    },

    /// Semantic relevance scoring (0.0 - 1.0)
    Score {
        /// Text to evaluate
        #[arg(long)]
        text: String,

        /// Criteria to score against
        #[arg(long)]
        criteria: String,

        /// Output as raw JSON
        #[arg(long)]
        json: bool,
    },

    /// Fast intent route classification
    Route {
        /// User prompt
        #[arg(long)]
        prompt: String,

        /// Comma-separated candidate routes
        #[arg(long, value_delimiter = ',')]
        routes: Vec<String>,

        /// Output as raw JSON
        #[arg(long)]
        json: bool,
    },

    /// Run microsecond latency benchmarks
    Benchmark {
        /// Number of decision iterations (default: 1000)
        #[arg(long, default_value_t = 1000)]
        iterations: usize,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let engine = JevDecisionEngine::new();

    match cli.command {
        Commands::Serve { port, host } => {
            let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
            let app = create_router(engine);

            println!();
            println!("{}", "┌────────────────────────────────────────────────────────────────────────┐".cyan());
            println!(
                "{}  {}  {}",
                "│".cyan(),
                "⚡ local-jev Decision Server (TypeSafe Jev Drop-In)".bold(),
                "              │".cyan()
            );
            println!("{}", "├────────────────────────────────────────────────────────────────────────┤".cyan());
            println!(
                "{}  {: <14} :: {: <50} {}",
                "│".cyan(),
                "Endpoint".dimmed(),
                format!("http://{}:{}/v1", host, port).green().bold(),
                "│".cyan()
            );
            println!(
                "{}  {: <14} :: {: <50} {}",
                "│".cyan(),
                "Latency SLA".dimmed(),
                "< 18ms Single-Forward-Pass CPU Inference".white(),
                "│".cyan()
            );
            println!(
                "{}  {: <14} :: {: <50} {}",
                "│".cyan(),
                "Cost / Privacy".dimmed(),
                "$0.00 / 100% Offline Zero-Telemetry Air-Gapped".white(),
                "│".cyan()
            );
            println!("{}", "└────────────────────────────────────────────────────────────────────────┘".cyan());
            println!();

            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?;
        }

        Commands::Choice {
            state,
            question,
            options,
            temperature,
            json,
        } => {
            let req = ChoiceRequest {
                state,
                question,
                options,
                temperature,
            };
            let res = engine.decide_choice(&req)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                println!(
                    "{} {} (confidence: {:.1}%, latency: {:.2}ms)",
                    "✔ Choice:".green().bold(),
                    res.choice.cyan().bold(),
                    res.confidence * 100.0,
                    res.latency_ms
                );
            }
        }

        Commands::Score {
            text,
            criteria,
            json,
        } => {
            let req = ScoreRequest { text, criteria };
            let res = engine.score_text(&req)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                println!(
                    "{} {:.4} (latency: {:.2}ms)",
                    "✔ Score:".green().bold(),
                    res.score,
                    res.latency_ms
                );
            }
        }

        Commands::Route {
            prompt,
            routes,
            json,
        } => {
            let req = RouteRequest { prompt, routes };
            let res = engine.route_prompt(&req)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                println!(
                    "{} {} (confidence: {:.1}%, latency: {:.2}ms)",
                    "✔ Route:".green().bold(),
                    res.route.cyan().bold(),
                    res.confidence * 100.0,
                    res.latency_ms
                );
            }
        }

        Commands::Benchmark { iterations } => {
            println!("⚡ Benchmarking local-jev single-pass logit engine over {} iterations...", iterations);
            let req = ChoiceRequest {
                state: "User wants to inspect file structure of repository".to_string(),
                question: "Which tool should be invoked next?".to_string(),
                options: vec!["ls".to_string(), "grep".to_string(), "cat".to_string(), "git status".to_string()],
                temperature: 0.1,
            };

            let mut latencies_us = Vec::with_capacity(iterations);
            for _ in 0..iterations {
                let start = Instant::now();
                let _ = engine.decide_choice(&req)?;
                latencies_us.push(start.elapsed().as_micros());
            }

            latencies_us.sort_unstable();
            let p50 = latencies_us[iterations * 50 / 100];
            let p95 = latencies_us[iterations * 95 / 100];
            let p99 = latencies_us[iterations * 99 / 100];

            println!();
            println!("{}", "Latency SLA:".bold());
            println!("  p50 :: {:.2}µs ({:.4}ms)", p50 as f64, p50 as f64 / 1000.0);
            println!("  p95 :: {:.2}µs ({:.4}ms)", p95 as f64, p95 as f64 / 1000.0);
            println!("  p99 :: {:.2}µs ({:.4}ms)", p99 as f64, p99 as f64 / 1000.0);
            println!("  Throughput :: {:.1} decisions/sec on CPU", 1_000_000.0 / (p50 as f64));
            println!();
        }
    }

    Ok(())
}
