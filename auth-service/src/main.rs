use std::{collections::HashMap, sync::Arc};

use auth_service::{app_state, services::hashmap_user_store, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = hashmap_user_store::HashmapUserStore{
        users: HashMap::new(),
    };
    let app_state = app_state::AppState::new(Arc::new(RwLock::new(user_store)));

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
