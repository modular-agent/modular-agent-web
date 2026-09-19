# Web Modules for Modular Agent

Web automation modules for [Modular Agent](https://github.com/modular-agent/modular-agent):
HTTP requests, HTML scraping, web search, and content extraction.

## Modules

| Module | Purpose | Inputs | Outputs |
| ----- | ------- | ------ | ------- |
| FetchUrlModule | HTTP GET request | `url` | `text` |
| HtmlScraperModule | CSS selector extraction | `html` | `html[]` |
| HtmlToMarkdownModule | HTML to Markdown conversion | `html` | `markdown` |
| SearxngSearchModule | Web search via SearXNG JSON API | `query` | `results` |
| FetchYtTranscriptModule | YouTube transcript extraction | `url` or `video_id` | `transcript`, `text` |

## Features

| Feature | Modules | Extra dependencies |
| ------- | ------ | ------------------ |
| `fetch-url` | FetchUrlModule | - |
| `html-scraper` | HtmlScraperModule | scraper |
| `html-to-markdown` | HtmlToMarkdownModule | html-to-markdown-rs |
| `searxng` | SearxngSearchModule | - |
| `yt-transcript` | FetchYtTranscriptModule | quick-xml |

All features are enabled by default.

## Usage

This crate builds as part of the
[modular-agent monorepo](https://github.com/modular-agent/modular-agent). Clone it
into the monorepo's `custom_modules/` directory and select it with the ma-config
wizard:

```sh
cd modular-agent/custom_modules
git clone https://github.com/modular-agent/modular-agent-web.git
cd ..
cargo run --manifest-path tools/ma-config/Cargo.toml -- desktop   # or: cli
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE_APACHE-2.0) or
[MIT license](LICENSE_MIT) at your option.
