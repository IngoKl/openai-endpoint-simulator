# OpenAI Endpoint Simulator

This tool simulates a basic OpenAI [Chat Completions](https://platform.openai.com/docs/guides/chat-completions) endpoint. This is a minimal implementation of the API – do *not* expect any special features to work.

This tool can, for example, be used to test applications using a locally hosted LLM. It is intended as a quick stand-in for simple tests and experimentation.

## Usage

The endpoint is written in Rust. Hence, `cargo` can be used. For example:

```bash
cargo run
```

There is also a Python `test.py` script that calls the endpoint using the `openai` library.

```bash
python test.py
```
