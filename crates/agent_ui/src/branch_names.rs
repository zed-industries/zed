use collections::HashSet;
use rand::Rng;

/// Names of historical typewriter brands, for use in auto-generated branch names.
/// (Hyphens and parens have been dropped so that the branch names are one-word.)
///
/// Thanks to https://typewriterdatabase.com/alph.0.brands for the names!
const TYPEWRITER_NAMES: &[&str] = &[
    "abeille",
    "acme",
    "addo",
    "adler",
    "adlerette",
    "adlerita",
    "admiral",
    "agamli",
    "agar",
    "agidel",
    "agil",
    "aguia",
    "aguila",
    "ahram",
    "aigle",
    "ajax",
    "aktiv",
    "ala",
    "alba",
    "albus",
    "alexander",
    "alexis",
    "alfa",
    "allen",
    "alonso",
    "alpina",
    "amata",
    "amaya",
    "amka",
    "anavi",
    "anderson",
    "andina",
    "antares",
    "apex",
    "apsco",
    "aquila",
    "archo",
    "ardita",
    "argyle",
    "aristocrat",
    "aristokrat",
    "arlington",
    "armstrong",
    "arpha",
    "artus",
    "astoria",
    "atlantia",
    "atlantic",
    "atlas",
    "augusta",
    "aurora",
    "austro",
    "automatic",
    "avanti",
    "avona",
    "azzurra",
    "bajnok",
    "baldwin",
    "balkan",
    "baltica",
    "baltimore",
    "barlock",
    "barr",
    "barrat",
    "bartholomew",
    "bashkiriya",
    "bavaria",
    "beaucourt",
    "beko",
    "belka",
    "bennett",
    "bennington",
    "berni",
    "bianca",
    "bijou",
    "bing",
    "bisei",
    "biser",
    "bluebird",
    "bolida",
    "borgo",
    "boston",
    "boyce",
    "bradford",
    "brandenburg",
    "brigitte",
    "briton",
    "brooks",
    "brosette",
    "buddy",
    "burns",
    "burroughs",
    "byron",
    "calanda",
    "caligraph",
    "cappel",
    "cardinal",
    "carissima",
    "carlem",
    "carlton",
    "carmen",
    "cawena",
    "cella",
    "celtic",
    "century",
    "champignon",
    "cherryland",
    "chevron",
    "chicago",
    "cicero",
    "cifra",
    "citizen",
    "claudia",
    "cleveland",
    "clover",
    "coffman",
    "cole",
    "columbia",
    "commercial",
    "companion",
    "concentra",
    "concord",
    "concordia",
    "conover",
    "constanta",
    "consul",
    "conta",
    "contenta",
    "contimat",
    "contina",
    "continento",
    "cornelia",
    "coronado",
    "cosmopolita",
    "courier",
    "craftamatic",
    "crandall",
    "crown",
    "culema",
    "dactyle",
    "dankers",
    "dart",
    "daugherty",
    "davis",
    "dayton",
    "dea",
    "delmar",
    "densmore",
    "depantio",
    "diadema",
    "dial",
    "diamant",
    "diana",
    "dictatype",
    "diplomat",
    "diskret",
    "dolfus",
    "dollar",
    "domus",
    "drake",
    "draper",
    "duplex",
    "durabel",
    "dynacord",
    "eagle",
    "eclipse",
    "edelmann",
    "edelweiss",
    "edison",
    "edita",
    "edland",
    "efka",
    "eldorado",
    "electa",
    "electromatic",
    "elektro",
    "elgin",
    "elliot",
    "emerson",
    "emka",
    "emona",
    "empire",
    "engadine",
    "engler",
    "erfurt",
    "erika",
    "esko",
    "essex",
    "eureka",
    "europa",
    "everest",
    "everlux",
    "excelsior",
    "express",
    "fabers",
    "facit",
    "fairbanks",
    "faktotum",
    "famos",
    "federal",
    "felio",
    "fidat",
    "filius",
    "fips",
    "fish",
    "fitch",
    "fleet",
    "florida",
    "flott",
    "flyer",
    "flying",
    "fontana",
    "ford",
    "forto",
    "fortuna",
    "fox",
    "framo",
    "franconia",
    "franklin",
    "friden",
    "frolio",
    "furstenberg",
    "galesburg",
    "galiette",
    "gallia",
    "garbell",
    "gardner",
    "geka",
    "generation",
    "genia",
    "geniatus",
    "gerda",
    "gisela",
    "glashutte",
    "gloria",
    "godrej",
    "gossen",
    "gourland",
    "grandjean",
    "granta",
    "granville",
    "graphic",
    "gritzner",
    "groma",
    "guhl",
    "guidonia",
    "gundka",
    "hacabo",
    "haddad",
    "halberg",
    "halda",
    "hall",
    "hammond",
    "hammonia",
    "hanford",
    "hansa",
    "harmony",
    "harris",
    "hartford",
    "hassia",
    "hatch",
    "heady",
    "hebronia",
    "hebros",
    "hega",
    "helios",
    "helma",
    "herald",
    "hercules",
    "hermes",
    "herold",
    "heros",
    "hesperia",
    "hogar",
    "hooven",
    "hopkins",
    "horton",
    "hugin",
    "hungaria",
    "hurtu",
    "iberia",
    "idea",
    "ideal",
    "imperia",
    "impo",
    "industria",
    "industrio",
    "ingersoll",
    "international",
    "invicta",
    "irene",
    "iris",
    "iskra",
    "ivitsa",
    "ivriah",
    "jackson",
    "janalif",
    "janos",
    "jolux",
    "juki",
    "junior",
    "juventa",
    "juwel",
    "kamkap",
    "kamo",
    "kanzler",
    "kappel",
    "karli",
    "karstadt",
    "keaton",
    "kenbar",
    "keystone",
    "kim",
    "klein",
    "kneist",
    "knoch",
    "koh",
    "kolibri",
    "kolumbus",
    "komet",
    "kondor",
    "koniger",
    "konryu",
    "kontor",
    "kosmopolit",
    "krypton",
    "lambert",
    "lasalle",
    "lectra",
    "leframa",
    "lemair",
    "lemco",
    "liberty",
    "libia",
    "liga",
    "lignose",
    "lilliput",
    "lindeteves",
    "linowriter",
    "listvitsa",
    "ludolf",
    "lutece",
    "luxa",
    "lyubava",
    "mafra",
    "magnavox",
    "maher",
    "majestic",
    "majitouch",
    "manhattan",
    "mapuua",
    "marathon",
    "marburger",
    "maritsa",
    "maruzen",
    "maskelyne",
    "masspro",
    "matous",
    "mccall",
    "mccool",
    "mcloughlin",
    "mead",
    "mechno",
    "mehano",
    "meiselbach",
    "melbi",
    "melior",
    "melotyp",
    "mentor",
    "mepas",
    "mercedesia",
    "mercurius",
    "mercury",
    "merkur",
    "merritt",
    "merz",
    "messa",
    "meteco",
    "meteor",
    "micron",
    "mignon",
    "mikro",
    "minerva",
    "mirian",
    "mirina",
    "mitex",
    "molle",
    "monac",
    "monarch",
    "mondiale",
    "monica",
    "monofix",
    "monopol",
    "monpti",
    "monta",
    "montana",
    "montgomery",
    "moon",
    "morgan",
    "morris",
    "morse",
    "moya",
    "moyer",
    "munson",
    "musicwriter",
    "nadex",
    "nakajima",
    "neckermann",
    "neubert",
    "neya",
    "ninety",
    "nisa",
    "noiseless",
    "noor",
    "nora",
    "nord",
    "norden",
    "norica",
    "norma",
    "norman",
    "north",
    "nototyp",
    "nova",
    "novalevi",
    "odell",
    "odhner",
    "odo",
    "odoma",
    "ohio",
    "ohtani",
    "oliva",
    "oliver",
    "olivetti",
    "olympia",
    "omega",
    "optima",
    "orbis",
    "orel",
    "orga",
    "oriette",
    "orion",
    "orn",
    "orplid",
    "pacior",
    "pagina",
    "parisienne",
    "passat",
    "pearl",
    "peerless",
    "perfect",
    "perfecta",
    "perkeo",
    "perkins",
    "perlita",
    "pettypet",
    "phoenix",
    "piccola",
    "picht",
    "pinnock",
    "pionier",
    "plurotyp",
    "plutarch",
    "pneumatic",
    "pocket",
    "polyglott",
    "polygraph",
    "pontiac",
    "portable",
    "portex",
    "pozzi",
    "premier",
    "presto",
    "primavera",
    "progress",
    "protos",
    "pterotype",
    "pullman",
    "pulsatta",
    "quick",
    "racer",
    "radio",
    "rally",
    "rand",
    "readers",
    "reed",
    "referent",
    "reff",
    "regent",
    "regia",
    "regina",
    "rekord",
    "reliable",
    "reliance",
    "remagg",
    "rembrandt",
    "remer",
    "remington",
    "remsho",
    "remstar",
    "remtor",
    "reporters",
    "resko",
    "rex",
    "rexpel",
    "rheinita",
    "rheinmetall",
    "rival",
    "roberts",
    "robotron",
    "rocher",
    "rochester",
    "roebuck",
    "rofa",
    "roland",
    "rooy",
    "rover",
    "roxy",
    "roy",
    "royal",
    "rundstatler",
    "sabaudia",
    "sabb",
    "saleem",
    "salter",
    "sampo",
    "sarafan",
    "saturn",
    "saxonia",
    "schade",
    "schapiro",
    "schreibi",
    "scripta",
    "sears",
    "secor",
    "selectric",
    "selekta",
    "senator",
    "sense",
    "senta",
    "serd",
    "shilling",
    "shimade",
    "shimer",
    "sholes",
    "shuang",
    "siegfried",
    "siemag",
    "silma",
    "silver",
    "simplex",
    "simtype",
    "singer",
    "smith",
    "soemtron",
    "sonja",
    "speedwriter",
    "sphinx",
    "starlet",
    "stearns",
    "steel",
    "stella",
    "steno",
    "sterling",
    "stoewer",
    "stolzenberg",
    "stott",
    "strangfeld",
    "sture",
    "stylotyp",
    "sun",
    "superba",
    "superia",
    "supermetall",
    "surety",
    "swintec",
    "swissa",
    "talbos",
    "talleres",
    "tatrapoint",
    "taurus",
    "taylorix",
    "tell",
    "tempotype",
    "tippco",
    "titania",
    "tops",
    "towa",
    "toyo",
    "tradition",
    "transatlantic",
    "traveller",
    "trebla",
    "triumph",
    "turia",
    "typatune",
    "typen",
    "typorium",
    "ugro",
    "ultima",
    "unda",
    "underwood",
    "unica",
    "unitype",
    "ursula",
    "utax",
    "varityper",
    "vasanta",
    "vendex",
    "venus",
    "victor",
    "victoria",
    "video",
    "viking",
    "vira",
    "virotyp",
    "visigraph",
    "vittoria",
    "volcan",
    "vornado",
    "voss",
    "vultur",
    "waltons",
    "wanamaker",
    "wanderer",
    "ward",
    "warner",
    "waterloo",
    "waverley",
    "wayne",
    "webster",
    "wedgefield",
    "welco",
    "wellington",
    "wellon",
    "weltblick",
    "westphalia",
    "wiedmer",
    "williams",
    "wilson",
    "winkel",
    "winsor",
    "wizard",
    "woodstock",
    "woodwards",
    "yatran",
    "yost",
    "zenit",
    "zentronik",
    "zeta",
    "zeya",
];

/// Picks a typewriter name that isn't already taken by an existing branch.
///
/// Each entry in `existing_branches` is expected to be a full branch name
/// like `"olivetti-a3f9b2c1"`. The prefix before the last `'-'` is treated
/// as the taken typewriter name. Branches without a `'-'` are ignored.
///
/// Returns `None` when every name in the pool is already taken.
pub fn pick_typewriter_name(
    existing_branches: &[&str],
    rng: &mut impl Rng,
) -> Option<&'static str> {
    let disallowed: HashSet<&str> = existing_branches
        .iter()
        .filter_map(|branch| branch.rsplit_once('-').map(|(prefix, _)| prefix))
        .collect();

    let available: Vec<&'static str> = TYPEWRITER_NAMES
        .iter()
        .copied()
        .filter(|name| !disallowed.contains(name))
        .collect();

    if available.is_empty() {
        return None;
    }

    let index = rng.random_range(0..available.len());
    Some(available[index])
}

/// Generates a branch name like `"olivetti-a3f9b2c1"` by picking a typewriter
/// name that isn't already taken and appending an 8-character alphanumeric hash.
///
/// Returns `None` when every typewriter name in the pool is already taken.
pub fn generate_branch_name(existing_branches: &[&str], rng: &mut impl Rng) -> Option<String> {
    let typewriter_name = pick_typewriter_name(existing_branches, rng)?;
    let hash: String = (0..8)
        .map(|_| {
            let idx: u8 = rng.random_range(0..36);
            if idx < 10 {
                (b'0' + idx) as char
            } else {
                (b'a' + idx - 10) as char
            }
        })
        .collect();
    Some(format!("{typewriter_name}-{hash}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;

    #[gpui::test(iterations = 10)]
    fn test_pick_typewriter_name_with_no_disallowed(mut rng: StdRng) {
        let name = pick_typewriter_name(&[], &mut rng);
        assert!(name.is_some());
        assert!(TYPEWRITER_NAMES.contains(&name.unwrap()));
    }

    #[gpui::test(iterations = 10)]
    fn test_pick_typewriter_name_excludes_taken_names(mut rng: StdRng) {
        let branch_names = &["olivetti-abc12345", "selectric-def67890"];
        let name = pick_typewriter_name(branch_names, &mut rng).unwrap();
        assert_ne!(name, "olivetti");
        assert_ne!(name, "selectric");
    }

    #[gpui::test]
    fn test_pick_typewriter_name_all_taken(mut rng: StdRng) {
        let branch_names: Vec<String> = TYPEWRITER_NAMES
            .iter()
            .map(|name| format!("{name}-00000000"))
            .collect();
        let branch_name_refs: Vec<&str> = branch_names.iter().map(|s| s.as_str()).collect();
        let name = pick_typewriter_name(&branch_name_refs, &mut rng);
        assert!(name.is_none());
    }

    #[gpui::test(iterations = 10)]
    fn test_pick_typewriter_name_ignores_branches_without_hyphen(mut rng: StdRng) {
        let branch_names = &["main", "develop", "feature"];
        let name = pick_typewriter_name(branch_names, &mut rng);
        assert!(name.is_some());
        assert!(TYPEWRITER_NAMES.contains(&name.unwrap()));
    }

    #[gpui::test(iterations = 10)]
    fn test_generate_branch_name_format(mut rng: StdRng) {
        let branch_name = generate_branch_name(&[], &mut rng).unwrap();
        let (prefix, suffix) = branch_name.rsplit_once('-').unwrap();
        assert!(TYPEWRITER_NAMES.contains(&prefix));
        assert_eq!(suffix.len(), 8);
        assert!(suffix.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[gpui::test]
    fn test_generate_branch_name_returns_none_when_exhausted(mut rng: StdRng) {
        let branch_names: Vec<String> = TYPEWRITER_NAMES
            .iter()
            .map(|name| format!("{name}-00000000"))
            .collect();
        let branch_name_refs: Vec<&str> = branch_names.iter().map(|s| s.as_str()).collect();
        let result = generate_branch_name(&branch_name_refs, &mut rng);
        assert!(result.is_none());
    }

    #[gpui::test(iterations = 100)]
    fn test_generate_branch_name_never_reuses_taken_prefix(mut rng: StdRng) {
        let existing = &["olivetti-123abc", "selectric-def456"];
        let branch_name = generate_branch_name(existing, &mut rng).unwrap();
        let (prefix, _) = branch_name.rsplit_once('-').unwrap();
        assert_ne!(prefix, "olivetti");
        assert_ne!(prefix, "selectric");
    }

    #[gpui::test(iterations = 100)]
    fn test_generate_branch_name_avoids_multiple_taken_prefixes(mut rng: StdRng) {
        let existing = &[
            "olivetti-aaa11111",
            "selectric-bbb22222",
            "corona-ccc33333",
            "remington-ddd44444",
            "underwood-eee55555",
        ];
        let taken_prefixes: HashSet<&str> = existing
            .iter()
            .filter_map(|b| b.rsplit_once('-').map(|(prefix, _)| prefix))
            .collect();
        let branch_name = generate_branch_name(existing, &mut rng).unwrap();
        let (prefix, _) = branch_name.rsplit_once('-').unwrap();
        assert!(
            !taken_prefixes.contains(prefix),
            "generated prefix {prefix:?} collides with an existing branch"
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_generate_branch_name_with_varied_hash_suffixes(mut rng: StdRng) {
        let existing = &[
            "olivetti-aaaaaaaa",
            "olivetti-bbbbbbbb",
            "olivetti-cccccccc",
        ];
        let branch_name = generate_branch_name(existing, &mut rng).unwrap();
        let (prefix, _) = branch_name.rsplit_once('-').unwrap();
        assert_ne!(
            prefix, "olivetti",
            "should avoid olivetti regardless of how many variants exist"
        );
    }

    #[test]
    fn test_typewriter_names_are_valid() {
        let mut seen = HashSet::default();
        for &name in TYPEWRITER_NAMES {
            assert!(
                seen.insert(name),
                "duplicate entry in TYPEWRITER_NAMES: {name:?}"
            );
        }

        for window in TYPEWRITER_NAMES.windows(2) {
            assert!(
                window[0] <= window[1],
                "TYPEWRITER_NAMES is not sorted: {0:?} should come after {1:?}",
                window[1],
                window[0],
            );
        }

        for &name in TYPEWRITER_NAMES {
            assert!(
                !name.contains('-'),
                "TYPEWRITER_NAMES entry contains a hyphen: {name:?}"
            );
        }

        for &name in TYPEWRITER_NAMES {
            assert!(
                name.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()),
                "TYPEWRITER_NAMES entry is not lowercase: {name:?}"
            );
        }
    }
}
