pub(self) mod account_data;
pub(self) mod global_edus;
pub(self) mod globals;
pub(self) mod rooms;
pub(self) mod users;

use directories::ProjectDirs;
use std::fs::remove_dir_all;

use rocket::Config;

pub struct Database {
    pub globals: globals::Globals,
    pub users: users::Users,
    pub rooms: rooms::Rooms,
    pub account_data: account_data::AccountData,
    pub global_edus: global_edus::GlobalEdus,
    pub _db: sled::Db,
}

impl Database {
    /// Tries to remove the old database but ignores all errors.
    pub fn try_remove(hostname: &str) {
        let mut path = ProjectDirs::from("xyz", "koesters", "conduit")
            .unwrap()
            .data_dir()
            .to_path_buf();
        path.push(hostname);
        let _ = remove_dir_all(path);
    }

    /// Load an existing database or create a new one.
    pub fn load_or_create(config: &Config) -> Self {
        let _hostname = config.get_str("hostname").unwrap_or("localhost");
        let server_name = config.get_str("server_name").unwrap_or("localhost");

        let path = config
            .get_str("database_path")
            .map(|x| x.to_owned())
            .unwrap_or_else(|_| {
                let path = ProjectDirs::from("xyz", "koesters", "conduit")
                    .unwrap()
                    .data_dir()
                    .join(server_name);
                path.to_str().unwrap().to_owned()
            });

        let db = sled::open(&path).unwrap();
        log::info!("Opened sled database at {}", path);

        Self {
            globals: globals::Globals::load(
                db.open_tree("global").unwrap(),
                server_name.to_owned(),
            ),
            users: users::Users {
                userid_password: db.open_tree("userid_password").unwrap(),
                userdeviceids: db.open_tree("userdeviceids").unwrap(),
                userid_displayname: db.open_tree("userid_displayname").unwrap(),
                userid_avatarurl: db.open_tree("userid_avatarurl").unwrap(),
                userdeviceid_token: db.open_tree("userdeviceid_token").unwrap(),
                token_userid: db.open_tree("token_userid").unwrap(),
            },
            rooms: rooms::Rooms {
                edus: rooms::RoomEdus {
                    roomuserid_lastread: db.open_tree("roomuserid_lastread").unwrap(),
                    roomlatestid_roomlatest: db.open_tree("roomlatestid_roomlatest").unwrap(), // Read receipts
                    roomactiveid_roomactive: db.open_tree("roomactiveid_roomactive").unwrap(), // Typing notifs
                },
                pduid_pdu: db.open_tree("pduid_pdu").unwrap(),
                eventid_pduid: db.open_tree("eventid_pduid").unwrap(),
                roomid_pduleaves: db.open_tree("roomid_pduleaves").unwrap(),
                roomstateid_pdu: db.open_tree("roomstateid_pdu").unwrap(),

                userroomid_joined: db.open_tree("userroomid_joined").unwrap(),
                roomuserid_joined: db.open_tree("roomuserid_joined").unwrap(),
                userroomid_invited: db.open_tree("userroomid_invited").unwrap(),
                roomuserid_invited: db.open_tree("roomuserid_invited").unwrap(),
                userroomid_left: db.open_tree("userroomid_left").unwrap(),
            },
            account_data: account_data::AccountData {
                roomuserdataid_accountdata: db.open_tree("roomuserdataid_accountdata").unwrap(),
            },
            global_edus: global_edus::GlobalEdus {
                //globalallid_globalall: db.open_tree("globalallid_globalall").unwrap(),
                globallatestid_globallatest: db.open_tree("globallatestid_globallatest").unwrap(), // Presence
            },
            _db: db,
        }
    }
}
