use collections::HashSet;
use rand::Rng;

const ADJECTIVES: &[&str] = &[
    "able", "agate", "agile", "alpine", "amber", "ample", "aqua", "arctic", "arid", "astral",
    "autumn", "avid", "azure", "balmy", "birch", "bold", "boreal", "brave", "breezy", "brief",
    "bright", "brisk", "broad", "bronze", "calm", "cerith", "civil", "clean", "clear", "clever",
    "cobalt", "cool", "copper", "coral", "cozy", "crisp", "cubic", "cyan", "deft", "dense", "dewy",
    "direct", "dusky", "dusty", "eager", "early", "earnest", "elder", "elfin", "equal", "even",
    "exact", "faint", "fair", "fast", "fawn", "ferny", "fiery", "fine", "firm", "fleet", "floral",
    "focal", "fond", "frank", "fresh", "frosty", "full", "gentle", "gilded", "glacial", "glad",
    "glossy", "golden", "grand", "green", "gusty", "hale", "happy", "hardy", "hazel", "hearty",
    "hilly", "humble", "hushed", "icy", "ideal", "inner", "iron", "ivory", "jade", "jovial",
    "keen", "kind", "lapis", "leafy", "level", "light", "lilac", "limber", "lively", "local",
    "lofty", "lucid", "lunar", "major", "maple", "mellow", "merry", "mild", "milky", "misty",
    "modal", "modest", "mossy", "muted", "native", "naval", "neat", "nimble", "noble", "north",
    "novel", "oaken", "ochre", "olive", "onyx", "opal", "open", "optic", "outer", "owed", "ozone",
    "pale", "pastel", "pearl", "pecan", "peppy", "pilot", "placid", "plain", "plum", "plush",
    "poised", "polar", "polished", "poplar", "prime", "proof", "proud", "pure", "quartz", "quick",
    "quiet", "rapid", "raspy", "ready", "regal", "rooted", "rosy", "round", "royal", "ruby",
    "ruddy", "russet", "rustic", "sage", "salty", "sandy", "satin", "scenic", "sedge", "serene",
    "sharp", "sheer", "silky", "silver", "sleek", "smart", "smooth", "snowy", "solar", "solid",
    "south", "spry", "stark", "steady", "steel", "steep", "still", "stoic", "stony", "stout",
    "sturdy", "suede", "sunny", "supple", "sure", "swift", "tall", "tawny", "teal", "terse",
    "thick", "tidal", "tidy", "timber", "topaz", "total", "trim", "tropic", "true", "tulip",
    "upper", "urban", "valid", "vast", "velvet", "verde", "vivid", "vocal", "warm", "waxen",
    "west", "whole", "wide", "wild", "wise", "witty", "woven", "young", "zealous", "zephyr",
    "zesty", "zinc",
];

const NOUNS: &[&str] = &[
    "anchor", "anvil", "arbor", "arch", "arrow", "atlas", "badge", "badger", "basin", "bay",
    "beacon", "beam", "bell", "birch", "blade", "bloom", "bluff", "bolt", "bower", "breeze",
    "bridge", "brook", "bunting", "cabin", "cairn", "canyon", "cape", "cedar", "chasm", "cliff",
    "cloud", "clover", "coast", "cobble", "colt", "comet", "condor", "coral", "cove", "crane",
    "crater", "creek", "crest", "curlew", "cypress", "dale", "dawn", "delta", "den", "dove",
    "drake", "drift", "drum", "dune", "dusk", "eagle", "echo", "egret", "elk", "elm", "ember",
    "falcon", "fawn", "fern", "ferry", "field", "finch", "fjord", "flame", "flint", "flower",
    "forge", "fossil", "fox", "frost", "gale", "garnet", "gate", "gazelle", "geyser", "glade",
    "glen", "gorge", "granite", "grove", "gull", "harbor", "hare", "haven", "hawk", "hazel",
    "heath", "hedge", "heron", "hill", "hollow", "horizon", "ibis", "inlet", "isle", "ivy",
    "jackal", "jasper", "juniper", "kestrel", "kinglet", "knoll", "lagoon", "lake", "lantern",
    "larch", "lark", "laurel", "lava", "leaf", "ledge", "lily", "linden", "lodge", "loft", "lotus",
    "lynx", "mantle", "maple", "marble", "marsh", "marten", "meadow", "merlin", "mesa", "mill",
    "mint", "moon", "moose", "moss", "newt", "north", "nutmeg", "oak", "oasis", "obsidian",
    "orbit", "orchid", "oriole", "osprey", "otter", "owl", "palm", "panther", "pass", "path",
    "peak", "pebble", "pelican", "peony", "perch", "pier", "pine", "plover", "plume", "pond",
    "poppy", "prairie", "prism", "puma", "quail", "quarry", "quartz", "rain", "rampart", "range",
    "raven", "ravine", "reed", "reef", "ridge", "river", "robin", "rowan", "sage", "salmon",
    "sequoia", "shore", "shrike", "sigma", "sky", "slate", "slope", "snow", "spark", "sparrow",
    "spider", "spruce", "stag", "star", "stone", "stork", "storm", "stream", "summit", "swift",
    "sycamore", "tern", "terrace", "thistle", "thorn", "thrush", "tide", "timber", "torch",
    "tower", "trail", "trout", "tulip", "tundra", "vale", "valley", "veranda", "viper", "vista",
    "vole", "walrus", "warbler", "willow", "wolf", "wren", "yew", "zenith",
];

/// Generates a worktree name in `"adjective-noun"` format (e.g. `"swift-falcon"`).
///
/// Tries up to 10 random combinations, skipping any name that already appears
/// in `existing_names`. Returns `None` if no unused name is found.
pub fn generate_worktree_name(existing_names: &[&str], rng: &mut impl Rng) -> Option<String> {
    let existing: HashSet<&str> = existing_names.iter().copied().collect();

    for _ in 0..10 {
        let adjective = ADJECTIVES[rng.random_range(0..ADJECTIVES.len())];
        let noun = NOUNS[rng.random_range(0..NOUNS.len())];
        let name = format!("{adjective}-{noun}");

        if !existing.contains(name.as_str()) {
            return Some(name);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;

    #[gpui::test(iterations = 10)]
    fn test_generate_worktree_name_format(mut rng: StdRng) {
        let name = generate_worktree_name(&[], &mut rng).unwrap();
        let (adjective, noun) = name.split_once('-').expect("name should contain a hyphen");
        assert!(
            ADJECTIVES.contains(&adjective),
            "{adjective:?} is not in ADJECTIVES"
        );
        assert!(NOUNS.contains(&noun), "{noun:?} is not in NOUNS");
    }

    #[gpui::test(iterations = 100)]
    fn test_generate_worktree_name_avoids_existing(mut rng: StdRng) {
        let existing = &["swift-falcon", "calm-river", "bold-cedar"];
        let name = generate_worktree_name(existing, &mut rng).unwrap();
        for &branch in existing {
            assert_ne!(
                name, branch,
                "generated name should not match an existing branch"
            );
        }
    }

    #[gpui::test]
    fn test_generate_worktree_name_returns_none_when_stuck(mut rng: StdRng) {
        let all_names: Vec<String> = ADJECTIVES
            .iter()
            .flat_map(|adj| NOUNS.iter().map(move |noun| format!("{adj}-{noun}")))
            .collect();
        let refs: Vec<&str> = all_names.iter().map(|s| s.as_str()).collect();
        let result = generate_worktree_name(&refs, &mut rng);
        assert!(result.is_none());
    }

    #[test]
    fn test_adjectives_are_valid() {
        let mut seen = HashSet::default();
        for &word in ADJECTIVES {
            assert!(seen.insert(word), "duplicate entry in ADJECTIVES: {word:?}");
        }

        for window in ADJECTIVES.windows(2) {
            assert!(
                window[0] < window[1],
                "ADJECTIVES is not sorted: {0:?} should come before {1:?}",
                window[0],
                window[1],
            );
        }

        for &word in ADJECTIVES {
            assert!(
                !word.contains('-'),
                "ADJECTIVES entry contains a hyphen: {word:?}"
            );
            assert!(
                word.chars().all(|c| c.is_lowercase()),
                "ADJECTIVES entry is not all lowercase: {word:?}"
            );
        }
    }

    #[test]
    fn test_nouns_are_valid() {
        let mut seen = HashSet::default();
        for &word in NOUNS {
            assert!(seen.insert(word), "duplicate entry in NOUNS: {word:?}");
        }

        for window in NOUNS.windows(2) {
            assert!(
                window[0] < window[1],
                "NOUNS is not sorted: {0:?} should come before {1:?}",
                window[0],
                window[1],
            );
        }

        for &word in NOUNS {
            assert!(
                !word.contains('-'),
                "NOUNS entry contains a hyphen: {word:?}"
            );
            assert!(
                word.chars().all(|c| c.is_lowercase()),
                "NOUNS entry is not all lowercase: {word:?}"
            );
        }
    }
}
