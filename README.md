# 👕 Digital Closet (CLI)

Digital Closet is a Rust-based command-line tool that generates **outfit ideas and direct shopping links** based on your **event, season, gender style, clothing size, shoe size, accessories preference, budget, and region**.

It converts structured inputs into **search-optimized keywords** and links directly to **official brand stores** — no Google redirects, no scraping, no third-party APIs.

Currently supported regions:
- 🇩🇪 Germany (`de`)

---

## ✨ Features:

- Outfit recommendations based on real constraints
- Budget-aware keyword generation
- Region-specific retailer links (DE)
- Deterministic output (same input → same output)
- CLI-first, scriptable, automation-friendly
- Built entirely in Rust

---

## 🧠 How It Works:

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
   - Germany uses different official brand ecosystems

---

## 🌍 Supported Regions:

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

## 📦 Clone & Build:

### Requirements
- Rust (stable): https://rustup.rs

### Build Steps
bash
```
git clone:
https://github.com/your-username/digital-closet.git
cd digital-closet
cargo build --release
```
---

## 📟 Binary Location:
target/release/digital-closet

## ⌥ (Optional) Add to PATH:
cp target/release/digital-closet /usr/local/bin/

---

## 🚀 Usage:
Command Syntax

bash
```
digital-closet recommend <EVENT> <SEASON> <GENDER> \
  --size <SIZE> \
  --shoe-size <EU_SIZE> \
  --region <de|it> \
  [--accessories] \
  [--budget-min <EUR>] \
  [--budget-max <EUR>]
```

## 🧪 Example (Germany)

Command

bash
```
digital-closet recommend wedding summer men \
  --size M \
  --shoe-size 43 \
  --region de \
  --accessories \
  --budget-min 100 \
  --budget-max 300
```

## ⨳ Output:

```
text

Digital Closet (DE)

Event: Wedding
Season: Summer
Gender: Men

Clothing size: M
Shoe size (EU): 43
Accessories: yes
Budget: €100–€300

Outfit idea:
Formal look: tailored outfit + smart shoes + subtle accessories

Keywords:
men wedding formal blazer trousers shirt summer linen size m shoes eu 43 accessories belt watch bag budget 300

Links:
- H&M (DE)
- Zara (DE)
- Adidas (DE)
- Nike (DE)
- Mango (DE)
- Massimo Dutti (DE)
- Zalando (DE)
- Jack & Jones (DE)
- Loro Piana
```
---
## 🎯 Supported Values:

Events:

- 💒  Weddings
- 🎉 Parties
- 🥷🏻 Casual

Seasons:

- 🥶 Winter
- ⛲️ Spring
- ☀ Summer
- 🍁 Autumn

Gender Style:

- Men
- Women
- Unisex

Clothing Size:

- S
- M
- L

Shoe Size:

- 👟 EU 40–45 (validated)

---
## ⚜️ Design Principles:

- Deterministic behavior
- Zero scraping
- Zero unofficial APIs
- Official brand links only
- Simple, readable, extendable logic

---
## 🔮 Future Ideas:

- More regions (FR, ES, UK)
- Color preferences
- Price-aware filtering
- JSON output
- Smarter layering rules

---
## 🛠 Built With:
- Rust
- Clap (CLI argument parsing)
- No external APIs

