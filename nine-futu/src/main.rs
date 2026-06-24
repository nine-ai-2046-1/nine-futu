use clap::{Parser, Subcommand};
use nine_futu_lib::{FutuClient, MarketRegistry, ProcessManager, ErrorHandler, DaemonManager, LiveStorage, PushDataHandler, DefaultPushHandler, CliCallbackHandler, SubType, OutputMode};

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
    /// Subscribe to real-time data (default: kline only, stdout output)
    Sub {
        /// Stock code (e.g., 700, AAPL)
        #[arg(short = 'c', required = true)]
        code: String,

        /// Kline timeframe (1m, 3m, 5m, 15m, 30m, 60m, 1d, 1w, 1M)
        #[arg(short = 't', long = "tf", default_value = "5m")]
        timeframe: String,

        /// Run in foreground (don't daemonize)
        #[arg(short = 'f', long = "fe")]
        foreground: bool,

        /// Subscribe to all data types (Quote, OrderBook, Ticker, RtData, Broker, Kline)
        #[arg(long)]
        all: bool,

        /// Subscribe to real-time quotes
        #[arg(long)]
        quote: bool,

        /// Subscribe to order book depth
        #[arg(long)]
        orderbook: bool,

        /// Subscribe to ticker/trades
        #[arg(long)]
        ticker: bool,

        /// Subscribe to real-time data
        #[arg(long)]
        rtdata: bool,

        /// Subscribe to broker queue
        #[arg(long)]
        broker: bool,

        /// Save to files at path (default: stdout only). Use --output "" for default path
        #[arg(long)]
        output: Option<Option<String>>,

        /// CLI callback session ID
        #[arg(long)]
        cli: Option<String>,
    },
    Trade {
        #[command(subcommand)]
        command: TradeCommands,
    },
    Process {
        #[command(subcommand)]
        command: ProcessCommands,
    },
    Clean {
        #[arg(short = 'd', required = true)]
        dest: String,

        #[arg(short = 'y')]
        yes: bool,
    },
}

#[derive(Subcommand)]
enum QuoteCommands {
    Snapshot {
        #[arg(short = 'c', required = true)]
        code: String,

        #[arg(long)]
        json: bool,
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

        #[arg(short = 'p', long = "period")]
        period: Option<u32>,

        #[arg(short = 'n', long = "num", default_value = "10")]
        num: u32,

        #[arg(long)]
        json: bool,

        #[arg(long, default_value = "0")]
        delay: u64,

        #[arg(long)]
        cli: Option<String>,

        #[arg(long)]
        extended: bool,
    },
}

#[derive(Subcommand)]
enum TradeCommands {
    Accounts,
    Funds,
    Buy {
        #[command(subcommand)]
        command: BuyCommands,
    },
    Sell {
        #[command(subcommand)]
        command: SellCommands,
    },
    Modify {
        #[arg(short = 'o', long = "oi", required = true)]
        order_id: u64,

        #[arg(short = 'p')]
        price: Option<f64>,

        #[arg(short = 'q')]
        qty: Option<i32>,

        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
    Cancel {
        #[arg(short = 'o', long = "oi", required = true)]
        order_id: u64,

        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
    Orders {
        #[arg(long)]
        history: bool,

        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
    Positions {
        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
    Trades {
        #[arg(long)]
        history: bool,

        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
    Cashflow {
        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
}

#[derive(Subcommand)]
enum BuyCommands {
    Limit {
        #[arg(short = 'c', required = true)]
        code: String,

        #[arg(short = 'q', required = true)]
        qty: i32,

        #[arg(short = 'p', required = true)]
        price: f64,

        #[arg(short = 's')]
        sl: Option<f64>,

        #[arg(short = 't')]
        tp: Option<f64>,

        #[arg(short = 'y')]
        yes: bool,

        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
    Market {
        #[arg(short = 'c', required = true)]
        code: String,

        #[arg(short = 'q', required = true)]
        qty: i32,

        #[arg(short = 's')]
        sl: Option<f64>,

        #[arg(short = 't')]
        tp: Option<f64>,

        #[arg(short = 'y')]
        yes: bool,

        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
}

#[derive(Subcommand)]
enum SellCommands {
    Limit {
        #[arg(short = 'c', required = true)]
        code: String,

        #[arg(short = 'q', required = true)]
        qty: i32,

        #[arg(short = 'p', required = true)]
        price: f64,

        #[arg(short = 'y')]
        yes: bool,

        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
    Market {
        #[arg(short = 'c', required = true)]
        code: String,

        #[arg(short = 'q', required = true)]
        qty: i32,

        #[arg(short = 'y')]
        yes: bool,

        #[arg(long)]
        sim: bool,

        #[arg(long)]
        real: bool,
    },
}

#[derive(Subcommand)]
enum ProcessCommands {
    List,
    Stop {
        pid: u32,
    },
    Status {
        code: String,
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
        if let Some((date, time)) = datetime_str.split_once(' ') {
            (date.to_string(), time.to_string())
        } else {
            (datetime_str.to_string(), String::new())
        }
    } else {
        if let Some((date, _)) = datetime_str.split_once(' ') {
            (date.to_string(), String::new())
        } else {
            (datetime_str.to_string(), String::new())
        }
    }
}

async fn run_subscription(
    cli: &Cli,
    full_code: &str,
    timeframe: &str,
    process_mgr: &ProcessManager,
    cli_session: Option<&str>,
    all_types: bool,
    quote: bool,
    orderbook: bool,
    ticker: bool,
    rtdata: bool,
    broker: bool,
    output_mode: OutputMode,
) -> anyhow::Result<()> {
    // Create PID file
    process_mgr.create_pid_file(full_code, timeframe)?;

    // Setup signal handlers
    let mut shutdown_rx = DaemonManager::setup_signal_handlers().await;

    // Connect to FutuOpenD
    if cli.debug {
        eprintln!("[DEBUG] Connecting to {}:{}...", cli.host, cli.port);
    }

    let mut client = FutuClient::connect(&cli.host, cli.port, cli.debug).await?;
    
    if cli.debug {
        eprintln!("[DEBUG] Initializing connection...");
    }
    client.init_connect().await?;

    // Determine subscription types based on flags
    let mut sub_types = Vec::new();

    if all_types {
        // Subscribe to all data types
        sub_types = vec![
            SubType::Quote,
            SubType::OrderBook,
            SubType::Ticker,
            SubType::RtData,
            SubType::Broker,
        ];
    } else {
        // Add individual subscription types
        if quote {
            sub_types.push(SubType::Quote);
        }
        if orderbook {
            sub_types.push(SubType::OrderBook);
        }
        if ticker {
            sub_types.push(SubType::Ticker);
        }
        if rtdata {
            sub_types.push(SubType::RtData);
        }
        if broker {
            sub_types.push(SubType::Broker);
        }
    }

    // Add K-line type based on timeframe (always included)
    let kline_sub = match timeframe {
        "1m" => SubType::K1M,
        "3m" => SubType::K3M,
        "5m" => SubType::K5M,
        "15m" => SubType::K15M,
        "30m" => SubType::K30M,
        "60m" => SubType::K60M,
        "1d" => SubType::KDay,
        "1w" => SubType::KWeek,
        "1M" => SubType::KMon,
        _ => SubType::K5M,
    };
    sub_types.push(kline_sub);

    // Subscribe using the client's subscribe method
    if cli.debug {
        eprintln!("[DEBUG] Subscribing to {} with types: {:?}", full_code, sub_types);
    }
    client.subscribe(full_code, sub_types).await?;

    if cli.debug {
        eprintln!("[DEBUG] Subscribed to {} with timeframe {}", full_code, timeframe);
    }

    // Get push receiver and start background reader
    if let Some(rx) = client.get_push_receiver() {
        // Start background task to read push data
        let stream = client.get_stream();
        let push_tx = client.get_push_sender().unwrap();
        FutuClient::start_push_reader(stream, push_tx, cli.debug);

        let mut handler = PushDataHandler::new(rx, timeframe)
            .with_output_mode(output_mode.clone());
        
        // DefaultPushHandler not needed - output routing is handled by PushDataHandler
        
        // Add CLI callback handler if --cli flag is provided
        if let Some(session_id) = cli_session {
            handler.add_handler(Box::new(CliCallbackHandler::new(session_id, full_code, timeframe)));
        }

        // Run until shutdown signal
        tokio::select! {
            _ = handler.run() => {}
            _ = shutdown_rx.recv() => {
                if cli.debug {
                    eprintln!("[DEBUG] Shutting down...");
                }
            }
        }
    }

    // Cleanup PID file
    process_mgr.remove_pid_file(full_code)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let registry = MarketRegistry::new();

    match &cli.command {
        Commands::Quote { command } => match command {
            QuoteCommands::Snapshot { code, json } => {
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

                if *json {
                    // JSON array output
                    println!("{}", serde_json::to_string_pretty(&snapshots)?);
                } else {
                    // NDJSON output (default)
                    for snapshot in &snapshots {
                        println!("{}", serde_json::to_string(snapshot)?);
                    }
                }
            }
            QuoteCommands::Kline { code, ktype, start, end, period, num, json, delay, cli: cli_session, extended } => {
                let (full_code, _market_id) = registry.parse_code(code)?;

                if cli.debug {
                    eprintln!("[DEBUG] Connecting to {}:{}...", cli.host, cli.port);
                }
                let mut client = FutuClient::connect(&cli.host, cli.port, cli.debug).await?;
                
                if cli.debug {
                    eprintln!("[DEBUG] Initializing connection...");
                }
                client.init_connect().await?;

                let is_minute = matches!(ktype.as_str(), "1m" | "3m" | "5m" | "15m" | "30m" | "60m");

                // Calculate start date based on period flag or explicit start
                let effective_start = if let Some(days) = period {
                    // Period flag: calculate start date as N days before today (or before end)
                    let end_date = if !end.is_empty() {
                        end.clone()
                    } else {
                        chrono::Local::now().format("%Y-%m-%d").to_string()
                    };
                    
                    if let Ok(end_naive) = chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d") {
                        let start_naive = end_naive - chrono::Duration::days(*days as i64);
                        start_naive.format("%Y-%m-%d").to_string()
                    } else {
                        start.clone()
                    }
                } else {
                    start.clone()
                };

                let has_start = !effective_start.is_empty();
                let has_end = !end.is_empty();

                if cli.debug {
                    eprintln!("[DEBUG] Kline: code={}, ktype={}, start={}, end={}, period={:?}, delay={}", 
                        full_code, ktype, effective_start, end, period, delay);
                }

                let kline_data = if is_minute && !has_start && !has_end {
                    // Auto-find last trading day (max 7 days back)
                    let mut found_data = Vec::new();
                    let mut days_back = 0;
                    let max_days_back = 7;

                    while days_back < max_days_back && found_data.is_empty() {
                        let target_date = chrono::Local::now() - chrono::Duration::days(days_back);
                        let date_str = target_date.format("%Y-%m-%d").to_string();
                        let next_day = (target_date + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();

                        if cli.debug {
                            eprintln!("[DEBUG] Trying date: {} (days_back={})", date_str, days_back);
                        }

                        found_data = client.get_history_kline_all(
                            &full_code,
                            ktype,
                            &date_str,
                            &next_day,
                            *extended,
                        ).await?;

                        if !found_data.is_empty() {
                            if cli.debug {
                                eprintln!("[DEBUG] Found {} bars for {}", found_data.len(), date_str);
                            }
                            break;
                        }

                        days_back += 1;
                    }

                    found_data
                } else if has_start || has_end {
                    let (start_date, _start_time) = parse_ktype_time(ktype, &effective_start);
                    let (end_date, _end_time) = parse_ktype_time(ktype, end);

                    let final_start = if start_date.is_empty() {
                        chrono::Local::now().format("%Y-%m-%d").to_string()
                    } else {
                        start_date
                    };

                    let final_end = if end_date.is_empty() {
                        let tomorrow = chrono::Local::now() + chrono::Duration::days(1);
                        tomorrow.format("%Y-%m-%d").to_string()
                    } else {
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d") {
                            let next_day = date + chrono::Duration::days(1);
                            next_day.format("%Y-%m-%d").to_string()
                        } else {
                            end_date
                        }
                    };

                    if cli.debug {
                        eprintln!("[DEBUG] History mode: start={}, end={}, extended={}", final_start, final_end, extended);
                    }

                    client.get_history_kline_all(
                        &full_code,
                        ktype,
                        &final_start,
                        &final_end,
                        *extended,
                    ).await?
                } else {
                    if cli.debug {
                        eprintln!("[DEBUG] Realtime mode: num={}", num);
                    }
                    client.get_cur_kline(&full_code, ktype, *num).await?
                };

                if cli.debug {
                    eprintln!("[DEBUG] Got {} kline bars", kline_data.len());
                }

                let output: Vec<KlineBar> = kline_data.iter().map(|bar| {
                    let (date, time) = if is_minute {
                        if let Some((d, t)) = bar.time_key.split_once(' ') {
                            (d.to_string(), t[..5].to_string())
                        } else {
                            (bar.time_key.clone(), String::new())
                        }
                    } else {
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

                if *json {
                    // JSON array output
                    println!("{}", serde_json::to_string_pretty(&output)?);
                } else {
                    // NDJSON output (default) with optional delay and CLI callback
                    for (i, bar) in output.iter().enumerate() {
                        println!("{}", serde_json::to_string(bar)?);
                        
                        // Call CLI callback if --cli flag is provided
                        if let Some(session_id) = cli_session {
                            let data_json = serde_json::to_string(bar)?;
                            if let Err(e) = nine_futu_lib::call_cli(session_id, &full_code, ktype, &data_json) {
                                eprintln!("[CLI callback error] {}", e);
                            }
                        }
                        
                        if *delay > 0 && i < output.len() - 1 {
                            tokio::time::sleep(tokio::time::Duration::from_secs(*delay)).await;
                        }
                    }
                }
            }
        },

        Commands::Sub { code, timeframe, foreground, all: all_types, quote, orderbook, ticker, rtdata, broker, output, cli: cli_session } => {
            let (full_code, _market_id) = registry.parse_code(code)?;
            let process_mgr = ProcessManager::new();

            // Check if already running
            let existing_pid = process_mgr.check_process(&full_code);
            if existing_pid != -1 {
                eprintln!("Error: Subscription already running for {} (PID: {})", full_code, existing_pid);
                std::process::exit(1);
            }

            // Build output mode from --output flag
            let output_mode = match output {
                Some(Some(path)) => {
                    if path.is_empty() {
                        OutputMode::FileDefault
                    } else {
                        OutputMode::File(std::path::PathBuf::from(path))
                    }
                }
                Some(None) => OutputMode::FileDefault,
                None => OutputMode::Stdout,
            };

            // Run in foreground if -f flag OR stdout mode (no --output)
            let run_foreground = *foreground || matches!(output_mode, OutputMode::Stdout);

            if run_foreground {
                // Run in foreground
                run_subscription(
                    &cli,
                    &full_code,
                    timeframe,
                    &process_mgr,
                    cli_session.as_deref(),
                    *all_types,
                    *quote,
                    *orderbook,
                    *ticker,
                    *rtdata,
                    *broker,
                    output_mode,
                ).await?;
            } else {
                // Daemon mode: spawn new process
                let exe = std::env::current_exe()?;
                let mut args = vec![
                    "sub".to_string(),
                    "-c".to_string(),
                    code.clone(),
                    "-t".to_string(),
                    timeframe.clone(),
                    "-f".to_string(),
                ];

                if *all_types {
                    args.push("--all".to_string());
                }
                if *quote {
                    args.push("--quote".to_string());
                }
                if *orderbook {
                    args.push("--orderbook".to_string());
                }
                if *ticker {
                    args.push("--ticker".to_string());
                }
                if *rtdata {
                    args.push("--rtdata".to_string());
                }
                if *broker {
                    args.push("--broker".to_string());
                }
                if let Some(out) = output {
                    match out {
                        Some(path) => {
                            args.push("--output".to_string());
                            if !path.is_empty() {
                                args.push(path.clone());
                            }
                        }
                        None => args.push("--output".to_string()),
                    }
                }

                if cli.debug {
                    eprintln!("[DEBUG] Spawning daemon process...");
                }

                let mut cmd = std::process::Command::new(&exe);
                cmd.args(&args);
                cmd.stdout(std::process::Stdio::null());
                cmd.stderr(std::process::Stdio::null());

                let child = cmd.spawn()?;
                eprintln!("Daemon started with PID: {}", child.id());
            }
        },

        Commands::Trade { command } => {
            let config = nine_futu_lib::Config::load()?;
            
            // Show config advice
            let account_status = if config.account.account_id.is_empty() {
                "not set"
            } else {
                &config.account.account_id
            };
            let real_status = if config.is_real_trade_enabled() {
                "enabled"
            } else {
                "disabled"
            };
            eprintln!("[CONFIG] Account: {} | Real trade: {}", account_status, real_status);
            if config.account.account_id.is_empty() || !config.is_real_trade_enabled() {
                eprintln!("[CONFIG] Update config.toml at {}", nine_futu_lib::Config::config_path().display());
            }
            
            match command {
                TradeCommands::Accounts => {
                    let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                    let accounts = trade_client.get_acc_list().await?;
                    println!("{}", serde_json::to_string_pretty(&accounts)?);
                }
                TradeCommands::Funds => {
                    let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                    let funds = trade_client.get_funds().await?;
                    println!("{}", serde_json::to_string_pretty(&funds)?);
                }
                TradeCommands::Buy { command } => match command {
                    BuyCommands::Limit { code, qty, price, sl, tp, yes, sim, real } => {
                        let mut config = config;
                        if *real {
                            config.account.real_trade_enabled = true;
                        }
                        let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                        
                        if *real {
                            trade_client.set_trade_env("REAL");
                        } else if *sim {
                            trade_client.set_trade_env("SIMULATE");
                        }
                        
                        let env = trade_client.get_trade_env();
                        eprintln!("[{}] Buy {} {} @ {}", env, code, qty, price);
                        
                        if !*yes {
                            eprint!("Confirm? [y/N] ");
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input)?;
                            if !input.trim().eq_ignore_ascii_case("y") {
                                eprintln!("Cancelled");
                                return Ok(());
                            }
                        }
                        
                        let result = trade_client.place_order(code, "BUY", *qty, *price, "NORMAL").await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                    BuyCommands::Market { code, qty, sl, tp, yes, sim, real } => {
                        let mut config = config;
                        if *real {
                            config.account.real_trade_enabled = true;
                        }
                        let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                        
                        if *real {
                            trade_client.set_trade_env("REAL");
                        } else if *sim {
                            trade_client.set_trade_env("SIMULATE");
                        }
                        
                        let env = trade_client.get_trade_env();
                        eprintln!("[{}] Buy {} {} @ MARKET", env, code, qty);
                        
                        if !*yes {
                            eprint!("Confirm? [y/N] ");
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input)?;
                            if !input.trim().eq_ignore_ascii_case("y") {
                                eprintln!("Cancelled");
                                return Ok(());
                            }
                        }
                        
                        let result = trade_client.place_order(code, "BUY", *qty, 0.0, "MARKET").await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                },
                TradeCommands::Sell { command } => match command {
                    SellCommands::Limit { code, qty, price, yes, sim, real } => {
                        let mut config = config;
                        if *real {
                            config.account.real_trade_enabled = true;
                        }
                        let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                        
                        if *real {
                            trade_client.set_trade_env("REAL");
                        } else if *sim {
                            trade_client.set_trade_env("SIMULATE");
                        }
                        
                        let env = trade_client.get_trade_env();
                        eprintln!("[{}] Sell {} {} @ {}", env, code, qty, price);
                        
                        if !*yes {
                            eprint!("Confirm? [y/N] ");
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input)?;
                            if !input.trim().eq_ignore_ascii_case("y") {
                                eprintln!("Cancelled");
                                return Ok(());
                            }
                        }
                        
                        let result = trade_client.place_order(code, "SELL", *qty, *price, "NORMAL").await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                    SellCommands::Market { code, qty, yes, sim, real } => {
                        let mut config = config;
                        if *real {
                            config.account.real_trade_enabled = true;
                        }
                        let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                        
                        if *real {
                            trade_client.set_trade_env("REAL");
                        } else if *sim {
                            trade_client.set_trade_env("SIMULATE");
                        }
                        
                        let env = trade_client.get_trade_env();
                        eprintln!("[{}] Sell {} {} @ MARKET", env, code, qty);
                        
                        if !*yes {
                            eprint!("Confirm? [y/N] ");
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input)?;
                            if !input.trim().eq_ignore_ascii_case("y") {
                                eprintln!("Cancelled");
                                return Ok(());
                            }
                        }
                        
                        let result = trade_client.place_order(code, "SELL", *qty, 0.0, "MARKET").await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                },
                TradeCommands::Modify { order_id, price, qty, sim, real } => {
                    let mut config = config;
                    if *real {
                        config.account.real_trade_enabled = true;
                    }
                    let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                    
                    if *real {
                        trade_client.set_trade_env("REAL");
                    } else if *sim {
                        trade_client.set_trade_env("SIMULATE");
                    }
                    
                    let result = trade_client.modify_order(*order_id, *price, *qty).await?;
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                TradeCommands::Cancel { order_id, sim, real } => {
                    let mut config = config;
                    if *real {
                        config.account.real_trade_enabled = true;
                    }
                    let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                    
                    if *real {
                        trade_client.set_trade_env("REAL");
                    } else if *sim {
                        trade_client.set_trade_env("SIMULATE");
                    }
                    
                    let result = trade_client.cancel_order(*order_id).await?;
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                TradeCommands::Orders { history, sim, real } => {
                    let mut config = config;
                    if *real {
                        config.account.real_trade_enabled = true;
                    }
                    let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                    
                    if *real {
                        trade_client.set_trade_env("REAL");
                    } else if *sim {
                        trade_client.set_trade_env("SIMULATE");
                    }
                    
                    let orders = trade_client.get_orders(*history).await?;
                    println!("{}", serde_json::to_string_pretty(&orders)?);
                }
                TradeCommands::Positions { sim, real } => {
                    let mut config = config;
                    if *real {
                        config.account.real_trade_enabled = true;
                    }
                    let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                    
                    if *real {
                        trade_client.set_trade_env("REAL");
                    } else if *sim {
                        trade_client.set_trade_env("SIMULATE");
                    }
                    
                    let positions = trade_client.get_positions().await?;
                    println!("{}", serde_json::to_string_pretty(&positions)?);
                }
                TradeCommands::Trades { history, sim, real } => {
                    let mut config = config;
                    if *real {
                        config.account.real_trade_enabled = true;
                    }
                    let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                    
                    if *real {
                        trade_client.set_trade_env("REAL");
                    } else if *sim {
                        trade_client.set_trade_env("SIMULATE");
                    }
                    
                    let trades = trade_client.get_trades(*history).await?;
                    println!("{}", serde_json::to_string_pretty(&trades)?);
                }
                TradeCommands::Cashflow { sim, real } => {
                    let mut config = config;
                    if *real {
                        config.account.real_trade_enabled = true;
                    }
                    let mut trade_client = nine_futu_lib::TradeClient::new(&cli.host, cli.port, config).await?;
                    
                    if *real {
                        trade_client.set_trade_env("REAL");
                    } else if *sim {
                        trade_client.set_trade_env("SIMULATE");
                    }
                    
                    let cashflow = trade_client.get_cashflow().await?;
                    println!("{}", serde_json::to_string_pretty(&cashflow)?);
                }
            }
        },

        Commands::Process { command } => {
            let process_mgr = ProcessManager::new();

            match command {
                ProcessCommands::List => {
                    let daemons = process_mgr.list_daemons()?;
                    if daemons.is_empty() {
                        println!("No running processes");
                    } else {
                        println!("{:<10} {:<15} {:<10} {}", "PID", "CODE", "TIMEFRAME", "START TIME");
                        println!("{}", "-".repeat(60));
                        for d in &daemons {
                            println!("{:<10} {:<15} {:<10} {}", d.pid, d.code, d.timeframe, d.start_time);
                        }
                    }
                }
                ProcessCommands::Stop { pid } => {
                    process_mgr.stop_daemon(*pid)?;
                    println!("Stopped process {}", pid);
                }
                ProcessCommands::Status { code } => {
                    let (full_code, _market_id) = registry.parse_code(code)?;
                    let pid = process_mgr.check_process(&full_code);
                    println!("{}", pid);
                }
            }
        },

        Commands::Clean { dest, yes } => {
            let storage = LiveStorage::new();
            let dest_path = std::path::PathBuf::from(dest);

            // Find old folders
            let mut old_folders = Vec::new();

            if storage.base_dir().exists() {
                for code_entry in std::fs::read_dir(storage.base_dir())? {
                    let code_entry = code_entry?;
                    if !code_entry.file_type()?.is_dir() {
                        continue;
                    }
                    let code = code_entry.file_name().to_string_lossy().to_string();

                    for date_entry in std::fs::read_dir(code_entry.path())? {
                        let date_entry = date_entry?;
                        if !date_entry.file_type()?.is_dir() {
                            continue;
                        }
                        let date = date_entry.file_name().to_string_lossy().to_string();

                        if storage.isOLDER(&date, 1)? {
                            old_folders.push((code.clone(), date, date_entry.path()));
                        }
                    }
                }
            }

            if old_folders.is_empty() {
                println!("No old folders to move");
                return Ok(());
            }

            // Show folders to move
            println!("Found {} folders older than 1 day:", old_folders.len());
            for (code, date, _) in &old_folders {
                println!("  - {}/{}", code, date);
            }

            // Ask confirmation unless -y
            if !yes {
                print!("\nMove to {}? [y/N] ", dest);
                use std::io::{self, Write};
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled");
                    return Ok(());
                }
            }

            // Move folders
            println!("\nMoving...");
            for (code, date, src_path) in &old_folders {
                let dest_code_dir = dest_path.join(code);
                let dest_date_dir = dest_code_dir.join(date);

                std::fs::create_dir_all(&dest_code_dir)?;
                std::fs::rename(src_path, &dest_date_dir)?;
                println!("  {} ← created", dest_date_dir.display());
            }

            println!("\nDone. Moved {} folders.", old_folders.len());
        }
    }

    Ok(())
}
