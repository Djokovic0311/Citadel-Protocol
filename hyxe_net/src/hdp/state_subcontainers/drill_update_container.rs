use tokio::time::Duration;

use crate::constants::{DRILL_UPDATE_FREQUENCY_DIVINE_BASE, DRILL_UPDATE_FREQUENCY_HIGH_BASE, DRILL_UPDATE_FREQUENCY_LOW_BASE, DRILL_UPDATE_FREQUENCY_MEDIUM_BASE, DRILL_UPDATE_FREQUENCY_ULTRA_BASE};
use crate::hdp::time::TransferStats;
use hyxe_crypt::hyper_ratchet::constructor::HyperRatchetConstructor;

#[derive(Default)]
pub struct DrillUpdateState {
    pub(crate) alice_hyper_ratchet: Option<HyperRatchetConstructor>
}

impl DrillUpdateState {
    pub fn stage0_alice(&mut self, hyper_ratchet: HyperRatchetConstructor) {
        self.alice_hyper_ratchet = Some(hyper_ratchet);
    }
}

/// Calculates the frequency, in nanoseconds per update
pub fn calculate_update_frequency(security_level: u8, _transfer_stats: &TransferStats) -> Duration {
    match security_level {
        0 => {
            Duration::from_nanos(DRILL_UPDATE_FREQUENCY_LOW_BASE)
        }

        1 => {
            Duration::from_nanos(DRILL_UPDATE_FREQUENCY_MEDIUM_BASE)
        }

        2 => {
            Duration::from_nanos(DRILL_UPDATE_FREQUENCY_HIGH_BASE)
        }

        3 => {
            Duration::from_nanos(DRILL_UPDATE_FREQUENCY_ULTRA_BASE)
        }

        4 => {
            Duration::from_nanos(DRILL_UPDATE_FREQUENCY_DIVINE_BASE)
        }

        _ => {
            panic!("Invalid security level!")
        }
    }
}