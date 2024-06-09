type DefaultResult<T, E = Box<dyn std::error::Error>> = Result<T, E>;

#[tokio::main]
async fn main() -> DefaultResult<()> {
    Ok(())
}
