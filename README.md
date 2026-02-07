# 👕 Digital Closet (CLI)

**Digital Closet** is a Rust-based command-line tool that generates **outfit ideas and direct shopping links (Germany-focused)** based on your event, season, gender style, sizes, accessories preference, and budget.

Instead of vague fashion advice, it converts structured inputs into **search-optimized keywords** and links them directly to trusted German retailers like **H&M, Zara, Adidas, Nike, Mango, Massimo Dutti, and Zalando**.

---

## ✨ What This Tool Does

- Accepts structured fashion constraints (event, season, size, budget, etc.)
- Generates a clear, human-readable outfit idea
- Builds optimized shopping keywords automatically
- Outputs direct brand links (no Google, no scraping)
- Designed specifically for the German (DE) retail ecosystem

---

## 🧠 How It Works (Core Logic)

1. **CLI Parsing**
   - Uses `clap` with `Parser`, `Subcommand`, and `ValueEnum`
   - Ensures only valid inputs (enums, ranges, defaults)

2. **Outfit Logic**
   - `event` decides the base outfit style (formal, party, casual)
   - `season` adds weather-appropriate context
   - `gender`, `size`, and `shoe size` refine intent
   - `accessories` optionally expand scope
   - `budget_max` biases keyword results toward affordability

3. **Keyword Engine**
   - All inputs are merged into a single keyword string
   - Keywords are URL-encoded for safe use in search URLs

4. **Retail Linking**
   - Uses official brand search URLs where supported
   - Falls back to official landing pages when needed
   - No Google redirects, no unofficial APIs

---

## 📦 Installation

### Prerequisites
- Rust (stable): https://rustup.rs

### Clone & Build
```bash
git clone https://github.com/your-username/digital-closet.git
cd digital-closet
cargo build --release

Binary location:
target/release/digital-closet
(Optional) Add to PATH:
cp target/release/digital-closet /usr/local/bin/

🚀 Usage
Command Syntax
digital-closet recommend <EVENT> <SEASON> <GENDER> \
  --size <SIZE> \
  --shoe-size <EU_SIZE> \
  [--accessories] \
  [--budget-min <EUR>] \
  [--budget-max <EUR>]

🧪 Example
Command
digital-closet recommend wedding summer men \
  --size M \
  --shoe-size 43 \
  --accessories \
  --budget-min 100 \
  --budget-max 300

Output
Digital Closet (DE)
Event: Wedding | Season: Summer | Gender: Men
Clothing size: M | Shoe size (EU): 43 | Accessories: yes
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

🎯 Supported Values
Events
wedding
party
casual
Seasons
winter
spring
summer
autumn
Gender Style
men
women
unisex
Clothing Size
S
M
L
Shoe Size
EU 40–45 (validated)

🔒 Design Principles
Deterministic: same input → same output
Zero scraping, zero unofficial APIs
Brand-safe, official links only
CLI-first, scriptable, automation-friendly
Easy to extend with new events, brands, or regions

🔮 Future Ideas
Color preferences
Region switching (EU / US)
Price scraping behind a feature flag
Outfit history export (JSON)
Smarter layering rules

🛠 Built With
Rust
Clap (CLI parsing)
No external APIs


