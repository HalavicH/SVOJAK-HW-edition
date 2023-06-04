pub mod api {
    pub mod dto;
    pub mod mapper;

    pub mod controller {
        pub mod startup;
        pub mod gameplay;
    }
}

pub mod core {
    pub mod game_entities;
    pub mod hub_manager;
}

pub mod game_pack {
    mod pack_content_dto;
    pub mod pack_content_entities;
    pub mod pack_content_loader;
    pub mod game_pack_entites;
    pub mod game_pack_loader;
}
pub mod hw_comm {
    pub mod api;
}
pub mod tests {
}