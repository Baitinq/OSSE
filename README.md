# OSSE Search Engine

> Overly Simple Search Engine - Making search engines simple?? \
> Pronunciation: "oh-see"

![contributors](https://img.shields.io/github/contributors/baitinq/OSSE.svg)  [![license](https://img.shields.io/github/license/baitinq/OSSE.svg)](https://github.com/Baitinq/OSSE/blob/master/LICENSE)  [![PRs welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg)](https://github.com/baitinq/OSSE/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)  ![Stars](https://img.shields.io/github/stars/baitinq/OSSE.svg)  ![Forks](https://img.shields.io/github/forks/baitinq/OSSE.svg)  [![code with hearth by Baitinq](https://img.shields.io/badge/%3C%2F%3E%20with%20%E2%99%A5%20by-Baitinq-ff1414.svg)](https://github.com/Baitinq)

<img src="https://raw.githubusercontent.com/Baitinq/OSSE/master/docs/frontend.png" />


## 🚩 Table of Contents

- [Why?](#-why)
- [Usage](#-usage)
- [Features](#-features)
- [How it works](#-how-it-works)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)

## 🐂 Why?

[Just for fun!](https://justforfunnoreally.dev) I really wanted to learn [Rust](https://rust-lang.org) and at the time I was really interested in how search engines worked, so there wasn't any better way of achieving both goals than with this very project!

## 🤖 Usage

This repository is a monorepo formed by the independent components that form the OSSE search engine.

### Installing Dependencies

* #### With [Nix](https://nixos.org/):

```
$ nix develop
```

* #### Otherwise:

Install [cargo](https://doc.rust-lang.org/cargo/) and [trunk](https://trunkrs.dev) with your preferred method (such as your favorite package manager).

### Running

* [Crawler](https://github.com/Baitinq/OSSE/tree/master/crawler)
```
$ cargo run --bin crawler
```

* [Indexer](https://github.com/Baitinq/OSSE/tree/master/indexer)

```
$ cargo run --bin indexer
```

* [Frontend](https://github.com/Baitinq/OSSE/tree/master/frontend)
```
$ trunk serve frontend/index.html --open
```

Once all the components are running, you can navigate to ```127.0.0.1:8080``` on your favorite web browser and start using OSSE!

## 🎨 Features

* [Completely Self-Hosted](https://en.wikipedia.org/wiki/Self-hosting_(web_services)) : OSSE does not use any external services, all you need is its three components (indexer, crawler & frontend) to have a "complete" search engine.
* [Custom Indexing and Ranking Algorithms](https://github.com/Baitinq/OSSE/tree/master/indexer) : OSSE uses its own open-source indexing and ranking algorithm, meaning that its code is reviewable and improvable by third parties, ensuring its technically and morally correct functionality.
* [Hackable]() : OSSE is built with extensibility & modularity in mind, so it is entirely feasible to replace or customize its various components.
* [Privacy Respecting]() : As a result of OSSE being completely independent, it does not send any metadata to any services.


## ⚙️  How it works

The OSSE search engine is separated into three independent components:

* ### [Indexer](https://github.com/Baitinq/OSSE/tree/master/indexer)
This component provides both the actual search engine indexer's implementation and the REST API used to search and add indexed resources. It uses [Actix Web](https://actix.rs) for the REST API (running on port 4444). For the implementation of the actual indexer data structure, we currently use a very simple reverse index implemented with a hashmap, so all the indexed resources are currently lost each time the indexer is restarted.

* ### [Crawler](https://github.com/Baitinq/OSSE/tree/master/crawler)
This component is a simple recursive crawler that forwards the crawled raw HTML to the indexer. It uses [reqwest](https://docs.rs/reqwest/latest/reqwest) for fetching a predefined list of [root websites](https://github.com/Baitinq/OSSE/blob/master/crawler/top-1000-websites.txt) and parses them with [scraper](https://docs.rs/scraper/latest/scraper), sending the website contents to the indexer and extracting all its links, adding them to a queue of websites to be crawled. This process is "recursively" repeated indefinitely.

* ### [Frontend](https://github.com/Baitinq/OSSE/tree/master/frontend)
This component is a simple web interface to the indexer. It allows users to search and visualize results in a user friendly way. It is currently built using [Yew](https://yew.rs), which allows us to write the frontend in rust and produce a "blazingly fast" Wasm based web-ui.

## 🐾 Roadmap

- [x] Add frontend
- [ ] Change indexer to use a ngram index instead of a reverse index
- [ ] Improve frontend
- [ ] Improve responsiveness of searching when the indexer is recieving info from crawlers
- [ ] Rust cleanup
- [ ] Improve page ranking algorithm


## 💬 Contributing

> "If you have any ideas or patches, please do not hesitate to contribute to OSSE!"

## 📜 License

This software is licensed under the [BSD-2-Clause](https://github.com/baitinq/OSSE/blob/master/LICENSE) © [Baitinq](https://github.com/Baitinq).

