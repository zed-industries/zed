#![cfg(feature = "rand")]

extern crate num_bigint_dig as num_bigint;
extern crate num_traits;
extern crate rand;
extern crate rand_chacha;
extern crate rand_isaac;
extern crate rand_xorshift;

mod biguint {
    use crate::num_bigint::{BigUint, RandBigInt, RandomBits};
    use num_traits::Zero;
    use rand::distributions::Uniform;
    use rand::{Rng, SeedableRng};

    #[cfg(feature = "std")]
    fn thread_rng() -> impl Rng {
        rand::thread_rng()
    }
    #[cfg(not(feature = "std"))]
    fn thread_rng() -> impl Rng {
        // Chosen by fair dice roll
        rand::rngs::StdRng::seed_from_u64(4)
    }

    #[test]
    fn test_rand() {
        let mut rng = thread_rng();
        let n: BigUint = rng.gen_biguint(137);
        assert!(n.bits() <= 137);
        assert!(rng.gen_biguint(0).is_zero());
    }

    #[test]
    fn test_rand_bits() {
        let mut rng = thread_rng();
        let n: BigUint = rng.sample(&RandomBits::new(137));
        assert!(n.bits() <= 137);
        let z: BigUint = rng.sample(&RandomBits::new(0));
        assert!(z.is_zero());
    }

    #[test]
    fn test_rand_range() {
        let mut rng = thread_rng();

        for _ in 0..10 {
            assert_eq!(
                rng.gen_biguint_range(&BigUint::from(236u32), &BigUint::from(237u32)),
                BigUint::from(236u32)
            );
        }

        let l = BigUint::from(403469000u32 + 2352);
        let u = BigUint::from(403469000u32 + 3513);
        for _ in 0..1000 {
            let n: BigUint = rng.gen_biguint_below(&u);
            assert!(n < u);

            let n: BigUint = rng.gen_biguint_range(&l, &u);
            assert!(n >= l);
            assert!(n < u);
        }
    }

    #[test]
    #[should_panic]
    fn test_zero_rand_range() {
        thread_rng().gen_biguint_range(&BigUint::from(54u32), &BigUint::from(54u32));
    }

    #[test]
    #[should_panic]
    fn test_negative_rand_range() {
        let mut rng = thread_rng();
        let l = BigUint::from(2352u32);
        let u = BigUint::from(3513u32);
        // Switching u and l should fail:
        let _n: BigUint = rng.gen_biguint_range(&u, &l);
    }

    #[test]
    fn test_rand_uniform() {
        let mut rng = thread_rng();

        let tiny = Uniform::new(BigUint::from(236u32), BigUint::from(237u32));
        for _ in 0..10 {
            assert_eq!(rng.sample(&tiny), BigUint::from(236u32));
        }

        let l = BigUint::from(403469000u32 + 2352);
        let u = BigUint::from(403469000u32 + 3513);
        let below = Uniform::new(BigUint::zero(), u.clone());
        let range = Uniform::new(l.clone(), u.clone());
        for _ in 0..1000 {
            let n: BigUint = rng.sample(&below);
            assert!(n < u);

            let n: BigUint = rng.sample(&range);
            assert!(n >= l);
            assert!(n < u);
        }
    }

    fn seeded_value_stability<R: SeedableRng + RandBigInt>(expected: &[&str]) {
        let mut seed = <R::Seed>::default();
        for (i, x) in seed.as_mut().iter_mut().enumerate() {
            *x = (i as u8).wrapping_mul(191);
        }
        let mut rng = R::from_seed(seed);
        for (i, &s) in expected.iter().enumerate() {
            let n: BigUint = s.parse().unwrap();
            let r = rng.gen_biguint((1 << i) + i);
            assert_eq!(n, r, "expected {}, got {}", n, r);
        }
    }

    #[cfg(not(feature = "u64_digit"))]
    const EXPECTED_CHACHA: &[&str] = &[
        "0",
        "0",
        "52",
        "84",
        "23780",
        "86502865016",
        "187057847319509867386",
        "34045731223080904464438757488196244981910",
        "23813754422987836414755953516143692594193066497413249270287126597896871975915808",
        "57401636903146945411652549098818446911814352529449356393690984105383482703074355\
         67088360974672291353736011718191813678720755501317478656550386324355699624671",
    ];

    #[cfg(feature = "u64_digit")]
    const EXPECTED_CHACHA: &[&str] = &[
        "0",
        "0",
        "8",
        "1861",
        "172076",
        "5194801951",
        "259202797457072892019",
        "2806086822955830608275100562233284760859",
        "28771276448190704455825316568337256462972770861366848469339788407170414346460023",
        "501572804396703231264118826164515310701005506447150057229826006447721882571235378\
         4765127362270091441643338804096337494157874113908470083557122824480944132407",
    ];

    #[test]
    fn test_chacha_value_stability() {
        use rand_chacha::ChaChaRng;
        seeded_value_stability::<ChaChaRng>(EXPECTED_CHACHA);
    }

    #[cfg(not(feature = "u64_digit"))]
    const EXPECTED_ISAAC: &[&str] = &[
        "1",
        "4",
        "3",
        "649",
        "89116",
        "7730042024",
        "20773149082453254949",
        "35999009049239918667571895439206839620281",
        "10191757312714088681302309313551624007714035309632506837271600807524767413673006",
        "37805949268912387809989378008822038725134260145886913321084097194957861133272558\
         43458183365174899239251448892645546322463253898288141861183340823194379722556",
    ];

    #[cfg(feature = "u64_digit")]
    const EXPECTED_ISAAC: &[&str] = &[
        "1",
        "2",
        "51",
        "1198",
        "29707",
        "35688018574",
        "365090200586541225112",
        "14051533166427520604648370582738617763816",
        "26319846996091585801307964353534339679417889504909644767909019559631059772127122",
        "14567336733062747693583250833667292276083519237160662196899060257293814346680656\
         30951609693408423310563908301065751714778956255122249041917698392245727713420",
    ];
    #[test]
    fn test_isaac_value_stability() {
        use rand_isaac::IsaacRng;
        seeded_value_stability::<IsaacRng>(EXPECTED_ISAAC);
    }

    #[cfg(not(feature = "u64_digit"))]
    const EXPECTED_XOR: &[&str] = &[
        "1",
        "0",
        "37",
        "395",
        "181116",
        "122718231117",
        "1068467172329355695001",
        "28246925743544411614293300167064395633287",
        "12750053187017853048648861493745244146555950255549630854523304068318587267293038",
        "53041498719137109355568081064978196049094604705283682101683207799515709404788873\
         53417136457745727045473194367732849819278740266658219147356315674940229288531",
    ];
    #[cfg(feature = "u64_digit")]
    const EXPECTED_XOR: &[&str] = &[
        "0",
        "1",
        "36",
        "970",
        "940965",
        "61158366130",
        "590484965100191554896",
        "34050066418951688801044382442803594076612",
        "29147581645599998811521651062569705291155276949983132826461704326818089074318948",
        "4990842894093964353439376569956547459232523176881032246435842690389845516810345611554402412893818283310117202233021355634125020654279500443420515862554775828",
    ];

    #[test]
    fn test_xorshift_value_stability() {
        use rand_xorshift::XorShiftRng;
        seeded_value_stability::<XorShiftRng>(EXPECTED_XOR);
    }
}

mod bigint {
    use crate::num_bigint::{BigInt, RandBigInt, RandomBits};
    use num_traits::Zero;
    use rand::distributions::Uniform;
    use rand::{Rng, SeedableRng};

    #[cfg(feature = "std")]
    fn thread_rng() -> impl Rng {
        rand::thread_rng()
    }
    #[cfg(not(feature = "std"))]
    fn thread_rng() -> impl Rng {
        // Chosen by fair dice roll
        rand::rngs::StdRng::seed_from_u64(4)
    }

    #[test]
    fn test_rand() {
        let mut rng = thread_rng();
        let n: BigInt = rng.gen_bigint(137);
        assert!(n.bits() <= 137);
        assert!(rng.gen_bigint(0).is_zero());
    }

    #[test]
    fn test_rand_bits() {
        let mut rng = thread_rng();
        let n: BigInt = rng.sample(&RandomBits::new(137));
        assert!(n.bits() <= 137);
        let z: BigInt = rng.sample(&RandomBits::new(0));
        assert!(z.is_zero());
    }

    #[test]
    fn test_rand_range() {
        let mut rng = thread_rng();

        for _ in 0..10 {
            assert_eq!(
                rng.gen_bigint_range(&BigInt::from(236), &BigInt::from(237)),
                BigInt::from(236)
            );
        }

        fn check(l: BigInt, u: BigInt) {
            let mut rng = thread_rng();
            for _ in 0..1000 {
                let n: BigInt = rng.gen_bigint_range(&l, &u);
                assert!(n >= l);
                assert!(n < u);
            }
        }
        let l: BigInt = BigInt::from(403469000 + 2352);
        let u: BigInt = BigInt::from(403469000 + 3513);
        check(l.clone(), u.clone());
        check(-l.clone(), u.clone());
        check(-u.clone(), -l.clone());
    }

    #[test]
    #[should_panic]
    fn test_zero_rand_range() {
        thread_rng().gen_bigint_range(&BigInt::from(54), &BigInt::from(54));
    }

    #[test]
    #[should_panic]
    fn test_negative_rand_range() {
        let mut rng = thread_rng();
        let l = BigInt::from(2352);
        let u = BigInt::from(3513);
        // Switching u and l should fail:
        let _n: BigInt = rng.gen_bigint_range(&u, &l);
    }

    #[test]
    fn test_rand_uniform() {
        let mut rng = thread_rng();

        let tiny = Uniform::new(BigInt::from(236u32), BigInt::from(237u32));
        for _ in 0..10 {
            assert_eq!(rng.sample(&tiny), BigInt::from(236u32));
        }

        fn check(l: BigInt, u: BigInt) {
            let mut rng = thread_rng();
            let range = Uniform::new(l.clone(), u.clone());
            for _ in 0..1000 {
                let n: BigInt = rng.sample(&range);
                assert!(n >= l);
                assert!(n < u);
            }
        }
        let l: BigInt = BigInt::from(403469000 + 2352);
        let u: BigInt = BigInt::from(403469000 + 3513);
        check(l.clone(), u.clone());
        check(-l.clone(), u.clone());
        check(-u.clone(), -l.clone());
    }

    fn seeded_value_stability<R: SeedableRng + RandBigInt>(expected: &[&str]) {
        let mut seed = <R::Seed>::default();
        for (i, x) in seed.as_mut().iter_mut().enumerate() {
            *x = (i as u8).wrapping_mul(191);
        }
        let mut rng = R::from_seed(seed);
        for (i, &s) in expected.iter().enumerate() {
            let n: BigInt = s.parse().unwrap();
            let r = rng.gen_bigint((1 << i) + i);
            assert_eq!(n, r, "expected {}, got {}", n, r);
        }
    }
    #[cfg(not(feature = "u64_digit"))]
    const EXPECTED_CHACHA: &[&str] = &[
        "0",
        "-6",
        "-1",
        "1321",
        "-147247",
        "8486373526",
        "-272736656290199720696",
        "2731152629387534140535423510744221288522",
        "-28820024790651190394679732038637785320661450462089347915910979466834461433196572",
        "501454570554170484799723603981439288209930393334472085317977614690773821680884844\
         8530978478667288338327570972869032358120588620346111979053742269317702532328",
    ];

    #[cfg(feature = "u64_digit")]
    const EXPECTED_CHACHA: &[&str] = &[
        "0",
        "-7",
        "-62",
        "105",
        "13025",
        "-33857814162",
        "768483926407291599143",
        "-42356168828789885585553598574661841382586",
        "28813250216034838684899917677182169473483558650956121225920149068989083656174824",
        "27056553770481404639717657695702187062015359344716548489861498121037858109133467\
         99640556108506718020020878739044048067894089601665199172215093468287730555599",
    ];

    #[test]
    fn test_chacha_value_stability() {
        use rand_chacha::ChaChaRng;
        seeded_value_stability::<ChaChaRng>(EXPECTED_CHACHA);
    }

    #[cfg(not(feature = "u64_digit"))]
    const EXPECTED_ISAAC: &[&str] = &[
        "1",
        "0",
        "5",
        "113",
        "-132240",
        "-36348760761",
        "-365690596708430705434",
        "-14090753008246284277803606722552430292432",
        "-26313941628626248579319341019368550803676255307056857978955881718727601479436059",
        "-14563174552421101848999036239003801073335703811160945137332228646111920972691151\
         88341090358094331641182310792892459091016794928947242043358702692294695845817",
    ];
    #[cfg(feature = "u64_digit")]
    const EXPECTED_ISAAC: &[&str] = &[
        "-1",
        "-4",
        "-29",
        "1621",
        "23872",
        "-40371956434",
        "-350815272425024298187",
        "-38554888817044546636456097200348998322457",
        "7474426176220721712055446211154990065592106428397966654956172383998793852911545",
        "6168955541726830487961394166772329653532583907235825721475483003506842180688827\
         391385624898257369023912466314791483731902392667906094226608113824795883754631",
    ];

    #[test]
    fn test_isaac_value_stability() {
        use rand_isaac::IsaacRng;
        seeded_value_stability::<IsaacRng>(EXPECTED_ISAAC);
    }

    #[cfg(not(feature = "u64_digit"))]
    const EXPECTED_XOR: &[&str] = &[
        "-1",
        "-4",
        "11",
        "-1802",
        "966495",
        "-62592045703",
        "-602281783447192077116",
        "-34335811410223060575607987996861632509125",
        "29156580925282215857325937227200350542000244609280383263289720243118706105351199",
        "49920038676141573457451407325930326489996232208489690499754573826911037849083623\
         24546142615325187412887314466195222441945661833644117700809693098722026764846",
    ];

    #[cfg(feature = "u64_digit")]
    const EXPECTED_XOR: &[&str] = &[
        "-1",
        "-3",
        "4",
        "-228",
        "377276",
        "32032893086",
        "885120221048601050706",
        "33404877924318663206979407569537287223622",
        "-15253093455306269007559295940333933266263385975865952571271093251749752787075084",
        "4502641950394305250103130679458759592222756470562408577296380915684757985604969904\
         381774527626485128207406911227296090734227576935034372181808818486328078978",
    ];

    #[test]
    fn test_xorshift_value_stability() {
        use rand_xorshift::XorShiftRng;
        seeded_value_stability::<XorShiftRng>(EXPECTED_XOR);
    }
}

#[cfg(feature = "prime")]
mod prime {
    use num_bigint::prime::probably_prime;
    use num_bigint::RandPrime;
    use rand::prelude::*;

    #[test]
    fn test_prime_small() {
        let mut rng = StdRng::from_seed([0u8; 32]);
        for n in 2..10 {
            let p = rng.gen_prime(n);

            assert_eq!(p.bits(), n);
            assert!(probably_prime(&p, 32));
        }
    }

    #[test]
    fn test_gen_prime_1024() {
        let mut rng = StdRng::from_seed([0u8; 32]);
        let p = rng.gen_prime(1024);
        assert_eq!(p.bits(), 1024);
    }
}
