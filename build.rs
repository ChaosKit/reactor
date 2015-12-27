extern crate capnpc;

fn main() {
    ::capnpc::compile("chaoskit", &["src/chaoskit.capnp"]).unwrap()
}
