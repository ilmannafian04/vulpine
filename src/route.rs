use actix_web::web::{self, ServiceConfig};

use crate::controller as c;

pub fn configuration(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").route("/ping", web::get().to(c::ping)));
}
