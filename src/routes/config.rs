use actix_web::web;

use super::customer::{get_customers};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_customers);

    conf.service(scope);
}