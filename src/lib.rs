mod routes;

use routes::create_route;

pub async fn run() {
    let app = create_route();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
