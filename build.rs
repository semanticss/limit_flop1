
fn main() {
    cc::Build::new()
        .files(["c_eval/pokerlib.cpp", "c_eval/mtrand.cpp", "wrapper.cpp"])
        .include("c_eval")
        .cpp(true)
        .compile("pokerlib");
}