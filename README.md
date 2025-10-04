# MDJournal

**MDJournal** is a simple, terminal-based Markdown journaling tool written in Rust.  
It prompts you with daily reflective questions, formats your answers into Markdown, and saves them as dated `.md` files. This project is extremely simple, I just wanted something to work on **to learn the absolute basics of Rust**.

<img width="3680" height="2488" alt="ray-so-export" src="https://github.com/user-attachments/assets/7b9137b7-dcdf-4dc1-bb0b-0f976355e037" />

[![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)

Each entry is saved as a separate Markdown file named after the current date (e.g. `2025-07-10.md`).

---

## 🚀 Getting Started

### 📦 Build Instructions

1. **Clone the repository**
```bash
git clone https://github.com/fwtwoo/mdjournal.git
cd mdjournal
```
2. **Build the project**
```bash
cargo build --release
```
3. **Run it**
```bash
cargo run
```

### ✅ Requirements

    Rust (stable) — install via rustup

### 📂 Output Format

Each journal entry is saved in this Markdown format:
<img width="920" height="653" alt="markdown-image-143009" src="https://github.com/user-attachments/assets/31151e48-a83a-4faf-8d69-bb5a5b77dfbd" />

### 📄 License
Licensed under the MIT License. See LICENSE for details.
