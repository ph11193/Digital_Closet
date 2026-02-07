# 👕 Digital Closet (CLI)

Digital Closet is a Rust-based command-line tool that generates **outfit ideas and direct shopping links** based on your **event, season, gender style, clothing size, shoe size, accessories preference, budget, and region**.

It converts structured inputs into **search-optimized keywords** and links directly to **official brand stores** — no Google redirects, no scraping, no third-party APIs.

Currently supported regions:
- 🇩🇪 Germany (`de`)

---

## ✨ Features

- Outfit recommendations based on real constraints
- Budget-aware keyword generation
- Region-specific retailer links (DE / IT)
- Deterministic output (same input → same output)
- CLI-first, scriptable, automation-friendly
- Built entirely in Rust

---

## 🧠 How It Works

1. **CLI Parsing**
   - Uses `clap` with enums and validators
   - Prevents invalid inputs (sizes, ranges, enums)

2. **Outfit Logic**
   - Event defines the base outfit (formal, party, casual)
   - Season adjusts materials and layering
   - Gender, clothing size, and shoe size refine intent
   - Accessories optionally expand scope
   - Budget biases keyword relevance

3. **Keyword Engine**
   - All inputs are merged into one optimized keyword string
   - Keywords are URL-encoded for safe search URLs

4. **Region Switching**
   - Region only affects **retailers**, not outfit logic
   - Germany and Italy use different official brand ecosystems

---

## 🌍 Supported Regions

### Germany (`de`)
- H&M
- Zara
- Adidas
- Nike
- Mango
- Massimo Dutti
- Zalando
- Jack & Jones
- Loro Piana

---

## 📦 Clone & Build

### Requirements
- Rust (stable): https://rustup.rs

### Build Steps
```bash
git clone https://github.com/your-username/digital-closet.git
cd digital-closet
cargo build --release

