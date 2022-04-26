use actix_web::{middleware::Logger, web, web::ServiceConfig};

use crate::controller as c;

pub fn configuration(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/ping", web::get().to(c::ping))
            .route("/sleep", web::get().to(c::sleep))
            .route("/sleep/{value}", web::get().to(c::sleep))
            .route("/message", web::get().to(c::message))
            .wrap(Logger::default()),
    )
    .service(
        web::scope("/health")
            .route("/live", web::get().to(c::ping))
            .route("/ready", web::get().to(c::ping)),
    );
}
