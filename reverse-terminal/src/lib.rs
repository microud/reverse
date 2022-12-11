mod terminal;
mod management;

use axum::extract::WebSocketUpgrade;
use axum::Router;
use axum::routing::{get, post, put, delete};
use crate::terminal::{handle_start_pty_terminal_socket};

pub fn register_terminal_router(app: Router) -> Router {
    // let terminal_management = TerminalManagement::new(&app_storage);
    let terminal = Router::new()
        .route("/openpty", get(|ws: WebSocketUpgrade| async {
            tracing::debug!("receive openpty websocket");
            ws.on_upgrade(handle_start_pty_terminal_socket)
        }))
        .route("/terminals", get(|| async {
            // terminal_management.list_terminals().await;
        }))
        .route("/terminals", post(|| async {
            // terminal_management.add_terminal().await;
        }))
        .route("/terminals/:id", put(|| async {
            // terminal_management.update_terminal();
        }))
        .route("/terminals/:id", delete(|| async {
            // terminal_management.update_terminal()
        }));

    app.nest("/terminal", terminal)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
