#[tarpc::service]
pub trait World {
    /// Returns a greeting for name.
    async fn hello(name: String) -> String;
    async fn read_file(file: String) -> String;
    async fn write_file(file: String, contents: String);
}
