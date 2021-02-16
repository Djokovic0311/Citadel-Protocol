use crate::hdp::hdp_packet_processor::includes::SocketAddr;
use hyxe_crypt::hyper_ratchet::constructor::HyperRatchetConstructor;
use hyxe_crypt::fcm::fcm_ratchet::FcmRatchetConstructor;

#[derive(Default)]
pub struct PeerKemStateContainer {
    pub(crate) last_state: u8,
    pub(crate) constructor: Option<HyperRatchetConstructor>,
    pub(crate) fcm_constructor: Option<FcmRatchetConstructor>,
    // during the NAT-traversal sage, this gets flipped ON if the local node makes it across
    pub(crate) p2p_conn_established: bool,
    pub(crate) local_is_initiator: bool,
    pub(crate) verified_socket_addr: Option<SocketAddr>
}