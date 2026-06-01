# nine-futu-cli

富途 OpenD API 嘅命令列工具，等股票炒家可以即刻攞到市場數據同執行交易。

## 功能

- **即時行情**: 攞到實時報價、擺盤、逐筆成交
- **歷史 K 線**: 用灵活日期範圍攞歷史蠟燭圖數據
- **訂閱管理**: 訂閱/取消訂閱實時數據流
- **JSON & NDJSON 輸出**: 機器可讀嘅輸出，方便自動化同腳本
- **多市場支援**: 香港、美國同更多市場

## 快速開始

### 前提條件

1. **FutuOpenD** 必須喺你部機運行
   - 去 [FutuOpenD 官網](https://openapi.futunn.com/futu-api-doc/quick/opend-base.html) 下載
   - 預設地址: `127.0.0.1:11111`

2. **安裝 nine-futu-cli**
   ```bash
   cargo install nine-futu-cli
   ```

### 基本用法

```bash
# 攞股票快照
nine-futu-cli quote snapshot -c 700

# 攞日 K 線（最近 10 根）
nine-futu-cli quote kline -c 700 -k 1d

# 攞今日 5 分鐘 K 線
nine-futu-cli quote kline -c 700 -k 5m

# 攞指定日期嘅 K 線
nine-futu-cli quote kline -c 700 -k 5m -s "2026-05-28" -e "2026-05-28"

# JSON 輸出
nine-futu-cli quote snapshot -c 700 --json

# NDJSON 輸出（每行一個 JSON）
nine-futu-cli quote kline -c 700 -k 5m --ndjson
```

## 股票炒家用例

### 1. 開市前睇價

```bash
# 快速睇現價
nine-futu-cli quote snapshot -c 700

# 輸出:
# {"code":"HK.00700","name":"TENCENT","last_done":436.2,...}
```

### 2. 分析趨勢

```bash
# 攞最近 30 日日 K 線
nine-futu-cli quote kline -c 700 -k 1d --num 30

# 攞指定日期範圍嘅 K 線
nine-futu-cli quote kline -c 700 -k 1d -s "2026-04-01" -e "2026-04-30"
```

### 3. 日內交易分析

```bash
# 攞今日 5 分鐘 K 線
nine-futu-cli quote kline -c 700 -k 5m

# 攞指定時間範圍嘅 1 分鐘 K 線
nine-futu-cli quote kline -c 700 -k 1m -s "2026-05-28 09:30" -e "2026-05-28 16:00"
```

### 4. 監控擺盤深度

```bash
# 訂閱擺盤
nine-futu-cli subscribe add -c 700 -t ORDER_BOOK

# 查詢訂閱狀態
nine-futu-cli subscribe list
```

### 5. 自動數據收集

```bash
# 收集日 K 線做回測（NDJSON 格式）
nine-futu-cli quote kline -c 700 -k 1d -s "2026-01-01" -e "2026-12-31" --ndjson > data.jsonl
```

## 命令

### 行情命令

| 命令 | 說明 | 範例 |
|------|------|------|
| `snapshot` | 攞市場快照 | `quote snapshot -c 700` |
| `kline` | 攞 K 線數據 | `quote kline -c 700 -k 1d` |

### 訂閱命令

| 命令 | 說明 | 範例 |
|------|------|------|
| `subscribe list` | 列出所有訂閱 | `subscribe list` |
| `subscribe add` | 加訂閱 | `subscribe add -c 700` |
| `subscribe remove` | 移除訂閱 | `subscribe remove -c 700` |
| `subscribe clear` | 移除所有訂閱 | `subscribe clear` |

## 股票代碼格式

| 輸入 | 解析為 | 說明 |
|------|--------|------|
| `700` | `HK.00700` | 純數字 → 港股，補零至 5 位 |
| `00700` | `HK.00700` | 已經係 5 位 |
| `AAPL` | `US.AAPL` | 字母 → 美股 |
| `HK.00700` | `HK.00700` | 完整代碼帶前綴 |
| `US.AAPL` | `US.AAPL` | 完整代碼帶前綴 |

## K 線類型

| 參數 | 說明 |
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

### JSON（預設）
```bash
$ nine-futu-cli quote snapshot -c 700
[
  {
    "code": "HK.00700",
    "name": "TENCENT",
    "last_done": 436.2,
    ...
  }
]
```

### NDJSON（每行一個 JSON）
```bash
$ nine-futu-cli quote kline -c 700 -k 5m --ndjson
{"code":"HK.00700","ktype":"5m","date":"2026-05-28","time":"09:35","open":431.0,...}
{"code":"HK.00700","ktype":"5m","date":"2026-05-28","time":"09:40","open":429.2,...}
```

## Debug 模式

加 `--debug` 可以睇到連接同訂閱詳情：

```bash
$ nine-futu-cli --debug quote kline -c 700 -k 5m
[DEBUG] Connecting to 127.0.0.1:11111...
[DEBUG] Initializing connection...
[DEBUG] Connected! conn_id=7467121556106515641, server_ver=906
[DEBUG] Kline: code=HK.00700, ktype=5m, start=, end=
[DEBUG] Minute mode (all day): start=2026-06-01, end=2026-06-02
[DEBUG] Got 66 kline bars
[
  {"code":"HK.00700",...},
  ...
]
```

## 設定

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

## 從源碼安裝

```bash
git clone https://github.com/your-repo/nine-futu.git
cd nine-futu
cargo build --release
cp target/release/nine-futu-cli /usr/local/bin/
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
