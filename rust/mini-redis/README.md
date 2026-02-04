# Mini-Redis (Rust 進階實戰)

這是一個基於 Tokio 實現的高性能、異步 Redis 伺服器，旨在深入探討 Rust 的 **生命週期 (Lifetimes)**、**零拷貝解析 (Zero-copy Parsing)** 和 **高併發狀態管理**。

## 核心特性
- **RESP 協議解析器**：使用 `Frame<'a>` 實現零拷貝解析，極大地減少了內存分配。
- **異步 IO 層**：基於 `tokio` 的緩衝讀寫，支持大併發連接。
- **高併發存儲**：使用 `DashMap` 提供細粒度的鎖控制。
- **TTL 支持**：基於 `crossbeam-skiplist` 和後台任務的自動過期清理。
- **Pub/Sub**：支持多對多實時消息推送。

## 快速開始
```bash
# 啟動伺服器
cargo run --bin server

# 運行集成測試
cargo test --test integration
```

## 已支持命令
- `PING [message]`
- `GET key`
- `SET key value [EX seconds]`
- `PUBLISH channel message`
- `SUBSCRIBE channel [channel ...]`
