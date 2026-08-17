fn main() {
    cc::Build::new().file("src/errno.c").compile("liberrno.a");
}
