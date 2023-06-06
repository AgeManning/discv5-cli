use discv5::{ConnectionState, Discv5, Discv5ConfigBuilder, ListenConfig};
use discv5_cli::server::{bootstrap, command, enr_build, keys};

#[tokio::test]
pub async fn test_bootstrap_file() -> std::result::Result<(), eyre::Report> {
    // Mock Server Input
    let server = command::Server {
        listen_addresses: "0.0.0.0".to_string(),
        listen_port: 9000,
        enr_addresses: Some("45.31.182.164".to_string()),
        enr_v4_port: Some(50862),
        enr: Some("-KO4QMMX2IadOccPtAC29ueqrRzNEO5omCNRRTctz-QR20eyMwVR9yl6IDsoxmkg4naImuve8u2H6FO9fsmlXxso-suCAsKEZXRoMpBKJsWLAgAAAP__________gmlkgnY0gmlwhC0ftqSJc2VjcDI1NmsxoQI3vMd58jjnDZQm0KGQpbuWGSW9Bnon1GsF07XnlHna6YN0Y3CCekqDdWRwgsau".to_string()),
        bootstrap: Some("example.bootstrap.json".to_string()),
        ..Default::default()
    };

    let enr_key = keys::generate(&server).unwrap();
    let enr = enr_build::build(&server, &enr_key).unwrap();
    let listen_config = ListenConfig::from_ip(
        server
            .listen_addresses
            .parse::<std::net::IpAddr>()
            .expect("Valid IP"),
        server.listen_port,
    );
    let config = Discv5ConfigBuilder::new(listen_config)
        .enr_peer_update_min(2usize)
        .build();
    let mut discv5 = Discv5::new(enr, enr_key, config).unwrap();

    // Bootstrap the server peers
    bootstrap::boostrap(&mut discv5, server.bootstrap.clone())
        .await
        .unwrap();

    assert!(!discv5.table_entries().is_empty());

    assert_eq!(
        discv5
            .table_entries()
            .into_iter()
            .filter_map(|(node_id, _, node_status)| {
                println!("Node status: {:?}", node_status);
                if node_status.state == ConnectionState::Connected {
                    Some(node_id)
                } else {
                    None
                }
            })
            .count(),
        0
    );
    assert!(discv5
        .table_entries()
        .into_iter()
        .filter_map(|(node_id, _, node_status)| {
            if node_status.state == ConnectionState::Disconnected {
                Some(node_id)
            } else {
                None
            }
        })
        .next()
        .is_some());

    assert_eq!(discv5.connected_peers(), 0);

    Ok(())
}
