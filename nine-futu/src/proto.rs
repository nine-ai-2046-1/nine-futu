pub mod common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

pub mod init_connect {
    include!(concat!(env!("OUT_DIR"), "/init_connect.rs"));
}

pub mod keep_alive {
    include!(concat!(env!("OUT_DIR"), "/keep_alive.rs"));
}

pub mod get_global_state {
    include!(concat!(env!("OUT_DIR"), "/get_global_state.rs"));
}

pub mod qot_common {
    include!(concat!(env!("OUT_DIR"), "/qot_common.rs"));
}

pub mod qot_get_security_snapshot {
    include!(concat!(env!("OUT_DIR"), "/qot_get_security_snapshot.rs"));
}

pub mod qot_get_basic_qot {
    include!(concat!(env!("OUT_DIR"), "/qot_get_basic_qot.rs"));
}

pub mod qot_get_kl {
    include!(concat!(env!("OUT_DIR"), "/qot_get_kl.rs"));
}

pub mod qot_request_history_kl {
    include!(concat!(env!("OUT_DIR"), "/qot_request_history_kl.rs"));
}

pub mod qot_get_order_book {
    include!(concat!(env!("OUT_DIR"), "/qot_get_order_book.rs"));
}

pub mod qot_get_ticker {
    include!(concat!(env!("OUT_DIR"), "/qot_get_ticker.rs"));
}

pub mod qot_get_market_state {
    include!(concat!(env!("OUT_DIR"), "/qot_get_market_state.rs"));
}

pub mod qot_get_capital_flow {
    include!(concat!(env!("OUT_DIR"), "/qot_get_capital_flow.rs"));
}

pub mod qot_get_plate_set {
    include!(concat!(env!("OUT_DIR"), "/qot_get_plate_set.rs"));
}

pub mod qot_get_plate_security {
    include!(concat!(env!("OUT_DIR"), "/qot_get_plate_security.rs"));
}

pub mod qot_get_static_info {
    include!(concat!(env!("OUT_DIR"), "/qot_get_static_info.rs"));
}

pub mod qot_request_history_kl_quota {
    include!(concat!(env!("OUT_DIR"), "/qot_request_history_kl_quota.rs"));
}

pub mod qot_sub {
    include!(concat!(env!("OUT_DIR"), "/qot_sub.rs"));
}

pub mod qot_get_sub_info {
    include!(concat!(env!("OUT_DIR"), "/qot_get_sub_info.rs"));
}

pub mod qot_update_basic_qot {
    include!(concat!(env!("OUT_DIR"), "/qot_update_basic_qot.rs"));
}

pub mod qot_update_kl {
    include!(concat!(env!("OUT_DIR"), "/qot_update_kl.rs"));
}

pub mod qot_update_order_book {
    include!(concat!(env!("OUT_DIR"), "/qot_update_order_book.rs"));
}

pub mod qot_update_ticker {
    include!(concat!(env!("OUT_DIR"), "/qot_update_ticker.rs"));
}
