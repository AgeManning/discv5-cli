use discv5::enr::{CombinedKey, CombinedPublicKey};
use libp2p_core::{multiaddr::Protocol, Multiaddr};
use libp2p_identity::{Keypair, PeerId};

type Enr = discv5::enr::Enr<CombinedKey>;

/// Extend ENR for libp2p types.
pub trait EnrExt {
    /// The libp2p `PeerId` for the record.
    fn peer_id(&self) -> PeerId;

    /// Returns a list of multiaddrs if the ENR has an `ip` and either a `tcp` or `udp` key **or** an `ip6` and either a `tcp6` or `udp6`.
    /// The vector remains empty if these fields are not defined.
    fn multiaddr(&self) -> Vec<Multiaddr>;
}

/// Extend ENR CombinedPublicKey for libp2p types.
pub trait CombinedKeyPublicExt {
    #[allow(clippy::wrong_self_convention)]
    /// Converts the publickey into a peer id, without consuming the key.
    fn into_peer_id(&self) -> PeerId;
}

/// Extend ENR CombinedKey for conversion to libp2p keys.
pub trait CombinedKeyExt {
    /// Converts a libp2p key into an ENR combined key.
    fn from_libp2p(key: &Keypair) -> Result<CombinedKey, &'static str>;
}

impl EnrExt for Enr {
    /// The libp2p `PeerId` for the record.
    fn peer_id(&self) -> PeerId {
        self.public_key().into_peer_id()
    }

    /// Returns a list of multiaddrs if the ENR has an `ip` and either a `tcp` or `udp` key **or** an `ip6` and either a `tcp6` or `udp6`.
    /// The vector remains empty if these fields are not defined.
    ///
    /// Note: Only available with the `libp2p` feature flag.
    fn multiaddr(&self) -> Vec<Multiaddr> {
        let peer_id = self.peer_id();

        let mut multiaddrs: Vec<Multiaddr> = Vec::new();
        if let Some(ip) = self.ip4() {
            if let Some(udp) = self.udp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Udp(udp));
                multiaddr.push(Protocol::P2p(peer_id.clone().into()));
                multiaddrs.push(multiaddr);
            }

            if let Some(tcp) = self.tcp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Tcp(tcp));
                multiaddr.push(Protocol::P2p(peer_id.clone().into()));
                multiaddrs.push(multiaddr);
            }
        }
        if let Some(ip6) = self.ip6() {
            if let Some(udp6) = self.udp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Udp(udp6));
                multiaddr.push(Protocol::P2p(peer_id.clone().into()));
                multiaddrs.push(multiaddr);
            }

            if let Some(tcp6) = self.tcp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Tcp(tcp6));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }
        }
        multiaddrs
    }
}

impl CombinedKeyPublicExt for CombinedPublicKey {
    /// Converts the publickey into a peer id, without consuming the key.
    ///
    /// This is only available with the `libp2p` feature flag.
    fn into_peer_id(&self) -> PeerId {
        match self {
            Self::Secp256k1(pk) => {
                let pk_bytes = pk.to_bytes();
                let libp2p_pk = libp2p_identity::PublicKey::from(
                    libp2p_identity::secp256k1::PublicKey::try_from_bytes(&pk_bytes)
                        .expect("valid public key"),
                );
                PeerId::from_public_key(&libp2p_pk)
            }
            Self::Ed25519(pk) => {
                let pk_bytes = pk.to_bytes();
                let libp2p_pk = libp2p_identity::PublicKey::from(
                    libp2p_identity::ed25519::PublicKey::try_from_bytes(&pk_bytes)
                        .expect("valid public key"),
                );
                PeerId::from_public_key(&libp2p_pk)
            }
        }
    }
}

impl CombinedKeyExt for CombinedKey {
    fn from_libp2p(key: &Keypair) -> Result<CombinedKey, &'static str> {
        if let Ok(key) = key.clone().try_into_secp256k1() {
            let secret = discv5::enr::k256::ecdsa::SigningKey::from_bytes(&key.secret().to_bytes())
                .expect("libp2p key must be valid");
            Ok(CombinedKey::Secp256k1(secret))
        } else if let Ok(key) = key.clone().try_into_ed25519() {
            let ed_keypair =
                discv5::enr::ed25519_dalek::SecretKey::from_bytes(&key.to_bytes()[..32])
                    .expect("libp2p key must be valid");
            Ok(CombinedKey::from(ed_keypair))
        } else {
            Err("ENR: Unsupported libp2p key type")
        }
    }
}
