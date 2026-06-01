fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = &[
        "proto/Common.proto",
        "proto/InitConnect.proto",
        "proto/KeepAlive.proto",
        "proto/GetGlobalState.proto",
        "proto/Qot_Common.proto",
        "proto/Qot_GetSecuritySnapshot.proto",
        "proto/Qot_GetBasicQot.proto",
        "proto/Qot_GetKL.proto",
        "proto/Qot_RequestHistoryKL.proto",
        "proto/Qot_GetOrderBook.proto",
        "proto/Qot_GetTicker.proto",
        "proto/Qot_GetMarketState.proto",
        "proto/Qot_GetCapitalFlow.proto",
        "proto/Qot_GetPlateSet.proto",
        "proto/Qot_GetPlateSecurity.proto",
        "proto/Qot_GetStaticInfo.proto",
        "proto/Qot_RequestHistoryKLQuota.proto",
        "proto/Qot_Sub.proto",
        "proto/Qot_GetSubInfo.proto",
        "proto/Qot_UpdateBasicQot.proto",
        "proto/Qot_UpdateKL.proto",
        "proto/Qot_UpdateOrderBook.proto",
        "proto/Qot_UpdateTicker.proto",
    ];

    let mut config = prost_build::Config::new();
    config.compile_protos(proto_files, &["proto/"])?;

    Ok(())
}
