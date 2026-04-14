use crate::protocol::{cmd, Packet};

/// Voice control actions for alarm recording/playback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AlarmVoiceControl {
    Record = 0,
    Play = 1,
    Stop = 2,
}

/// Bitflag helper for selecting weekdays in alarm schedules.
///
/// Bits 0-6 correspond to Monday through Sunday.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Weekdays(pub u8);

impl Weekdays {
    pub const MON: Weekdays = Weekdays(1 << 0);
    pub const TUE: Weekdays = Weekdays(1 << 1);
    pub const WED: Weekdays = Weekdays(1 << 2);
    pub const THU: Weekdays = Weekdays(1 << 3);
    pub const FRI: Weekdays = Weekdays(1 << 4);
    pub const SAT: Weekdays = Weekdays(1 << 5);
    pub const SUN: Weekdays = Weekdays(1 << 6);
    pub const ALL: Weekdays = Weekdays(0x7F);

    /// Returns true if no weekdays are set.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns true if the given day flag is set.
    pub fn contains(self, other: Weekdays) -> bool {
        self.0 & other.0 == other.0
    }
}

impl std::ops::BitOr for Weekdays {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Weekdays(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Weekdays {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Weekdays(self.0 & rhs.0)
    }
}

/// Retrieve all configured alarms from the device.
pub fn get_alarms() -> Packet {
    Packet::new(cmd::SPP_GET_ALARM_TIME_SCENE, vec![])
}

/// Set or update an alarm on the device.
///
/// `speed` is encoded as little-endian u16.
#[allow(clippy::too_many_arguments)]
pub fn set_alarm(
    id: u8,
    enabled: bool,
    hour: u8,
    minute: u8,
    mode: u8,
    weekdays: u8,
    frequency: u8,
    speed: u16,
    volume: u8,
) -> Packet {
    Packet::new(
        cmd::SPP_SET_ALARM_TIME_SCENE,
        vec![
            id,
            if enabled { 1 } else { 0 },
            hour,
            minute,
            mode,
            weekdays,
            frequency,
            (speed & 0xFF) as u8,
            (speed >> 8) as u8,
            volume,
        ],
    )
}

/// Enable or disable alarm listen mode with two additional parameters.
pub fn set_alarm_listen(enabled: bool, param1: u8, param2: u8) -> Packet {
    Packet::new(
        cmd::SPP_SET_ALARM_LISTEN,
        vec![if enabled { 1 } else { 0 }, param1, param2],
    )
}

/// Set the volume for alarm listen mode.
pub fn set_alarm_listen_volume(volume: u8) -> Packet {
    Packet::new(cmd::SPP_SET_ALARM_LISTEN_VOLUME, vec![volume])
}

/// Send an alarm voice control command (record, play, or stop) with a volume level.
pub fn set_alarm_voice_ctrl(control: AlarmVoiceControl, volume: u8) -> Packet {
    Packet::new(
        cmd::SPP_SET_ALARM_VOICE_CTRL,
        vec![control as u8, volume],
    )
}

/// Configure the sleep timer from a raw config slice.
pub fn set_sleep_time(config: &[u8]) -> Packet {
    Packet::new(cmd::SPP_SET_SLEEP_TIME, config.to_vec())
}

/// Query the current sleep mode configuration.
///
/// Sends SPP_SET_SLEEP_TIME with a 0xFF payload byte to trigger a query response.
pub fn get_sleep_mode() -> Packet {
    Packet::new(cmd::SPP_SET_SLEEP_TIME, vec![0xFF])
}

/// Set the sleep mode ambient light color.
pub fn set_sleep_color(r: u8, g: u8, b: u8) -> Packet {
    Packet::new(cmd::SPP_SET_SLEEP_COLOR, vec![r, g, b])
}

/// Set the sleep mode light brightness level.
pub fn set_sleep_light(level: u8) -> Packet {
    Packet::new(cmd::SPP_SET_SLEEP_LIGHT, vec![level])
}

/// Configure the sleep scene from a raw config slice.
pub fn set_sleep_scene(config: &[u8]) -> Packet {
    Packet::new(cmd::SPP_SET_SLEEP_SCENE, config.to_vec())
}

/// Set the auto power-off timeout in minutes (little-endian u16).
pub fn set_auto_power_off(minutes: u16) -> Packet {
    Packet::new(
        cmd::SPP_SET_AUTO_POWER_OFF,
        vec![(minutes & 0xFF) as u8, (minutes >> 8) as u8],
    )
}

/// Query the current auto power-off setting.
pub fn get_auto_power_off() -> Packet {
    Packet::new(cmd::SPP_GET_AUTO_POWER_OFF, vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_alarms() {
        let pkt = get_alarms();
        assert_eq!(pkt.command, cmd::SPP_GET_ALARM_TIME_SCENE);
        assert!(pkt.payload.is_empty());
    }

    #[test]
    fn test_set_alarm_payload() {
        let pkt = set_alarm(3, true, 7, 30, 1, 0x1F, 2, 0x0201, 80);
        assert_eq!(pkt.command, cmd::SPP_SET_ALARM_TIME_SCENE);
        assert_eq!(
            pkt.payload,
            vec![3, 1, 7, 30, 1, 0x1F, 2, 0x01, 0x02, 80]
        );
    }

    #[test]
    fn test_set_alarm_disabled() {
        let pkt = set_alarm(0, false, 12, 0, 0, 0, 0, 0, 50);
        assert_eq!(pkt.payload[1], 0);
    }

    #[test]
    fn test_set_alarm_speed_encoding() {
        // Verify little-endian u16 encoding for speed
        let pkt = set_alarm(1, true, 8, 0, 0, 0, 0, 0x1234, 100);
        assert_eq!(pkt.payload[7], 0x34); // low byte
        assert_eq!(pkt.payload[8], 0x12); // high byte
    }

    #[test]
    fn test_set_alarm_listen() {
        let pkt = set_alarm_listen(true, 5, 10);
        assert_eq!(pkt.command, cmd::SPP_SET_ALARM_LISTEN);
        assert_eq!(pkt.payload, vec![1, 5, 10]);
    }

    #[test]
    fn test_set_alarm_listen_disabled() {
        let pkt = set_alarm_listen(false, 0, 0);
        assert_eq!(pkt.payload[0], 0);
    }

    #[test]
    fn test_set_alarm_listen_volume() {
        let pkt = set_alarm_listen_volume(75);
        assert_eq!(pkt.command, cmd::SPP_SET_ALARM_LISTEN_VOLUME);
        assert_eq!(pkt.payload, vec![75]);
    }

    #[test]
    fn test_set_alarm_voice_ctrl_record() {
        let pkt = set_alarm_voice_ctrl(AlarmVoiceControl::Record, 60);
        assert_eq!(pkt.command, cmd::SPP_SET_ALARM_VOICE_CTRL);
        assert_eq!(pkt.payload, vec![0, 60]);
    }

    #[test]
    fn test_set_alarm_voice_ctrl_play() {
        let pkt = set_alarm_voice_ctrl(AlarmVoiceControl::Play, 80);
        assert_eq!(pkt.payload, vec![1, 80]);
    }

    #[test]
    fn test_set_alarm_voice_ctrl_stop() {
        let pkt = set_alarm_voice_ctrl(AlarmVoiceControl::Stop, 0);
        assert_eq!(pkt.payload, vec![2, 0]);
    }

    #[test]
    fn test_set_sleep_time() {
        let config = [0x01, 0x02, 0x03];
        let pkt = set_sleep_time(&config);
        assert_eq!(pkt.command, cmd::SPP_SET_SLEEP_TIME);
        assert_eq!(pkt.payload, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_get_sleep_mode() {
        let pkt = get_sleep_mode();
        assert_eq!(pkt.command, cmd::SPP_SET_SLEEP_TIME);
        assert_eq!(pkt.payload, vec![0xFF]);
    }

    #[test]
    fn test_set_sleep_color() {
        let pkt = set_sleep_color(255, 128, 0);
        assert_eq!(pkt.command, cmd::SPP_SET_SLEEP_COLOR);
        assert_eq!(pkt.payload, vec![255, 128, 0]);
    }

    #[test]
    fn test_set_sleep_light() {
        let pkt = set_sleep_light(50);
        assert_eq!(pkt.command, cmd::SPP_SET_SLEEP_LIGHT);
        assert_eq!(pkt.payload, vec![50]);
    }

    #[test]
    fn test_set_sleep_scene() {
        let config = [10, 20, 30, 40];
        let pkt = set_sleep_scene(&config);
        assert_eq!(pkt.command, cmd::SPP_SET_SLEEP_SCENE);
        assert_eq!(pkt.payload, vec![10, 20, 30, 40]);
    }

    #[test]
    fn test_set_auto_power_off() {
        let pkt = set_auto_power_off(120);
        assert_eq!(pkt.command, cmd::SPP_SET_AUTO_POWER_OFF);
        assert_eq!(pkt.payload, vec![120, 0]);
    }

    #[test]
    fn test_set_auto_power_off_large_value() {
        let pkt = set_auto_power_off(0xABCD);
        assert_eq!(pkt.payload, vec![0xCD, 0xAB]);
    }

    #[test]
    fn test_get_auto_power_off() {
        let pkt = get_auto_power_off();
        assert_eq!(pkt.command, cmd::SPP_GET_AUTO_POWER_OFF);
        assert!(pkt.payload.is_empty());
    }

    #[test]
    fn test_weekdays_individual() {
        assert_eq!(Weekdays::MON.0, 0b0000001);
        assert_eq!(Weekdays::TUE.0, 0b0000010);
        assert_eq!(Weekdays::WED.0, 0b0000100);
        assert_eq!(Weekdays::THU.0, 0b0001000);
        assert_eq!(Weekdays::FRI.0, 0b0010000);
        assert_eq!(Weekdays::SAT.0, 0b0100000);
        assert_eq!(Weekdays::SUN.0, 0b1000000);
    }

    #[test]
    fn test_weekdays_all() {
        assert_eq!(Weekdays::ALL.0, 0x7F);
        assert!(Weekdays::ALL.contains(Weekdays::MON));
        assert!(Weekdays::ALL.contains(Weekdays::SUN));
    }

    #[test]
    fn test_weekdays_bitor() {
        let weekdays = Weekdays::MON | Weekdays::WED | Weekdays::FRI;
        assert_eq!(weekdays.0, 0b0010101);
        assert!(weekdays.contains(Weekdays::MON));
        assert!(weekdays.contains(Weekdays::FRI));
        assert!(!weekdays.contains(Weekdays::TUE));
    }

    #[test]
    fn test_weekdays_bitand() {
        let weekdays = Weekdays::ALL & Weekdays::MON;
        assert_eq!(weekdays, Weekdays::MON);
    }

    #[test]
    fn test_weekdays_is_empty() {
        assert!(Weekdays(0).is_empty());
        assert!(!Weekdays::MON.is_empty());
    }

    #[test]
    fn test_alarm_voice_control_repr() {
        assert_eq!(AlarmVoiceControl::Record as u8, 0);
        assert_eq!(AlarmVoiceControl::Play as u8, 1);
        assert_eq!(AlarmVoiceControl::Stop as u8, 2);
    }
}
