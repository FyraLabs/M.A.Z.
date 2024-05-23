fn main() {
    ::capnpc::CompilerCommand::new()
        .file("locker.capnp")
        .run()
        .expect("compiling schema");
}
