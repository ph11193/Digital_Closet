use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "digital-closet",
    version,
    about = "Outfit ideas + shopping links (Germany) from your event, season, budget, sizes, and gender.",
    long_about = None,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate outfit ideas and shopping links
    Recommend(RecommendArgs),
}

#[derive(Parser, Debug, Clone)]
struct RecommendArgs {
    /// Occasion (e.g., wedding, party)
    #[arg(value_enum)]
    event: Event,

    /// Season (e.g., winter, summer)
    #[arg(value_enum)]
    season: Season,

    /// Gender style target (used for keywords)
    #[arg(value_enum)]
    gender: Gender,

    /// Clothing size (S, M, L)
    #[arg(value_enum, long)]
    size: ClothingSize,

    /// Shoe size (EU), allowed: 40..=45
    #[arg(long, value_parser = clap::value_parser!(u8).range(40..=45))]
    shoe_size: u8,

    /// Include accessories in the suggestion (adds keywords)
    #[arg(long, default_value_t = false)]
    accessories: bool,

    /// Minimum budget (EUR)
    #[arg(long, default_value_t = 0)]
    budget_min: u32,

    /// Maximum budget (EUR)
    #[arg(long, default_value_t = 150)]
    budget_max: u32,
}

#[derive(ValueEnum, Debug, Clone)]
enum Event {
    Wedding,
    Party,
    Casual,
}

#[derive(ValueEnum, Debug, Clone)]
enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}

#[derive(ValueEnum, Debug, Clone)]
enum Gender {
    Men,
    Women,
    Unisex,
}

#[derive(ValueEnum, Debug, Clone)]
enum ClothingSize {
    S,
    M,
    L,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Recommend(args) => run_recommend(args),
    }
}

fn run_recommend(args: RecommendArgs) {
    let (idea, keywords) = build_outfit_and_keywords(&args);

    println!("Digital Closet (DE)");
    println!(
        "Event: {:?} | Season: {:?} | Gender: {:?}",
        args.event, args.season, args.gender
    );
    println!(
        "Clothing size: {:?} | Shoe size (EU): {} | Accessories: {}",
        args.size,
        args.shoe_size,
        if args.accessories { "yes" } else { "no" }
    );
    println!("Budget: €{}–€{}", args.budget_min, args.budget_max);
    println!();
    println!("Outfit idea: {idea}");
    println!("Keywords: {keywords}");
    println!();

    let q = url_encode(&keywords);

    // Direct company links only (no Google).
    // Some brands support query-filled search URLs; others open a reliable landing/search area.
    let links: Vec<(&str, String)> = vec![
        // Query-filled search links
        ("H&M (DE)", format!("https://www2.hm.com/de_de/search-results.html?q={q}")),
        ("Zara (DE)", format!("https://www.zara.com/de/en/search?searchTerm={q}")),
        ("Adidas (DE)", format!("https://www.adidas.de/search?q={q}")),
        ("Nike (DE)", format!("https://www.nike.com/de/w/search?q={q}")),
        ("Mango (DE)", format!("https://shop.mango.com/de/search?query={q}")),
        ("Massimo Dutti (DE)", format!("https://www.massimodutti.com/de/search?q={q}")),

        // Official company pages (reliable open, may not pre-fill query)
        ("Zalando (DE)", "https://www.zalando.de/".to_string()),
        ("Jack & Jones (DE)", "https://www.jackjones.com/de-de".to_string()),
        ("Loro Piana", "https://www.loropiana.com".to_string()),
    ];

    println!("Links:");
    for (brand, url) in links {
        println!("- {brand}: {url}");
    }
}

fn build_outfit_and_keywords(args: &RecommendArgs) -> (String, String) {
    let gender_kw = match args.gender {
        Gender::Men => "men",
        Gender::Women => "women",
        Gender::Unisex => "unisex",
    };

    let size_kw = match args.size {
        ClothingSize::S => "size s",
        ClothingSize::M => "size m",
        ClothingSize::L => "size l",
    };

    let (idea, base_kw) = match args.event {
        Event::Wedding => (
            "Formal look: tailored outfit + smart shoes + subtle accessories".to_string(),
            "wedding formal blazer trousers shirt".to_string(),
        ),
        Event::Party => (
            "Party look: statement top + clean shoes + simple layering".to_string(),
            "party outfit going out".to_string(),
        ),
        Event::Casual => (
            "Casual look: comfy basics + light layer + sneakers".to_string(),
            "casual outfit basics".to_string(),
        ),
    };

    let season_kw = match args.season {
        Season::Winter => "winter coat knit",
        Season::Spring => "spring jacket",
        Season::Summer => "summer linen",
        Season::Autumn => "autumn layering",
    };

    let accessory_kw = if args.accessories {
        "accessories belt watch bag"
    } else {
        ""
    };

    let shoe_kw = format!("shoes eu {}", args.shoe_size);

    let keywords = format!(
        "{gender_kw} {base_kw} {season_kw} {size_kw} {shoe_kw} {accessory_kw} budget {}",
        args.budget_max
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    (idea, keywords)
}

fn url_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' => c.to_string(),
            ' ' => "+".to_string(),
            '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
