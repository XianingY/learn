# VortexGate

基于 ConnectRPC + Vanguard 的轻量网关服务，提供 HTTP/JSON 到 RPC 的转发与示例 Echo 接口。

## 功能
- Echo 接口：返回带时间戳的响应
- HTTP/JSON -> RPC 转发（Vanguard Transcoder）
- 本地 h2c (HTTP/2 Cleartext) 支持

## 技术栈
- Go 1.25
- connectrpc/connect + vanguard
- Buf (proto 生成)

## 运行
```bash
go run ./cmd/server
```

默认端口：8080（可通过环境变量 `PORT` 配置）

## 生成代码
```bash
buf generate
```

## API
- GET `/v1/echo/{message}`

示例：
```bash
curl http://localhost:8080/v1/echo/hello
```

返回示例：
```json
{
  "message": "VortexGate says: hello",
  "timestamp": "2026-02-04T12:00:00Z"
}
```
