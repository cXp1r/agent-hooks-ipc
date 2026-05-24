# agent-hooks-ipc
## TODO
- [ ] Claude Code适配
- [ ] Codex适配


## 快速开始

```bash
# 启动服务端
cargo run --bin server

# 发送请求
cargo run --bin client -- claude < src/xxx.json
```

## 协议

客户端与服务端通过本地命名管道通信，每行一个完整 JSON。

### 请求格式

```json
{"action": "<action>", "payload": <任意 JSON>}
```

### 响应格式
根据不同agent, stdout略有不同
```json
{"status": "ok/timeout/error", "stdout": <任意 JSON>}
```
## 测试

```bash
# 测试会自动启动 server，用 xxx.json 作为输入
cargo test -- --nocapture
```

测试默认读取同目录下的 `xxx.json`，使用 `claude` 作为默认 action。
