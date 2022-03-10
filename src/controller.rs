use actix_web::{web, HttpResponse, Responder};
use log::info;
use rand::Rng;

use crate::dto::SleepArgs;

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn sleep(query: web::Query<SleepArgs>) -> impl Responder {
    if !query.random {
        match query.duration {
            Some(duration) => {
                info!("thread will sleep for {}ms", query.duration.unwrap());
                std::thread::sleep(std::time::Duration::from_millis(duration));
            }
            None => return HttpResponse::BadRequest().finish(),
        }
    } else {
        if let (Some(min), Some(max)) = (query.min_duration, query.max_duration) {
            let duration = rand::thread_rng().gen_range(min..max);
            info!("thread will sleep for {}ms", duration);
            std::thread::sleep(std::time::Duration::from_millis(duration));
        } else {
            return HttpResponse::BadRequest().finish();
        }
    }
    HttpResponse::Ok().finish()
}
