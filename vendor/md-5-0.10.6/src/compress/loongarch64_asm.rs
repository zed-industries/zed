//! LoongArch64 assembly backend

macro_rules! c {
    ($($l:expr)*) => {
        concat!($($l ,)*)
    };
}

macro_rules! round0 {
    ($a:literal, $b:literal, $c:literal, $d:literal, $k:literal, $s:literal, $i:literal) => {
        c!(
            "xor    $t4," $c "," $d ";"
            "and    $t4, $t4," $b ";"
            "xor    $t4, $t4," $d ";"
            roundtail!($a, $b, $k, $s, $i)
        )
    }
}

macro_rules! round1 {
    ($a:literal, $b:literal, $c:literal, $d:literal, $k:literal, $s:literal, $i:literal) => {
        c!(
            "andn    $t4," $c "," $d ";"
            "and     $t5," $d "," $b ";"
            "or      $t4, $t4, $t5;"
            roundtail!($a, $b, $k, $s, $i)
        )
    }
}

macro_rules! round2 {
    ($a:literal, $b:literal, $c:literal, $d:literal, $k:literal, $s:literal, $i:literal) => {
        c!(
            "xor    $t4," $c "," $d ";"
            "xor    $t4, $t4," $b ";"
            roundtail!($a, $b, $k, $s, $i)
        )
    }
}

macro_rules! round3 {
    ($a:literal, $b:literal, $c:literal, $d:literal, $k:literal, $s:literal, $i:literal) => {
        c!(
            "orn    $t4," $b "," $d ";"
            "xor    $t4, $t4," $c ";"
            roundtail!($a, $b, $k, $s, $i)
        )
    }
}

macro_rules! roundtail {
    ($a:literal, $b:literal, $k:literal, $s:literal, $i:literal) => {
        c!(
            "ld.w       $t5, $a3," $i " * 4;"
            "ld.w       $t6, $a1," $k " * 4;"
            "add.w      " $a "," $a ", $t5;"
            "add.w      " $a "," $a ", $t6;"
            "add.w      " $a "," $a ", $t4;"
            "rotri.w    " $a "," $a ", 32 -" $s ";"
            "add.w      " $a "," $a "," $b ";"
        )
    }
}

pub fn compress(state: &mut [u32; 4], blocks: &[[u8; 64]]) {
    if blocks.is_empty() {
        return;
    }

    unsafe {
        core::arch::asm!(
            "42:",

            "move    $t0, $a4",
            "move    $t1, $a5",
            "move    $t2, $a6",
            "move    $t3, $a7",

            /* 64 rounds of hashing */
            round0!("$t0", "$t1", "$t2", "$t3",  0,  7,  0),
            round0!("$t3", "$t0", "$t1", "$t2",  1, 12,  1),
            round0!("$t2", "$t3", "$t0", "$t1",  2, 17,  2),
            round0!("$t1", "$t2", "$t3", "$t0",  3, 22,  3),
            round0!("$t0", "$t1", "$t2", "$t3",  4,  7,  4),
            round0!("$t3", "$t0", "$t1", "$t2",  5, 12,  5),
            round0!("$t2", "$t3", "$t0", "$t1",  6, 17,  6),
            round0!("$t1", "$t2", "$t3", "$t0",  7, 22,  7),
            round0!("$t0", "$t1", "$t2", "$t3",  8,  7,  8),
            round0!("$t3", "$t0", "$t1", "$t2",  9, 12,  9),
            round0!("$t2", "$t3", "$t0", "$t1", 10, 17, 10),
            round0!("$t1", "$t2", "$t3", "$t0", 11, 22, 11),
            round0!("$t0", "$t1", "$t2", "$t3", 12,  7, 12),
            round0!("$t3", "$t0", "$t1", "$t2", 13, 12, 13),
            round0!("$t2", "$t3", "$t0", "$t1", 14, 17, 14),
            round0!("$t1", "$t2", "$t3", "$t0", 15, 22, 15),

            round1!("$t0", "$t1", "$t2", "$t3",  1,  5, 16),
            round1!("$t3", "$t0", "$t1", "$t2",  6,  9, 17),
            round1!("$t2", "$t3", "$t0", "$t1", 11, 14, 18),
            round1!("$t1", "$t2", "$t3", "$t0",  0, 20, 19),
            round1!("$t0", "$t1", "$t2", "$t3",  5,  5, 20),
            round1!("$t3", "$t0", "$t1", "$t2", 10,  9, 21),
            round1!("$t2", "$t3", "$t0", "$t1", 15, 14, 22),
            round1!("$t1", "$t2", "$t3", "$t0",  4, 20, 23),
            round1!("$t0", "$t1", "$t2", "$t3",  9,  5, 24),
            round1!("$t3", "$t0", "$t1", "$t2", 14,  9, 25),
            round1!("$t2", "$t3", "$t0", "$t1",  3, 14, 26),
            round1!("$t1", "$t2", "$t3", "$t0",  8, 20, 27),
            round1!("$t0", "$t1", "$t2", "$t3", 13,  5, 28),
            round1!("$t3", "$t0", "$t1", "$t2",  2,  9, 29),
            round1!("$t2", "$t3", "$t0", "$t1",  7, 14, 30),

            round1!("$t1", "$t2", "$t3", "$t0", 12, 20, 31),
            round2!("$t0", "$t1", "$t2", "$t3",  5,  4, 32),
            round2!("$t3", "$t0", "$t1", "$t2",  8, 11, 33),
            round2!("$t2", "$t3", "$t0", "$t1", 11, 16, 34),
            round2!("$t1", "$t2", "$t3", "$t0", 14, 23, 35),
            round2!("$t0", "$t1", "$t2", "$t3",  1,  4, 36),
            round2!("$t3", "$t0", "$t1", "$t2",  4, 11, 37),
            round2!("$t2", "$t3", "$t0", "$t1",  7, 16, 38),
            round2!("$t1", "$t2", "$t3", "$t0", 10, 23, 39),
            round2!("$t0", "$t1", "$t2", "$t3", 13,  4, 40),
            round2!("$t3", "$t0", "$t1", "$t2",  0, 11, 41),
            round2!("$t2", "$t3", "$t0", "$t1",  3, 16, 42),
            round2!("$t1", "$t2", "$t3", "$t0",  6, 23, 43),
            round2!("$t0", "$t1", "$t2", "$t3",  9,  4, 44),
            round2!("$t3", "$t0", "$t1", "$t2", 12, 11, 45),
            round2!("$t2", "$t3", "$t0", "$t1", 15, 16, 46),
            round2!("$t1", "$t2", "$t3", "$t0",  2, 23, 47),

            round3!("$t0", "$t1", "$t2", "$t3",  0,  6, 48),
            round3!("$t3", "$t0", "$t1", "$t2",  7, 10, 49),
            round3!("$t2", "$t3", "$t0", "$t1", 14, 15, 50),
            round3!("$t1", "$t2", "$t3", "$t0",  5, 21, 51),
            round3!("$t0", "$t1", "$t2", "$t3", 12,  6, 52),
            round3!("$t3", "$t0", "$t1", "$t2",  3, 10, 53),
            round3!("$t2", "$t3", "$t0", "$t1", 10, 15, 54),
            round3!("$t1", "$t2", "$t3", "$t0",  1, 21, 55),
            round3!("$t0", "$t1", "$t2", "$t3",  8,  6, 56),
            round3!("$t3", "$t0", "$t1", "$t2", 15, 10, 57),
            round3!("$t2", "$t3", "$t0", "$t1",  6, 15, 58),
            round3!("$t1", "$t2", "$t3", "$t0", 13, 21, 59),
            round3!("$t0", "$t1", "$t2", "$t3",  4,  6, 60),
            round3!("$t3", "$t0", "$t1", "$t2", 11, 10, 61),
            round3!("$t2", "$t3", "$t0", "$t1",  2, 15, 62),
            round3!("$t1", "$t2", "$t3", "$t0",  9, 21, 63),

            "add.w   $a4, $a4, $t0",
            "add.w   $a5, $a5, $t1",
            "add.w   $a6, $a6, $t2",
            "add.w   $a7, $a7, $t3",

            // Looping over blocks
            "addi.d  $a1, $a1, 64",
            "addi.d  $a2, $a2, -1",
            "bnez    $a2, 42b",

            inout("$a1") blocks.as_ptr() => _,
            inout("$a2") blocks.len() => _,
            in("$a3") crate::consts::RC.as_ptr(),
            inout("$a4") state[0],
            inout("$a5") state[1],
            inout("$a6") state[2],
            inout("$a7") state[3],

            // Clobbers
            out("$t0") _,
            out("$t1") _,
            out("$t2") _,
            out("$t3") _,
            out("$t4") _,
            out("$t5") _,
            out("$t6") _,

            options(preserves_flags, readonly, pure, nostack),
        );
    }
}
