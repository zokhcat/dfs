fn main() {
    tonic_build::compile_protos("./src/proto/dfs.proto").unwrap();
}