fn main() {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(
            &[
                // "proto/cart.proto",
                // "proto/homepage.proto",
                // "proto/product.proto",
                "proto/user.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
