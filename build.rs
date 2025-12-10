fn main() {
    cc::Build::new()
        .file("c_eval/pokerlib.cpp")
        .file("c_eval/mtrand.cpp")
        .file("wrapper.c")
        .include("c_eval")
        .cpp(true)
        .compile("pokerlib");
}

