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
    pub mod game_logic;
}

pub mod game_pack {
    mod pack_content_dto;
    pub mod pack_content_entities;
    pub mod pack_content_loader;
    pub mod game_pack_entites;
    pub mod game_pack_loader;
}

pub mod hub_comm {
    pub mod common {
        pub mod hub_api;
    }
    pub mod hw {
        pub mod hw_hub;
        pub mod hw_hub_manager;
        pub mod virtual_hw_hub;
        pub mod internal {
            pub mod api_types;
            pub mod hub_protocol_io_handler;
            pub mod byte_handler;
        }
    }
}
