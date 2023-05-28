pub mod api {
    pub mod gameplay;
    pub mod setup;
    pub mod dto;
    pub mod mapper;
}
pub mod core {
    pub mod game_entities;
    pub mod hub_manager;
}
pub mod game_pack {
    mod pack_dto;

    pub mod pack_entities;
    pub mod pack_loader;
}
pub mod hw_comm {
    pub mod api;
}
