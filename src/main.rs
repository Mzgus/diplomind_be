use diplomind::routes;

use poem::{
   Server,listener::TcpListener
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let routes = routes();
	let _ = Server::new(TcpListener::bind("0.0.0.0:3000")).run(routes).await;
    Ok(())
}