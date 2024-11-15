use ethers::providers::{Provider, Ws};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// First, create a WsPool struct
pub struct WsPool {
    connections: Vec<Arc<Provider<Ws>>>,
    current: AtomicUsize,
}

impl WsPool {
    pub async fn new(ws_url: &str, pool_size: usize) -> anyhow::Result<Self> {
        let mut connections = Vec::with_capacity(pool_size);

        for _ in 0..pool_size {
            let provider = Provider::<Ws>::connect(ws_url).await?;
            connections.push(Arc::new(provider));
        }

        Ok(Self {
            connections,
            current: AtomicUsize::new(0),
        })
    }

    pub fn get_connection(&self) -> Arc<Provider<Ws>> {
        let index = self.current.fetch_add(1, Ordering::SeqCst) % self.connections.len();
        self.connections[index].clone()
    }
}
