fn main() {
    tonic_build::compile_protos("proto/dfs.proto").unwrap();
}