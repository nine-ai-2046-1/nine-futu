# nine-futu

富途 OpenD API 嘅命令列工具，等股票炒家可以即刻攞到市場數據同執行交易。

## 功能

- **即時行情**: 攞到實時報價、擺盤、逐筆成交
- **歷史 K 線**: 用灵活日期範圍攞歷史蠟燭圖數據
- **訂閱管理**: 訂閱/取消訂閱實時數據流
- **JSON & NDJSON 輸出**: 機器可讀嘅輸出，方便自動化同腳本
- **多市場支援**: 香港、美國同更多市場
- **期間計算**: 用 `-p` 標誌自動設定開始日期（例如 `-p 30` 代表最近 30 日）
- **延遲輸出**: 用 `--delay` 標誌喺 NDJSON 輸出之間加延遲
- **CLI 整合**: 用 `--cli` 標誌為每個 K 線數據調用外部工具

## 快速開始

### 前提條件

1. **FutuOpenD** 必須喺你部機運行
   - 去 [FutuOpenD 官網](https://openapi.futunn.com/futu-api-doc/quick/opend-base.html) 下載
   - 預設地址: `127.0.0.1:11111`

2. **安裝 nine-futu**
   ```bash
   cargo install nine-futu
   ```

### 基本用法

```bash
# 攞股票快照
nine-futu quote snapshot -c 700

# 攞日 K 線（最近 10 根）
nine-futu quote kline -c 700 -k 1d

# 攞今日 5 分鐘 K 線
nine-futu quote kline -c 700 -k 5m

# 攞指定日期嘅 K 線
nine-futu quote kline -c 700 -k 5m -s "2026-05-28" -e "2026-05-28"

# JSON 輸出
nine-futu quote snapshot -c 700 --json

# NDJSON 輸出（每行一個 JSON）
nine-futu quote kline -c 700 -k 5m
```

## 用例

### 1. 快速睇價

```bash
nine-futu quote snapshot -c 700
# 輸出: {"code":"HK.00700","name":"TENCENT","last_done":436.2,...}
```

### 2. 分析每日走勢

```bash
# 最近 30 日日 K 線
nine-futu quote kline -c 700 -k 1d -p 30

# 指定日期範圍
nine-futu quote kline -c 700 -k 1d -s "2026-04-01" -e "2026-04-30"
```

### 3. 日內分析

```bash
# 今日 5 分鐘 K 線
nine-futu quote kline -c 700 -k 5m

# 指定時間範圍
nine-futu quote kline -c 700 -k 1m -s "2026-05-28 09:30" -e "2026-05-28 16:00"
```

### 4. 匯出數據做回測

```bash
# 匯出日 K 線到檔案
nine-futu quote kline -c 700 -k 1d -p 365 --ndjson > data.jsonl
```

### 5. 延遲輸出（模擬即時）

```bash
# 每 5 分鐘 K 線之間加 60 秒延遲
nine-futu quote kline -c 700 -k 5m -p 7 --delay 60
```

### 6. CLI 整合（調用外部工具）

```bash
# 每個 K 線數據調用 nine-stock
nine-futu quote kline -c 700 -k 5m -p 30 --cli "session-123"

# 訂閱模式加 CLI 回調
nine-futu sub -c 700 -t 5m --cli "my-session"
```

### 7. 背景訂閱

```bash
# 啟動訂閱守護進程
nine-futu sub -c 700 -t 5m

# 查看運行中嘅進程
nine-futu process list

# 停止守護進程
nine-futu process stop <PID>
```

## 命令

### 行情命令

| 命令 | 說明 | 範例 |
|------|------|------|
| `snapshot` | 攞市場快照 | `quote snapshot -c 700` |
| `kline` | 攞 K 線數據 | `quote kline -c 700 -k 1d` |

### K 線標誌

| 標誌 | 說明 | 範例 |
|------|------|------|
| `-p <days>` | 期間：自動設定開始日期為 N 日前 | `-p 30` |
| `--delay <sec>` | NDJSON 輸出之間加延遲 | `--delay 60` |
| `--cli <session>` | 每個 K 線數據調用外部 CLI | `--cli "session-123"` |
| `--json` | 輸出為 JSON 陣列（預設: NDJSON） | `--json` |

### 訂閱命令

| 命令 | 說明 | 範例 |
|------|------|------|
| `sub` | 啟動訂閱（預設守護進程） | `sub -c 700 -t 5m` |
| `sub -f` | 啟動訂閱（前台） | `sub -c 700 -t 5m -f` |
| `process list` | 列出運行中嘅守護進程 | `process list` |
| `process status <code>` | 查詢訂閱狀態 | `process status 700` |
| `process stop <pid>` | 停止守護進程 | `process stop 12345` |

### 訂閱標誌

| 標誌 | 說明 | 範例 |
|------|------|------|
| `-t <timeframe>` | K 線時間框架（預設: 5m） | `-t 15m` |
| `-f` | 前台運行（預設: 守護進程） | `-f` |
| `--cli <session>` | 每個 K 線數據調用外部 CLI | `--cli "session-123"` |

## 交易命令

### 買入/賣出

```bash
# 買入限價單 (止損 + 止盈)
nine-futu trade buy limit -c 700 -q 100 -p 430 -sl 400 -tp 460

# 買入市價單
nine-futu trade buy market -c 700 -q 100

# 賣出限價單
nine-futu trade sell limit -c 700 -q 100 -p 450

# 自動確認（跳過確認提示）
nine-futu trade buy limit -c 700 -q 100 -p 430 -y

# 使用真實交易（需要 config 開啟）
nine-futu trade buy limit -c 700 -q 100 -p 430 --real

# 使用保證金帳戶
nine-futu trade buy limit -c 700 -q 100 -p 430 --margin
```

### 修改/取消訂單

```bash
# 修改訂單價格
nine-futu trade modify -oi 12345 -p 435

# 取消訂單
nine-futu trade cancel -oi 12345
```

### 帳戶及持倉

```bash
# 列出交易帳戶
nine-futu trade accounts

# 查詢帳戶資金
nine-futu trade funds

# 列出持倉
nine-futu trade positions

# 列出訂單
nine-futu trade orders

# 列出成交
nine-futu trade trades
```

### 交易環境

| 輸入 | 說明 |
|------|------|
| `--sim` | 模擬交易（預設） |
| `--real` | 真實交易（需要 config 開啟） |
| `--margin` | 保證金帳戶 |
| `-y` | 自動確認訂單 |

### 訂單狀態

| 狀態 | 說明 |
|------|------|
| Submitted | 已提交 |
| Filled | 已全部成交 |
| Partially Filled | 部分成交 |
| Cancelled | 已取消 |
| Failed | 下單失敗 |

## CLI 整合

用 `--cli` 時，nine-futu 會為每個 K 線數據調用外部 CLI 工具：

```bash
nine-futu quote kline -c 700 -k 5m -p 30 --cli "session-123"
```

呢個會為每個數據生成子進程：
```bash
nine-stock --session "session-123" --code "HK.00700" --ktype "5m" --data '{"code":"HK.00700",...}'
```

### --cli 嘅依賴

用 `--cli` 功能需要安裝額外工具：

| 工具 | 倉庫 | 說明 |
|------|------|------|
| [nine-stock](https://github.com/nine-ai-2026-1/nine-stock) | github.com/nine-ai-2026-1/nine-stock | 分析 K 線數據同發送報告 |
| [nine-poe](https://github.com/nine-ai-2026-1/nine-poe) | github.com/nine-ai-2026-1/nine-poe | AI 驅動嘅分析引擎（nine-stock 必需） |
| opencb | （消息工具） | 發送報告畀用戶（nine-stock 必需） |

**注意**：呢啲工具只係用 `--cli` 標誌時先需要。核心 nine-futu 功能唔使佢哋。

## 股票代碼格式

| 輸入 | 解析為 | 說明 |
|------|--------|------|
| `700` | `HK.00700` | 純數字 → 港股，補零至 5 位 |
| `00700` | `HK.00700` | 已經係 5 位 |
| `AAPL` | `US.AAPL` | 字母 → 美股 |
| `HK.00700` | `HK.00700` | 完整代碼帶前綴 |
| `US.AAPL` | `US.AAPL` | 完整代碼帶前綴 |

## K 線類型

| 標誌 | 說明 |
|------|------|
| `-k 1m` | 1 分鐘 K 線 |
| `-k 5m` | 5 分鐘 K 線 |
| `-k 15m` | 15 分鐘 K 線 |
| `-k 30m` | 30 分鐘 K 線 |
| `-k 60m` | 60 分鐘 K 線 |
| `-k 1d` | 日 K 線 |
| `-k 1w` | 週 K 線 |
| `-k 1M` | 月 K 線 |

## 訂閱類型

| 類型 | 說明 |
|------|------|
| `QUOTE` | 實時報價（預設） |
| `ORDER_BOOK` | 擺盤深度（預設） |
| `TICKER` | 逐筆成交 |
| `RT_DATA` | 實時數據 |
| `K_1M` | 1 分鐘 K 線 |
| `K_5M` | 5 分鐘 K 線 |
| `K_15M` | 15 分鐘 K 線 |
| `K_30M` | 30 分鐘 K 線 |
| `K_60M` | 60 分鐘 K 線 |
| `K_DAY` | 日 K 線 |
| `K_WEEK` | 週 K 線 |
| `K_MON` | 月 K 線 |
| `BROKER` | 經紀隊列 |

## 輸出格式

### NDJSON（預設）
```bash
$ nine-futu quote kline -c 700 -k 5m
{"code":"HK.00700","ktype":"5m","date":"2026-06-07","time":"09:35","open":431.0,...}
{"code":"HK.00700","ktype":"5m","date":"2026-06-07","time":"09:40","open":429.2,...}
```

### JSON 陣列
```bash
$ nine-futu quote snapshot -c 700 --json
[
  {
    "code": "HK.00700",
    "name": "TENCENT",
    "last_done": 436.2,
    ...
  }
]
```

## Debug 模式

加 `--debug` 可以睇到連接同訂閱詳情：

```bash
$ nine-futu --debug quote kline -c 700 -k 5m
[DEBUG] Connecting to 127.0.0.1:11111...
[DEBUG] Initializing connection...
[DEBUG] Connected! conn_id=7467121556106515641, server_ver=906
```

## 設定

### 設定檔案

位置: `~/.opens/nine-futu/config.toml`

```toml
[account]
account_id = ""                    # 你嘅富途帳號 ID
password = ""                      # 密碼（可選）
real_trade_enabled = false         # 開啟真實交易
default_trade_env = "SIMULATE"     # 預設: SIMULATE 或 REAL
default_account_type = "CASH"      # 預設: CASH 或 MARGIN

[connection]
host = "127.0.0.1"                # FutuOpenD 主機
port = 11111                       # FutuOpenD 端口
```

### 環境變數

| 變數 | 預設值 | 說明 |
|------|--------|------|
| `FUTU_HOST` | `127.0.0.1` | FutuOpenD 主機 |
| `FUTU_PORT` | `11111` | FutuOpenD 端口 |

### 命令列選項

| 選項 | 說明 |
|------|------|
| `--host <HOST>` | FutuOpenD 主機 |
| `--port <PORT>` | FutuOpenD 端口 |
| `--debug` | 開啟 debug 輸出 |

## 安裝

### 從源碼安裝

```bash
git clone https://github.com/nine-ai-2026-1/nine-futu.git
cd nine-futu
cargo build --release
cp target/release/nine-futu /usr/local/bin/
```

### 用建置腳本

```bash
./scripts/build-deploy
```

## 測試

```bash
# 運行所有測試
cargo test

# 運行需要 FutuOpenD 嘅測試
cargo test -- --ignored
```

## 授權

Apache License 2.0
