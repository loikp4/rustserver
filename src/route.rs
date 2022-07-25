//use crate::controllers::{posts, top};
use actix_web::web;

pub fn top(cfg: &mut web::ServiceConfig){
        cfg.route("/tweet", web::get());
}


