#![feature(proc_macro_hygiene, decl_macro)]
#![warn(rust_2018_idioms)]

mod client_server;
mod database;
mod error;
mod pdu;
mod ruma_wrapper;
mod server_server;
mod utils;

#[cfg(test)]
mod test;

pub use database::Database;
pub use error::{Error, Result};
pub use pdu::PduEvent;
pub use ruma_wrapper::{MatrixResult, Ruma};

use directories::ProjectDirs;
use rocket::{config::ConfigError, fairing::AdHoc, routes};
use serde::Deserialize;

fn setup_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                client_server::get_supported_versions_route,
                client_server::get_register_available_route,
                client_server::register_route,
                client_server::get_login_route,
                client_server::login_route,
                client_server::get_capabilities_route,
                client_server::get_pushrules_all_route,
                client_server::set_pushrule_route,
                client_server::set_pushrule_enabled_route,
                client_server::get_filter_route,
                client_server::create_filter_route,
                client_server::set_global_account_data_route,
                client_server::get_global_account_data_route,
                client_server::set_displayname_route,
                client_server::get_displayname_route,
                client_server::set_avatar_url_route,
                client_server::get_avatar_url_route,
                client_server::get_profile_route,
                client_server::set_presence_route,
                client_server::get_keys_route,
                client_server::upload_keys_route,
                client_server::set_read_marker_route,
                client_server::create_typing_event_route,
                client_server::create_room_route,
                client_server::get_alias_route,
                client_server::join_room_by_id_route,
                client_server::join_room_by_id_or_alias_route,
                client_server::leave_room_route,
                client_server::forget_room_route,
                client_server::invite_user_route,
                client_server::get_public_rooms_filtered_route,
                client_server::search_users_route,
                client_server::get_member_events_route,
                client_server::get_protocols_route,
                client_server::create_message_event_route,
                client_server::create_state_event_for_key_route,
                client_server::create_state_event_for_empty_key_route,
                client_server::sync_route,
                client_server::get_message_events_route,
                client_server::turn_server_route,
                client_server::publicised_groups_route,
                client_server::send_event_to_device_route,
                client_server::get_media_config_route,
                client_server::options_route,
                server_server::well_known_server,
                server_server::get_server_version,
                server_server::get_server_keys,
                server_server::get_server_keys_deprecated,
            ],
        )
        .attach(AdHoc::on_attach("Config", |rocket| {
            let data = Database::load_or_create(&rocket.config());

            Ok(rocket.manage(data))
        }))
}

fn main() {
    // Log info by default
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "warn");
    }

    setup_rocket().launch().unwrap();
}
