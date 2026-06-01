use clap::{Parser, Subcommand};
use nine_futu_lib::{FutuClient, MarketRegistry};

#[derive(Parser)]
#[command(name = "nine-futu")]
#[command(about = "CLI tool for Futu OpenD API")]
#[command(version)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1", env = "FUTU_HOST")]
    host: String,

    #[arg(long, default_value = "11111", env = "FUTU_PORT")]
    port: u16,

    #[arg(long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Quote {
        #[command(subcommand)]
        command: QuoteCommands,
    },
}

#[derive(Subcommand)]
enum QuoteCommands {
    Snapshot {
        #[arg(short = 'c', required = true)]
        code: String,
    },
    Kline {
        #[arg(short = 'c', required = true)]
        code: String,

        #[arg(short = 'k', long = "kt", default_value = "1d")]
        ktype: String,

        #[arg(short = 's', long = "st", default_value = "")]
        start: String,

        #[arg(short = 'e', long = "ed", default_value = "")]
        end: String,

        #[arg(short = 'n', long = "num", default_value = "10")]
        num: u32,

        #[arg(long = "ndjson")]
        ndjson: bool,
    },
}

#[derive(serde::Serialize)]
struct KlineBar {
    code: String,
    ktype: String,
    date: String,
    time: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
    turnover: f64,
}

fn parse_ktype_time(ktype: &str, datetime_str: &str) -> (String, String) {
    let is_minute = matches!(ktype, "1m" | "3m" | "5m" | "15m" | "30m" | "60m");

    if datetime_str.is_empty() {
        return (String::new(), String::new());
    }

    if is_minute {
        // Try to parse as "YYYY-MM-DD HH:MM"
        if let Some((date, time)) = datetime_str.split_once(' ') {
            (date.to_string(), time.to_string())
        } else {
            // Only date provided for minute ktype
            (datetime_str.to_string(), String::new())
        }
    } else {
        // For daily and above, only use date part
        if let Some((date, _)) = datetime_str.split_once(' ') {
            (date.to_string(), String::new())
        } else {
            (datetime_str.to_string(), String::new())
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let registry = MarketRegistry::new();

    match &cli.command {
        Commands::Quote { command } => match command {
            QuoteCommands::Snapshot { code } => {
                let (full_code, _market_id) = registry.parse_code(code)?;

                if cli.debug {
                    eprintln!("[DEBUG] Connecting to {}:{}...", cli.host, cli.port);
                }
                let mut client = FutuClient::connect(&cli.host, cli.port, cli.debug).await?;
                
                if cli.debug {
                    eprintln!("[DEBUG] Initializing connection...");
                }
                client.init_connect().await?;
                
                if cli.debug {
                    eprintln!("[DEBUG] Subscribing to {}...", full_code);
                }
                let snapshots = client.get_market_snapshot(vec![full_code.clone()]).await?;
                
                if cli.debug {
                    eprintln!("[DEBUG] Got {} snapshot(s)", snapshots.len());
                }
                println!("{}", serde_json::to_string_pretty(&snapshots)?);
            }
            QuoteCommands::Kline { code, ktype, start, end, num, ndjson } => {
                let (full_code, _market_id) = registry.parse_code(code)?;

                if cli.debug {
                    eprintln!("[DEBUG] Connecting to {}:{}...", cli.host, cli.port);
                }
                let mut client = FutuClient::connect(&cli.host, cli.port, cli.debug).await?;
                
                if cli.debug {
                    eprintln!("[DEBUG] Initializing connection...");
                }
                client.init_connect().await?;

                let has_start = !start.is_empty();
                let has_end = !end.is_empty();
                let is_minute = matches!(ktype.as_str(), "1m" | "3m" | "5m" | "15m" | "30m" | "60m");

                if cli.debug {
                    eprintln!("[DEBUG] Kline: code={}, ktype={}, start={}, end={}", full_code, ktype, start, end);
                }

                let kline_data = if is_minute && !has_start && !has_end {
                    // Minute timeframe without start/end: get all data for today
                    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                    let tomorrow = (chrono::Local::now() + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();

                    if cli.debug {
                        eprintln!("[DEBUG] Minute mode (all day): start={}, end={}", today, tomorrow);
                    }

                    client.get_history_kline_all(
                        &full_code,
                        ktype,
                        &today,
                        &tomorrow,
                    ).await?
                } else if has_start || has_end {
                    // History mode
                    let (start_date, _start_time) = parse_ktype_time(ktype, start);
                    let (end_date, _end_time) = parse_ktype_time(ktype, end);

                    // If start not provided, use today
                    let final_start = if start_date.is_empty() {
                        chrono::Local::now().format("%Y-%m-%d").to_string()
                    } else {
                        start_date
                    };

                    // If end not provided, use today + 1 day (API is exclusive of end)
                    let final_end = if end_date.is_empty() {
                        // Use today + 1 day
                        let tomorrow = chrono::Local::now() + chrono::Duration::days(1);
                        tomorrow.format("%Y-%m-%d").to_string()
                    } else {
                        // Add 1 day to end date (API is exclusive of end)
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d") {
                            let next_day = date + chrono::Duration::days(1);
                            next_day.format("%Y-%m-%d").to_string()
                        } else {
                            end_date
                        }
                    };

                    if cli.debug {
                        eprintln!("[DEBUG] History mode: start={}, end={}", final_start, final_end);
                    }

                    client.get_history_kline_all(
                        &full_code,
                        ktype,
                        &final_start,
                        &final_end,
                    ).await?
                } else {
                    // Realtime mode (non-minute or with num specified)
                    if cli.debug {
                        eprintln!("[DEBUG] Realtime mode: num={}", num);
                    }
                    client.get_cur_kline(&full_code, ktype, *num).await?
                };

                if cli.debug {
                    eprintln!("[DEBUG] Got {} kline bars", kline_data.len());
                }

                // Convert to output format
                let output: Vec<KlineBar> = kline_data.iter().map(|bar| {
                    let (date, time) = if is_minute {
                        // For minute ktypes, parse time_key which is "YYYY-MM-DD HH:MM:SS"
                        if let Some((d, t)) = bar.time_key.split_once(' ') {
                            (d.to_string(), t[..5].to_string()) // Take only HH:MM
                        } else {
                            (bar.time_key.clone(), String::new())
                        }
                    } else {
                        // For daily and above, only date
                        (bar.time_key.clone(), String::new())
                    };

                    KlineBar {
                        code: full_code.clone(),
                        ktype: ktype.clone(),
                        date,
                        time,
                        open: bar.open,
                        high: bar.high,
                        low: bar.low,
                        close: bar.close,
                        volume: bar.volume,
                        turnover: bar.turnover,
                    }
                }).collect();

                if *ndjson {
                    for bar in &output {
                        println!("{}", serde_json::to_string(bar)?);
                    }
                } else {
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
            }
        },
    }

    Ok(())
}
