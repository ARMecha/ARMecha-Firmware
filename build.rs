
#[path = "keyboards/handwired/BMMP/build_kb.rs"] mod build_kb;

fn main() {
    build_kb::gen_build();
}