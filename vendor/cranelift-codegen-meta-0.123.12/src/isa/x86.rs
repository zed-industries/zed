use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::{PredicateNode, SettingGroupBuilder};

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("x86");

    // CPUID.01H:ECX
    let has_sse3 = settings.add_bool(
        "has_sse3",
        "Has support for SSE3.",
        "SSE3: CPUID.01H:ECX.SSE3[bit 0]",
        false,
    );
    let has_ssse3 = settings.add_bool(
        "has_ssse3",
        "Has support for SSSE3.",
        "SSSE3: CPUID.01H:ECX.SSSE3[bit 9]",
        false,
    );
    let has_cmpxchg16b = settings.add_bool(
        "has_cmpxchg16b",
        "Has support for CMPXCHG16b.",
        "CMPXCHG16b: CPUID.01H:ECX.CMPXCHG16B[bit 13]",
        false,
    );
    let has_sse41 = settings.add_bool(
        "has_sse41",
        "Has support for SSE4.1.",
        "SSE4.1: CPUID.01H:ECX.SSE4_1[bit 19]",
        false,
    );
    let has_sse42 = settings.add_bool(
        "has_sse42",
        "Has support for SSE4.2.",
        "SSE4.2: CPUID.01H:ECX.SSE4_2[bit 20]",
        false,
    );
    let has_avx = settings.add_bool(
        "has_avx",
        "Has support for AVX.",
        "AVX: CPUID.01H:ECX.AVX[bit 28]",
        false,
    );
    let has_avx2 = settings.add_bool(
        "has_avx2",
        "Has support for AVX2.",
        "AVX2: CPUID.07H:EBX.AVX2[bit 5]",
        false,
    );
    let has_fma = settings.add_bool(
        "has_fma",
        "Has support for FMA.",
        "FMA: CPUID.01H:ECX.FMA[bit 12]",
        false,
    );
    let has_avx512bitalg = settings.add_bool(
        "has_avx512bitalg",
        "Has support for AVX512BITALG.",
        "AVX512BITALG: CPUID.07H:ECX.AVX512BITALG[bit 12]",
        false,
    );
    let has_avx512dq = settings.add_bool(
        "has_avx512dq",
        "Has support for AVX512DQ.",
        "AVX512DQ: CPUID.07H:EBX.AVX512DQ[bit 17]",
        false,
    );
    let has_avx512vl = settings.add_bool(
        "has_avx512vl",
        "Has support for AVX512VL.",
        "AVX512VL: CPUID.07H:EBX.AVX512VL[bit 31]",
        false,
    );
    let has_avx512vbmi = settings.add_bool(
        "has_avx512vbmi",
        "Has support for AVX512VMBI.",
        "AVX512VBMI: CPUID.07H:ECX.AVX512VBMI[bit 1]",
        false,
    );
    let has_avx512f = settings.add_bool(
        "has_avx512f",
        "Has support for AVX512F.",
        "AVX512F: CPUID.07H:EBX.AVX512F[bit 16]",
        false,
    );
    let has_popcnt = settings.add_bool(
        "has_popcnt",
        "Has support for POPCNT.",
        "POPCNT: CPUID.01H:ECX.POPCNT[bit 23]",
        false,
    );

    // CPUID.(EAX=07H, ECX=0H):EBX
    let has_bmi1 = settings.add_bool(
        "has_bmi1",
        "Has support for BMI1.",
        "BMI1: CPUID.(EAX=07H, ECX=0H):EBX.BMI1[bit 3]",
        false,
    );
    let has_bmi2 = settings.add_bool(
        "has_bmi2",
        "Has support for BMI2.",
        "BMI2: CPUID.(EAX=07H, ECX=0H):EBX.BMI2[bit 8]",
        false,
    );

    // CPUID.EAX=80000001H:ECX
    let has_lzcnt = settings.add_bool(
        "has_lzcnt",
        "Has support for LZCNT.",
        "LZCNT: CPUID.EAX=80000001H:ECX.LZCNT[bit 5]",
        false,
    );

    settings.add_predicate("use_cmpxchg16b", predicate!(has_cmpxchg16b));
    settings.add_predicate("use_sse3", predicate!(has_sse3));
    settings.add_predicate("use_ssse3", predicate!(has_ssse3));
    settings.add_predicate("use_sse41", predicate!(has_sse41));
    settings.add_predicate("use_sse42", predicate!(has_sse41 && has_sse42));
    settings.add_predicate("use_fma", predicate!(has_avx && has_fma));

    settings.add_predicate("use_avx", predicate!(has_avx));
    settings.add_predicate("use_avx2", predicate!(has_avx && has_avx2));
    settings.add_predicate("use_avx512bitalg", predicate!(has_avx512bitalg));
    settings.add_predicate("use_avx512dq", predicate!(has_avx512dq));
    settings.add_predicate("use_avx512vl", predicate!(has_avx512vl));
    settings.add_predicate("use_avx512vbmi", predicate!(has_avx512vbmi));
    settings.add_predicate("use_avx512f", predicate!(has_avx512f));

    settings.add_predicate("use_popcnt", predicate!(has_popcnt && has_sse42));
    settings.add_predicate("use_bmi1", predicate!(has_bmi1));
    settings.add_predicate("use_bmi2", predicate!(has_bmi2));
    settings.add_predicate("use_lzcnt", predicate!(has_lzcnt));

    let sse3 = settings.add_preset("sse3", "SSE3 and earlier.", preset!(has_sse3));
    let ssse3 = settings.add_preset("ssse3", "SSSE3 and earlier.", preset!(sse3 && has_ssse3));
    let sse41 = settings.add_preset("sse41", "SSE4.1 and earlier.", preset!(ssse3 && has_sse41));
    let sse42 = settings.add_preset("sse42", "SSE4.2 and earlier.", preset!(sse41 && has_sse42));

    // Presets corresponding to x86 CPUs.
    // Features and architecture names are from LLVM's x86 presets:
    // https://github.com/llvm/llvm-project/blob/d4493dd1ed58ac3f1eab0c4ca6e363e2b15bfd1c/llvm/lib/Target/X86/X86.td#L1300-L1643
    settings.add_preset(
        "baseline",
        "A baseline preset with no extensions enabled.",
        preset!(),
    );

    // Intel CPUs

    // Netburst
    settings.add_preset(
        "nocona",
        "Nocona microarchitecture.",
        preset!(sse3 && has_cmpxchg16b),
    );

    // Intel Core 2 Solo/Duo
    settings.add_preset(
        "core2",
        "Core 2 microarchitecture.",
        preset!(sse3 && has_cmpxchg16b),
    );
    settings.add_preset(
        "penryn",
        "Penryn microarchitecture.",
        preset!(sse41 && has_cmpxchg16b),
    );

    // Intel Atom CPUs
    let atom = settings.add_preset(
        "atom",
        "Atom microarchitecture.",
        preset!(ssse3 && has_cmpxchg16b),
    );
    settings.add_preset("bonnell", "Bonnell microarchitecture.", preset!(atom));
    let silvermont = settings.add_preset(
        "silvermont",
        "Silvermont microarchitecture.",
        preset!(atom && sse42 && has_popcnt),
    );
    settings.add_preset("slm", "Silvermont microarchitecture.", preset!(silvermont));
    let goldmont = settings.add_preset(
        "goldmont",
        "Goldmont microarchitecture.",
        preset!(silvermont),
    );
    settings.add_preset(
        "goldmont-plus",
        "Goldmont Plus microarchitecture.",
        preset!(goldmont),
    );
    let tremont = settings.add_preset("tremont", "Tremont microarchitecture.", preset!(goldmont));

    let alderlake = settings.add_preset(
        "alderlake",
        "Alderlake microarchitecture.",
        preset!(tremont && has_bmi1 && has_bmi2 && has_lzcnt && has_fma),
    );
    let sierra_forest = settings.add_preset(
        "sierraforest",
        "Sierra Forest microarchitecture.",
        preset!(alderlake),
    );
    settings.add_preset(
        "grandridge",
        "Grandridge microarchitecture.",
        preset!(sierra_forest),
    );
    let nehalem = settings.add_preset(
        "nehalem",
        "Nehalem microarchitecture.",
        preset!(sse42 && has_popcnt && has_cmpxchg16b),
    );
    settings.add_preset("corei7", "Core i7 microarchitecture.", preset!(nehalem));
    let westmere = settings.add_preset("westmere", "Westmere microarchitecture.", preset!(nehalem));
    let sandy_bridge = settings.add_preset(
        "sandybridge",
        "Sandy Bridge microarchitecture.",
        preset!(westmere && has_avx),
    );
    settings.add_preset(
        "corei7-avx",
        "Core i7 AVX microarchitecture.",
        preset!(sandy_bridge),
    );
    let ivy_bridge = settings.add_preset(
        "ivybridge",
        "Ivy Bridge microarchitecture.",
        preset!(sandy_bridge),
    );
    settings.add_preset(
        "core-avx-i",
        "Intel Core CPU with 64-bit extensions.",
        preset!(ivy_bridge),
    );
    let haswell = settings.add_preset(
        "haswell",
        "Haswell microarchitecture.",
        preset!(ivy_bridge && has_avx2 && has_bmi1 && has_bmi2 && has_fma && has_lzcnt),
    );
    settings.add_preset(
        "core-avx2",
        "Intel Core CPU with AVX2 extensions.",
        preset!(haswell),
    );
    let broadwell = settings.add_preset(
        "broadwell",
        "Broadwell microarchitecture.",
        preset!(haswell),
    );
    let skylake = settings.add_preset("skylake", "Skylake microarchitecture.", preset!(broadwell));
    let knights_landing = settings.add_preset(
        "knl",
        "Knights Landing microarchitecture.",
        preset!(
            has_popcnt
                && has_avx512f
                && has_fma
                && has_bmi1
                && has_bmi2
                && has_lzcnt
                && has_cmpxchg16b
        ),
    );
    settings.add_preset(
        "knm",
        "Knights Mill microarchitecture.",
        preset!(knights_landing),
    );
    let skylake_avx512 = settings.add_preset(
        "skylake-avx512",
        "Skylake AVX512 microarchitecture.",
        preset!(broadwell && has_avx512f && has_avx512dq && has_avx512vl),
    );
    settings.add_preset(
        "skx",
        "Skylake AVX512 microarchitecture.",
        preset!(skylake_avx512),
    );
    let cascadelake = settings.add_preset(
        "cascadelake",
        "Cascade Lake microarchitecture.",
        preset!(skylake_avx512),
    );
    settings.add_preset(
        "cooperlake",
        "Cooper Lake microarchitecture.",
        preset!(cascadelake),
    );
    let cannonlake = settings.add_preset(
        "cannonlake",
        "Canon Lake microarchitecture.",
        preset!(skylake && has_avx512f && has_avx512dq && has_avx512vl && has_avx512vbmi),
    );
    let icelake_client = settings.add_preset(
        "icelake-client",
        "Ice Lake microarchitecture.",
        preset!(cannonlake && has_avx512bitalg),
    );
    // LLVM doesn't use the name "icelake" but Cranelift did in the past; alias it
    settings.add_preset(
        "icelake",
        "Ice Lake microarchitecture",
        preset!(icelake_client),
    );
    let icelake_server = settings.add_preset(
        "icelake-server",
        "Ice Lake (server) microarchitecture.",
        preset!(icelake_client),
    );
    settings.add_preset(
        "tigerlake",
        "Tiger Lake microarchitecture.",
        preset!(icelake_client),
    );
    let sapphire_rapids = settings.add_preset(
        "sapphirerapids",
        "Sapphire Rapids microarchitecture.",
        preset!(icelake_server),
    );
    settings.add_preset(
        "raptorlake",
        "Raptor Lake microarchitecture.",
        preset!(alderlake),
    );
    settings.add_preset(
        "meteorlake",
        "Meteor Lake microarchitecture.",
        preset!(alderlake),
    );
    settings.add_preset(
        "graniterapids",
        "Granite Rapids microarchitecture.",
        preset!(sapphire_rapids),
    );

    // AMD CPUs

    settings.add_preset("opteron", "Opteron microarchitecture.", preset!());
    settings.add_preset("k8", "K8 Hammer microarchitecture.", preset!());
    settings.add_preset("athlon64", "Athlon64 microarchitecture.", preset!());
    settings.add_preset("athlon-fx", "Athlon FX microarchitecture.", preset!());
    settings.add_preset(
        "opteron-sse3",
        "Opteron microarchitecture with support for SSE3 instructions.",
        preset!(sse3 && has_cmpxchg16b),
    );
    settings.add_preset(
        "k8-sse3",
        "K8 Hammer microarchitecture with support for SSE3 instructions.",
        preset!(sse3 && has_cmpxchg16b),
    );
    settings.add_preset(
        "athlon64-sse3",
        "Athlon 64 microarchitecture with support for SSE3 instructions.",
        preset!(sse3 && has_cmpxchg16b),
    );
    let barcelona = settings.add_preset(
        "barcelona",
        "Barcelona microarchitecture.",
        preset!(has_popcnt && has_lzcnt && has_cmpxchg16b),
    );
    settings.add_preset(
        "amdfam10",
        "AMD Family 10h microarchitecture",
        preset!(barcelona),
    );

    let btver1 = settings.add_preset(
        "btver1",
        "Bobcat microarchitecture.",
        preset!(ssse3 && has_lzcnt && has_popcnt && has_cmpxchg16b),
    );
    settings.add_preset(
        "btver2",
        "Jaguar microarchitecture.",
        preset!(btver1 && has_avx && has_bmi1),
    );

    let bdver1 = settings.add_preset(
        "bdver1",
        "Bulldozer microarchitecture",
        preset!(has_lzcnt && has_popcnt && ssse3 && has_cmpxchg16b),
    );
    let bdver2 = settings.add_preset(
        "bdver2",
        "Piledriver microarchitecture.",
        preset!(bdver1 && has_bmi1),
    );
    let bdver3 = settings.add_preset("bdver3", "Steamroller microarchitecture.", preset!(bdver2));
    settings.add_preset(
        "bdver4",
        "Excavator microarchitecture.",
        preset!(bdver3 && has_avx2 && has_bmi2),
    );

    let znver1 = settings.add_preset(
        "znver1",
        "Zen (first generation) microarchitecture.",
        preset!(
            sse42 && has_popcnt && has_bmi1 && has_bmi2 && has_lzcnt && has_fma && has_cmpxchg16b
        ),
    );
    let znver2 = settings.add_preset(
        "znver2",
        "Zen (second generation) microarchitecture.",
        preset!(znver1),
    );
    let znver3 = settings.add_preset(
        "znver3",
        "Zen (third generation) microarchitecture.",
        preset!(znver2),
    );
    settings.add_preset(
        "znver4",
        "Zen (fourth generation) microarchitecture.",
        preset!(
            znver3
                && has_avx512bitalg
                && has_avx512dq
                && has_avx512f
                && has_avx512vbmi
                && has_avx512vl
        ),
    );

    // Generic

    settings.add_preset("x86-64", "Generic x86-64 microarchitecture.", preset!());
    let x86_64_v2 = settings.add_preset(
        "x86-64-v2",
        "Generic x86-64 (V2) microarchitecture.",
        preset!(sse42 && has_popcnt && has_cmpxchg16b),
    );
    let x86_64_v3 = settings.add_preset(
        "x84_64_v3",
        "Generic x86_64 (V3) microarchitecture.",
        preset!(x86_64_v2 && has_bmi1 && has_bmi2 && has_fma && has_lzcnt && has_avx2),
    );
    settings.add_preset(
        "x86_64_v4",
        "Generic x86_64 (V4) microarchitecture.",
        preset!(x86_64_v3 && has_avx512dq && has_avx512vl),
    );

    TargetIsa::new("x86", settings.build())
}
