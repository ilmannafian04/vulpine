use actix_web::{middleware::Logger, web, web::ServiceConfig};

use crate::controller as c;

pub fn configuration(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/ping", web::get().to(c::ping))
            .wrap(Logger::default()),
    );
}
