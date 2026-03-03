# Yahar Store

A minimal TCP-based in-memory key-value store written in Rust.

## Overview

Yahar Store is a simple Redis-inspired database built from scratch using raw TCP sockets.

---

## Features

- TCP server implementation
- Multithreaded client handling
- Shared in-memory key-value store
- Basic command protocol
- String value support (multi-word values supported)

---

## Supported Commands

```bash
set <key> <value>
get <key>
delete <key>
```
---

## Future Improvements

- Implement RESP-style protocol
- Add persistent storage
- Implement Custom CLI client
- Improve Error Handling
- Improve handling Concurrent Users

---

## Purpose

To understand
- Basic protocol Design
- TCP networking fundamentals
- Shared mutable state in multithreaded systems
- Synchronization in Rust

Not intended for production use.


---
