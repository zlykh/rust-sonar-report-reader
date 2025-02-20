extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.default_package_filename("report_parts");
    config.out_dir("src/proto");
    config.compile_protos(&["src/proto/report_parts.proto"],&["src/proto"]).unwrap();
}