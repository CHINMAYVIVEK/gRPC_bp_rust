// pub mod cart {
//     tonic::include_proto!("cart");
// }

// pub mod homepage {
//     tonic::include_proto!("homepage");
// }

// pub mod product {
//     tonic::include_proto!("product");
// }

pub mod user {
    tonic::include_proto!("user");
}

// Re-export services for easy access in other modules
// pub use cart::checkout_server::CheckoutServer;
// pub use homepage::banner_server::BannerServer;
// pub use product::get_product_server::GetProductServer;
pub use user::user_service_server::UserServiceServer;
