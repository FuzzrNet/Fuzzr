# Fuzzr

[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)
[![Build status](https://github.com/FuzzrNet/fuzzr/workflows/Rust/badge.svg)](github.com/FuzzrNet/fuzzr/actions?query=branch:master)

Fuzzr is intended to be a censorship-resistant platform for publishing, curation, and browsing of all content (with the explicit exception of HTML). At some point, an optional cryptocurrency miner will be added, so as to assist in monetization for content creators, scalable and low-latency content distribution that cannot be interfered with, and incentivizing privacy measures.

The focus is ensuring content distribution, censorship resistance, and keeping users as anonymous as they care to be. No user registration required. Encryption of content at rest isn't necessarily a focus for this project, but ensuring secure, unfettered encrypted connections to all anonymized peers is.

This is an architecture for an actual locally-run p2p app, not a simple web-based "decentralized app".

Built with Rust, for native desktop platforms, using [iced](https://github.com/hecrj/iced) for UI and [embedded IPFS](https://github.com/ipfs-rust/ipfs-embed/).

## Status

This project is a massive work in progress. Not all of these goals are currently met, but we do intend to. More thoughts in the notes directory. Feedback appreciated. Drop us an issue.

Only desktop Linux platforms are supported for now. Support is planned for all platforms except for web. This project is intended to make the web and all web technologies less necessary than they once were by doing what people use the web for in a standardized and hyper-minimal way.

## Support development!

Feel free to send some XMR to the address below. And if you do so, feel free to reach out, or promote what we're doing.

`8ADbBKaunVWjdg5aWQ5ZBNDACdPVMTUBnKETaZbUZ8gMfDfpwhcBeo31kfUgCJKATMPaqmsUoxBwicTpRLg4p4F57kPJ5ab`

If you'd like to get in touch with us, just send an email to those listed in the Cargo.toml
