use libp2p::PeerId;
use log::{debug, error};
use parity_multiaddr::Multiaddr;
use std::str::FromStr;

pub fn get_boot_nodes(boot_nodes: Vec<&str>) -> Vec<(Multiaddr, PeerId)> {
    boot_nodes
        .iter()
        .filter_map(|bn_str| {
            let entry: Vec<&str> = bn_str.split("/p2p/").collect();
            match entry.as_slice() {
                [multiaddr_str, peerid_str] => {
                    debug!("multiaddr {}, peerid: {}", multiaddr_str, peerid_str);

                    match (
                        multiaddr_str.parse::<Multiaddr>(),
                        PeerId::from_str(peerid_str),
                    ) {
                        (Ok(multiaddr), Ok(peerid)) => Some((multiaddr, peerid)),
                        (multiaddr_err, peerid_err) => {
                            error!(
                                "Error parsing bootstrap node. Multiaddr: {:?}, Peerid: {:?}",
                                multiaddr_err, peerid_err
                            );
                            None
                        }
                    }
                }
                _ => None,
            }
        })
        .collect()
}

#[test]
fn test_get_boot_nodes() {
    // Actual bootstrap nodes from `ipfs bootstrap list`
    let bootstrap_list = vec![
        "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
        "/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
        "/ip4/104.131.131.82/udp/4001/quic/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
    ];

    let results = get_boot_nodes(bootstrap_list);

    assert_eq!(results.len(), 3, "parses all three multiaddrs");

    assert_eq!(
        results[0].0.to_string(),
        "/dnsaddr/bootstrap.libp2p.io",
        "Parses dnsaddr p2p multiaddr"
    );
    assert_eq!(
        results[1].0.to_string(),
        "/ip4/104.131.131.82/tcp/4001",
        "Parses tcp ipv4 p2p multiaddr"
    );
    assert_eq!(
        results[2].0.to_string(),
        "/ip4/104.131.131.82/udp/4001/quic",
        "Parses udp ipv4 quic multiaddr"
    );

    assert_eq!(
        results[0].1.to_string(),
        "QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
        "Parses dnsaddr p2p peerid"
    );
    assert_eq!(
        results[1].1.to_string(),
        "QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
        "Parses tcp ipv4 p2p peerid"
    );
    assert_eq!(
        results[2].1.to_string(),
        "QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
        "Parses udp ipv4 quic peerid"
    );
}
