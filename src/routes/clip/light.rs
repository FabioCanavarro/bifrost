use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use serde_json::Value;
use uuid::Uuid;

use crate::hue::api::{Light, LightUpdate, RType, V2Reply};
use crate::routes::clip::ApiV2Result;
use crate::routes::extractor::Json;
use crate::server::appstate::AppState;
use crate::z2m::request::ClientRequest;
use crate::z2m::update::DeviceUpdate;

async fn put_light(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(put): Json<Value>,
) -> ApiV2Result {
    log::info!("PUT light/{id}");
    log::debug!("json data\n{}", serde_json::to_string_pretty(&put)?);

    let rlink = RType::Light.link_to(id);
    let lock = state.res.lock().await;

    let _ = lock.get::<Light>(&rlink)?;

    let upd: LightUpdate = serde_json::from_value(put)?;

    let payload = DeviceUpdate::default()
        .with_state(upd.on.map(|on| on.on))
        .with_brightness(upd.dimming.map(|dim| dim.brightness / 100.0 * 254.0))
        .with_color_temp(upd.color_temperature.map(|ct| ct.mirek))
        .with_color_xy(upd.color.map(|col| col.xy));

    lock.z2m_request(ClientRequest::light_update(rlink, payload))?;

    drop(lock);

    V2Reply::ok(rlink)
}

async fn get_light(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiV2Result {
    V2Reply::ok(state.res.lock().await.get_resource(RType::Light, &id)?)
}

pub fn router() -> Router<AppState> {
    Router::new().route("/:id", get(get_light).put(put_light))
}
