use rocket::serde::json::Json;

use bambangshop_receiver::Result;
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::service::notification::NotificationService;

#[get("/subscribe/<product_type>")]
pub fn subscribe(product_type: String) -> Result<Json<SubscriberRequest>> {
    match NotificationService::subscribe(&product_type) {
        Ok(x) => Ok(Json::from(x)),
        Err(e) => Err(e),
    }
}