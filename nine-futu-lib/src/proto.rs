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

pub mod trd_common {
    include!(concat!(env!("OUT_DIR"), "/trd_common.rs"));
}

pub mod trd_get_acc_list {
    include!(concat!(env!("OUT_DIR"), "/trd_get_acc_list.rs"));
}

pub mod trd_get_funds {
    include!(concat!(env!("OUT_DIR"), "/trd_get_funds.rs"));
}

pub mod trd_get_position_list {
    include!(concat!(env!("OUT_DIR"), "/trd_get_position_list.rs"));
}

pub mod trd_place_order {
    include!(concat!(env!("OUT_DIR"), "/trd_place_order.rs"));
}

pub mod trd_modify_order {
    include!(concat!(env!("OUT_DIR"), "/trd_modify_order.rs"));
}

pub mod trd_get_order_list {
    include!(concat!(env!("OUT_DIR"), "/trd_get_order_list.rs"));
}

pub mod trd_get_order_fill_list {
    include!(concat!(env!("OUT_DIR"), "/trd_get_order_fill_list.rs"));
}

pub mod trd_get_history_order_list {
    include!(concat!(env!("OUT_DIR"), "/trd_get_history_order_list.rs"));
}

pub mod trd_get_history_order_fill_list {
    include!(concat!(env!("OUT_DIR"), "/trd_get_history_order_fill_list.rs"));
}

pub mod trd_unlock_trade {
    include!(concat!(env!("OUT_DIR"), "/trd_unlock_trade.rs"));
}

pub mod trd_sub_acc_push {
    include!(concat!(env!("OUT_DIR"), "/trd_sub_acc_push.rs"));
}
