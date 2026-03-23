use collections::HashSet;
use rand::Rng;

const ADJECTIVES: &[&str] = &[
    "able", "agile", "amber", "ample", "aqua", "azure", "bold", "brave", "brief", "bright",
    "broad", "calm", "clean", "clear", "clever", "cool", "coral", "crisp", "deft", "eager",
    "earnest", "even", "fair", "fast", "fine", "firm", "fleet", "fond", "frank", "fresh", "full",
    "gentle", "glad", "golden", "grand", "green", "hale", "happy", "hardy", "humble", "ideal",
    "ivory", "jade", "jovial", "keen", "kind", "light", "lively", "lucid", "lunar", "mellow",
    "merry", "mild", "misty", "modest", "mossy", "noble", "novel", "oaken", "olive", "opal",
    "open", "outer", "pastel", "pearl", "placid", "plain", "plum", "poised", "polished", "prime",
    "proud", "pure", "quick", "quiet", "rapid", "ready", "regal", "rosy", "ruby", "rustic", "sage",
    "sandy", "scenic", "serene", "sharp", "silver", "sleek", "smart", "smooth", "snowy", "solar",
    "solid", "spry", "stark", "steady", "still", "stoic", "stout", "sunny", "sure", "swift",
    "tawny", "teal", "tidy", "topaz", "trim", "upper", "vast", "velvet", "vivid", "warm", "whole",
    "wise", "witty", "young", "zesty",
];

const NOUNS: &[&str] = &[
    "anchor", "arch", "atlas", "badge", "basin", "bay", "beam", "bell", "birch", "blade", "bloom",
    "bluff", "bolt", "bower", "breeze", "bridge", "brook", "cabin", "canyon", "cape", "cedar",
    "cliff", "cloud", "coast", "colt", "cove", "crane", "creek", "crest", "dale", "dawn", "delta",
    "drift", "dune", "dusk", "eagle", "echo", "elm", "ember", "falcon", "fern", "ferry", "field",
    "finch", "fjord", "flame", "flint", "forge", "frost", "gate", "glade", "glen", "gorge",
    "grove", "harbor", "haven", "hawk", "heath", "hedge", "heron", "hill", "hollow", "isle", "ivy",
    "lake", "lantern", "larch", "lark", "leaf", "ledge", "lily", "lodge", "loft", "mantle",
    "marsh", "meadow", "mesa", "mill", "moon", "north", "oak", "oasis", "orbit", "osprey", "otter",
    "pass", "path", "peak", "pebble", "pier", "pine", "plover", "plume", "pond", "quail", "rain",
    "range", "raven", "reef", "ridge", "river", "robin", "shore", "sky", "slate", "slope", "snow",
    "spark", "sparrow", "spruce", "star", "stone", "storm", "stream", "summit", "terrace", "thorn",
    "tide", "timber", "torch", "tower", "trail", "vale", "valley", "vista", "willow", "wren",
    "zenith",
];

/// Generates a branch name in `"adjective-noun"` format (e.g. `"swift-falcon"`).
///
/// Tries up to 100 random combinations, skipping any name that already appears
/// in `existing_branches`. Returns `None` if no unused name is found.
pub fn generate_branch_name(existing_branches: &[&str], rng: &mut impl Rng) -> Option<String> {
    let existing: HashSet<&str> = existing_branches.iter().copied().collect();

    for _ in 0..100 {
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
    fn test_generate_branch_name_format(mut rng: StdRng) {
        let name = generate_branch_name(&[], &mut rng).unwrap();
        let (adjective, noun) = name.split_once('-').expect("name should contain a hyphen");
        assert!(
            ADJECTIVES.contains(&adjective),
            "{adjective:?} is not in ADJECTIVES"
        );
        assert!(NOUNS.contains(&noun), "{noun:?} is not in NOUNS");
    }

    #[gpui::test(iterations = 100)]
    fn test_generate_branch_name_avoids_existing(mut rng: StdRng) {
        let existing = &["swift-falcon", "calm-river", "bold-cedar"];
        let name = generate_branch_name(existing, &mut rng).unwrap();
        for &branch in existing {
            assert_ne!(
                name, branch,
                "generated name should not match an existing branch"
            );
        }
    }

    #[gpui::test]
    fn test_generate_branch_name_returns_none_when_stuck(mut rng: StdRng) {
        let all_names: Vec<String> = ADJECTIVES
            .iter()
            .flat_map(|adj| NOUNS.iter().map(move |noun| format!("{adj}-{noun}")))
            .collect();
        let refs: Vec<&str> = all_names.iter().map(|s| s.as_str()).collect();
        let result = generate_branch_name(&refs, &mut rng);
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
