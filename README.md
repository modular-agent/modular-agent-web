# Web Agents for Modular Agent

Web automation agents for [Modular Agent](https://github.com/modular-agent/modular-agent):
HTTP requests, HTML scraping, web search, and content extraction.

## Agents

| Agent | Purpose | Inputs | Outputs |
| ----- | ------- | ------ | ------- |
| FetchUrlAgent | HTTP GET request | `url` | `text` |
| HtmlScraperAgent | CSS selector extraction | `html` | `html[]` |
| HtmlToMarkdownAgent | HTML to Markdown conversion | `html` | `markdown` |
| SearxngSearchAgent | Web search via SearXNG JSON API | `query` | `results` |
| FetchYtTranscriptAgent | YouTube transcript extraction | `url` or `video_id` | `transcript`, `text` |

## Features

| Feature | Agents | Extra dependencies |
| ------- | ------ | ------------------ |
| `fetch-url` | FetchUrlAgent | - |
| `html-scraper` | HtmlScraperAgent | scraper |
| `html-to-markdown` | HtmlToMarkdownAgent | html-to-markdown-rs |
| `searxng` | SearxngSearchAgent | - |
| `yt-transcript` | FetchYtTranscriptAgent | quick-xml |

All features are enabled by default.

## Usage

This crate builds as part of the
[modular-agent monorepo](https://github.com/modular-agent/modular-agent). Clone it
into the monorepo's `custom_agents/` directory and select it with the ma-config
wizard:

```sh
cd modular-agent/custom_agents
git clone https://github.com/modular-agent/modular-agent-web.git
cd ..
cargo run --manifest-path tools/ma-config/Cargo.toml -- desktop   # or: cli
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE_APACHE-2.0) or
[MIT license](LICENSE_MIT) at your option.
