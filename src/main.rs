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
    shoe_size: u8, // range validation via clap [web:205]

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
    let cli = Cli::parse(); // clap derive parse entrypoint [web:93]

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

    // Germany / Europe oriented links:
    // Use direct on-site search where simple & stable; otherwise use Google "site:" links (robust, no scraping).
    let links: Vec<(&str, String)> = vec![
        // Direct store search links
        ("Zara", format!("https://www.zara.com/search?searchTerm={q}")),
        ("H&M (DE)", format!("https://www2.hm.com/de_de/search-results.html?q={q}")),
        // Google site-search links (DE)
        ("Zalando (DE)", google_site_search("zalando.de", &keywords)),
        ("Jack & Jones (DE/EU)", google_site_search("jackjones.com", &keywords)),
        ("ABOUT YOU (DE)", google_site_search("aboutyou.de", &keywords)),
        ("Peek & Cloppenburg (DE)", google_site_search("peek-cloppenburg.de", &keywords)),
        ("Mango (DE)", google_site_search("shop.mango.com/de", &keywords)),
        ("Massimo Dutti (DE)", google_site_search("massimodutti.com/de", &keywords)),
        ("Bershka (DE)", google_site_search("bershka.com/de", &keywords)),
        ("Pull&Bear (DE)", google_site_search("pullandbear.com/de", &keywords)),
        ("Stradivarius (DE)", google_site_search("stradivarius.com/de", &keywords)),
    ];

    println!("Links:");
    for (brand, url) in links {
        println!("- {brand}: {url}");
    }
}

fn google_site_search(domain: &str, keywords: &str) -> String {
    // Google uses q= for the query string [web:163]
    let query = format!("site:{domain} {keywords}");
    let q = url_encode(&query);
    format!("https://www.google.com/search?q={q}")
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

    // Keep it readable for store searches
    let keywords = format!(
        "{gender_kw} {base_kw} {season_kw} {size_kw} {shoe_kw} {accessory_kw} budget {}",
        args.budget_max
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    (idea, keywords)
}

// Minimal URL encoding so links work for spaces and a few common characters.
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
