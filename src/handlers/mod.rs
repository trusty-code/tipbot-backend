use actix_web::{web, web::ServiceConfig,HttpResponse};
use did::utils::generate_seed;
use tracing::info;

pub fn app_config(config: &mut ServiceConfig) {
    let health_resource = web::resource("/")
        .route(web::get().to(health));
    
    config.service(health_resource);

    let rnd_seed_resource = web::resource("/rndSeed")
        .route(web::get().to(rnd_seed));
    
    config.service(rnd_seed_resource);
}



pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn rnd_seed() -> HttpResponse {
    let seed = generate_seed::new();
    info!("Generated Seed: {}", seed);
    HttpResponse::Ok().finish()
}