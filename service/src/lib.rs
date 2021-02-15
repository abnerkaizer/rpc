#[tarpc::service]
pub trait World {
    /// Returns a greeting for name.
    async fn init(name: String) -> String;
}
