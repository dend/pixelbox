//! Device settings, power, info, game, and configuration commands for the Divoom Ditoo Pro.

use chrono::{Datelike, Local, Timelike};

use crate::protocol::{cmd, ext_cmd, Packet};

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

/// Set the device system time from individual components.
///
/// The year is encoded as two bytes: `(year % 100, year / 100)` in little-endian style.
pub fn set_system_time(
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    weekday: u8,
) -> Packet {
    Packet::new(
        cmd::SPP_SET_SYSTEM_TIME,
        vec![
            (year % 100) as u8,
            (year / 100) as u8,
            month,
            day,
            hour,
            minute,
            second,
            weekday,
        ],
    )
}

/// Set the device system time to the current local time.
pub fn set_time_now() -> Packet {
    let now = Local::now();
    set_system_time(
        now.year() as u16,
        now.month() as u8,
        now.day() as u8,
        now.hour() as u8,
        now.minute() as u8,
        now.second() as u8,
        now.weekday().num_days_from_sunday() as u8,
    )
}

/// Query the device system time (extended command).
pub fn get_system_time() -> Packet {
    Packet::ext(ext_cmd::SPP_GET_SYSTEM_TIME, vec![])
}

// ---------------------------------------------------------------------------
// Brightness
// ---------------------------------------------------------------------------

/// Set the display brightness level.
pub fn set_brightness(level: u8) -> Packet {
    Packet::new(cmd::SPP_SET_SYSTEM_BRIGHT, vec![level])
}

/// Query the current brightness level.
pub fn get_brightness() -> Packet {
    Packet::new(cmd::SPP_LIGHT_CURRENT_LEVEL, vec![])
}

/// Adjust the brightness level from raw data.
pub fn adjust_brightness(data: &[u8]) -> Packet {
    Packet::new(cmd::SPP_LIGHT_ADJUST_LEVEL, data.to_vec())
}

// ---------------------------------------------------------------------------
// Device Name
// ---------------------------------------------------------------------------

/// Set the device Bluetooth name.
///
/// Payload: `[name_len, ...name_bytes]`.
pub fn set_device_name(name: &str) -> Packet {
    let bytes = name.as_bytes();
    let mut payload = Vec::with_capacity(1 + bytes.len());
    payload.push(bytes.len() as u8);
    payload.extend_from_slice(bytes);
    Packet::new(cmd::SPP_SET_DEVICE_NAME, payload)
}

// ---------------------------------------------------------------------------
// Device Info
// ---------------------------------------------------------------------------

/// Query extended device information.
pub fn get_device_info() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_GET_DEVICE_INFO, vec![0x00])
}

/// Query the device connected flag.
pub fn get_connected_flag() -> Packet {
    Packet::new(cmd::SPP_GET_CONNECTED_FLAG, vec![])
}

/// Set the device connected flag.
pub fn set_connected_flag(flag: u8) -> Packet {
    Packet::new(cmd::SPP_SET_CONNECTED_FLAG, vec![flag])
}

// ---------------------------------------------------------------------------
// Power
// ---------------------------------------------------------------------------

/// Configure the power-on/off behaviour with mode, enable flag, and RGB colour.
///
/// Payload: `[0x01, mode, enabled, 0, 0, 0, 0, r, g, b]`.
pub fn set_power_on_off(mode: u8, enabled: bool, r: u8, g: u8, b: u8) -> Packet {
    Packet::new(
        cmd::SPP_SPP_POWER_ON_OFF_INFO,
        vec![0x01, mode, if enabled { 1 } else { 0 }, 0, 0, 0, 0, r, g, b],
    )
}

/// Query the current power-on/off configuration.
pub fn get_power_on_off() -> Packet {
    Packet::new(cmd::SPP_SPP_POWER_ON_OFF_INFO, vec![0x00])
}

/// Set the power channel.
pub fn set_power_channel(channel: u8) -> Packet {
    Packet::new(cmd::SPP_SET_POWER_CHANNEL, vec![0x01, channel])
}

/// Query the current power channel.
pub fn get_power_channel() -> Packet {
    Packet::new(cmd::SPP_SET_POWER_CHANNEL, vec![0x00])
}

// ---------------------------------------------------------------------------
// Energy
// ---------------------------------------------------------------------------

/// Enable or disable the energy-saving controller.
pub fn set_energy_ctrl(enabled: bool) -> Packet {
    Packet::new(cmd::SPP_SET_ENERGY_CTRL, vec![if enabled { 1 } else { 0 }])
}

/// Query the current energy-saving state.
pub fn get_energy_ctrl() -> Packet {
    Packet::new(cmd::SPP_GET_ENERGY_CTRL, vec![])
}

// ---------------------------------------------------------------------------
// Eye Guard
// ---------------------------------------------------------------------------

/// Enable or disable the eye-guard (blue-light filter) mode.
pub fn set_eye_guard(enabled: bool) -> Packet {
    Packet::new(cmd::SPP_EYE_GUARD_INFO, vec![if enabled { 1 } else { 0 }])
}

/// Query the current eye-guard state.
///
/// The device replies with the active state when it receives `0xFF`.
pub fn get_eye_guard() -> Packet {
    Packet::new(cmd::SPP_EYE_GUARD_INFO, vec![0xFF])
}

// ---------------------------------------------------------------------------
// Temperature
// ---------------------------------------------------------------------------

/// Set the temperature display unit: `true` for Celsius, `false` for Fahrenheit.
pub fn set_temp_type(celsius: bool) -> Packet {
    Packet::new(cmd::SPP_SET_TEMP_TYPE, vec![if celsius { 1 } else { 0 }])
}

/// Query the current temperature display unit.
///
/// The device replies with the active unit when it receives `0xFF`.
pub fn get_temp_type() -> Packet {
    Packet::new(cmd::SPP_SET_TEMP_TYPE, vec![0xFF])
}

/// Query the device internal temperature.
pub fn get_device_temp() -> Packet {
    Packet::new(cmd::SPP_GET_DEVICE_TEMP_INFO, vec![])
}

// ---------------------------------------------------------------------------
// Game
// ---------------------------------------------------------------------------

/// Send a game control key-press event.
pub fn game_control(key: u8) -> Packet {
    Packet::new(cmd::SPP_SEND_GAME_CTRL_INFO, vec![key])
}

/// Send a game control key-up (release) event.
pub fn game_key_up(key: u8) -> Packet {
    Packet::new(cmd::SPP_SEND_GAME_CTRL_KEY_UP_INFO, vec![key])
}

/// Start or stop a game with an additional parameter byte.
pub fn set_game(enabled: bool, param: u8) -> Packet {
    Packet::new(cmd::SPP_SET_GAME, vec![if enabled { 1 } else { 0 }, param])
}

// ---------------------------------------------------------------------------
// Tool Info (scoreboard, countdown, stopwatch, noise)
// ---------------------------------------------------------------------------

/// Query the status of a tool by type (0 = scoreboard, 1 = countdown, etc.).
pub fn get_tool_info(tool_type: u8) -> Packet {
    Packet::new(cmd::SPP_GET_TOOL_INFO, vec![tool_type])
}

/// Configure the countdown tool.
///
/// Payload: `[0x01, enabled, minutes_lo, minutes_hi, seconds_lo, seconds_hi]`.
pub fn set_tool_countdown(enabled: bool, minutes: u16, seconds: u16) -> Packet {
    Packet::new(
        cmd::SPP_SET_TOOL_INFO,
        vec![
            0x01,
            if enabled { 1 } else { 0 },
            (minutes & 0xFF) as u8,
            (minutes >> 8) as u8,
            (seconds & 0xFF) as u8,
            (seconds >> 8) as u8,
        ],
    )
}

/// Configure the noise meter tool.
pub fn set_tool_noise(param: u8) -> Packet {
    Packet::new(cmd::SPP_SET_TOOL_INFO, vec![0x02, param])
}

/// Configure the scoreboard tool.
///
/// Payload: `[0x03, enabled, score1, score2]`.
pub fn set_tool_scoreboard(enabled: bool, score1: u16, score2: u16) -> Packet {
    Packet::new(
        cmd::SPP_SET_TOOL_INFO,
        vec![0x03, if enabled { 1 } else { 0 }, score1 as u8, score2 as u8],
    )
}

// ---------------------------------------------------------------------------
// Language
// ---------------------------------------------------------------------------

/// Set the device UI language.
///
/// Language indices: 0=en, 1=zh-hans, 2=zh-hant, 3=ja, 4=th, 5=fr, 6=it,
/// 7=iv, 8=es, 9=de, 10=ru, 11=pt, 12=ko, 13=nl, 14=uk, 15=ms.
pub fn set_language(lang_index: u8) -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_SET_LANGUAGE, vec![lang_index])
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// Enable or disable automatic BLE reconnection.
pub fn set_auto_connect(enabled: bool) -> Packet {
    Packet::ext(
        ext_cmd::SPP_SECOND_SET_AUTO_CONNECT_CFG,
        vec![if enabled { 1 } else { 0 }],
    )
}

/// Query the current auto-connect setting.
pub fn get_auto_connect() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_SET_AUTO_CONNECT_CFG, vec![0xFF])
}

/// Enable or disable the save-volume-on-disconnect setting.
pub fn set_save_volume(enabled: bool) -> Packet {
    Packet::ext(
        ext_cmd::SPP_SECOND_SET_SAVE_VOLUME_CFG,
        vec![if enabled { 1 } else { 0 }],
    )
}

// ---------------------------------------------------------------------------
// Peripheral
// ---------------------------------------------------------------------------

/// Enable or disable peripheral device control.
pub fn set_peripheral_ctrl(enabled: bool) -> Packet {
    Packet::new(
        cmd::SPP_SET_PERIPHERALS_DEVICE_CTRL,
        vec![0x01, if enabled { 1 } else { 0 }],
    )
}

// ---------------------------------------------------------------------------
// Talk
// ---------------------------------------------------------------------------

/// Enable or disable the talk/intercom feature with a parameter byte.
pub fn set_talk(enabled: bool, param: u8) -> Packet {
    Packet::new(cmd::SPP_SET_TALK, vec![if enabled { 1 } else { 0 }, param])
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::cmd as c;
    use crate::protocol::ext_cmd as ec;

    fn assert_std(pkt: &Packet, expected_cmd: u8, expected_payload: &[u8]) {
        assert_eq!(pkt.command, expected_cmd);
        assert_eq!(pkt.payload, expected_payload);
    }

    fn assert_ext(pkt: &Packet, expected_ext: u8, expected_tail: &[u8]) {
        assert_eq!(pkt.command, c::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(pkt.payload[0], expected_ext);
        assert_eq!(&pkt.payload[1..], expected_tail);
    }

    // -- Time ----------------------------------------------------------------

    #[test]
    fn test_set_system_time() {
        let pkt = set_system_time(2026, 4, 13, 10, 30, 45, 0);
        assert_std(
            &pkt,
            c::SPP_SET_SYSTEM_TIME,
            &[26, 20, 4, 13, 10, 30, 45, 0],
        );
    }

    #[test]
    fn test_set_system_time_year_encoding() {
        // 2000 -> (0, 20), 1999 -> (99, 19)
        let pkt = set_system_time(2000, 1, 1, 0, 0, 0, 0);
        assert_eq!(pkt.payload[0], 0);
        assert_eq!(pkt.payload[1], 20);

        let pkt = set_system_time(1999, 12, 31, 23, 59, 59, 6);
        assert_eq!(pkt.payload[0], 99);
        assert_eq!(pkt.payload[1], 19);
    }

    #[test]
    fn test_set_time_now_returns_valid_packet() {
        let pkt = set_time_now();
        assert_eq!(pkt.command, c::SPP_SET_SYSTEM_TIME);
        assert_eq!(pkt.payload.len(), 8);
        // Weekday should be 0..=6
        assert!(pkt.payload[7] <= 6);
    }

    #[test]
    fn test_get_system_time() {
        assert_ext(&get_system_time(), ec::SPP_GET_SYSTEM_TIME, &[]);
    }

    // -- Brightness ----------------------------------------------------------

    #[test]
    fn test_set_brightness() {
        assert_std(&set_brightness(100), c::SPP_SET_SYSTEM_BRIGHT, &[100]);
    }

    #[test]
    fn test_get_brightness() {
        assert_std(&get_brightness(), c::SPP_LIGHT_CURRENT_LEVEL, &[]);
    }

    #[test]
    fn test_adjust_brightness() {
        assert_std(
            &adjust_brightness(&[10, 20]),
            c::SPP_LIGHT_ADJUST_LEVEL,
            &[10, 20],
        );
    }

    #[test]
    fn test_adjust_brightness_empty() {
        assert_std(&adjust_brightness(&[]), c::SPP_LIGHT_ADJUST_LEVEL, &[]);
    }

    // -- Device Name ---------------------------------------------------------

    #[test]
    fn test_set_device_name() {
        let pkt = set_device_name("Ditoo");
        assert_eq!(pkt.command, c::SPP_SET_DEVICE_NAME);
        assert_eq!(pkt.payload[0], 5); // length
        assert_eq!(&pkt.payload[1..], b"Ditoo");
    }

    #[test]
    fn test_set_device_name_empty() {
        let pkt = set_device_name("");
        assert_eq!(pkt.payload, vec![0]);
    }

    #[test]
    fn test_set_device_name_utf8() {
        let pkt = set_device_name("Hi!");
        assert_eq!(pkt.payload[0], 3);
        assert_eq!(&pkt.payload[1..], b"Hi!");
    }

    // -- Device Info ---------------------------------------------------------

    #[test]
    fn test_get_device_info() {
        assert_ext(&get_device_info(), ec::SPP_SECOND_GET_DEVICE_INFO, &[0x00]);
    }

    #[test]
    fn test_get_connected_flag() {
        assert_std(&get_connected_flag(), c::SPP_GET_CONNECTED_FLAG, &[]);
    }

    #[test]
    fn test_set_connected_flag() {
        assert_std(&set_connected_flag(1), c::SPP_SET_CONNECTED_FLAG, &[1]);
        assert_std(&set_connected_flag(0), c::SPP_SET_CONNECTED_FLAG, &[0]);
    }

    // -- Power ---------------------------------------------------------------

    #[test]
    fn test_set_power_on_off_enabled() {
        let pkt = set_power_on_off(2, true, 0xFF, 0x80, 0x00);
        assert_std(
            &pkt,
            c::SPP_SPP_POWER_ON_OFF_INFO,
            &[0x01, 2, 1, 0, 0, 0, 0, 0xFF, 0x80, 0x00],
        );
    }

    #[test]
    fn test_set_power_on_off_disabled() {
        let pkt = set_power_on_off(0, false, 0, 0, 0);
        assert_eq!(pkt.payload[2], 0);
    }

    #[test]
    fn test_get_power_on_off() {
        assert_std(&get_power_on_off(), c::SPP_SPP_POWER_ON_OFF_INFO, &[0x00]);
    }

    #[test]
    fn test_set_power_channel() {
        assert_std(
            &set_power_channel(3),
            c::SPP_SET_POWER_CHANNEL,
            &[0x01, 3],
        );
    }

    #[test]
    fn test_get_power_channel() {
        assert_std(&get_power_channel(), c::SPP_SET_POWER_CHANNEL, &[0x00]);
    }

    // -- Energy --------------------------------------------------------------

    #[test]
    fn test_set_energy_ctrl() {
        assert_std(&set_energy_ctrl(true), c::SPP_SET_ENERGY_CTRL, &[1]);
        assert_std(&set_energy_ctrl(false), c::SPP_SET_ENERGY_CTRL, &[0]);
    }

    #[test]
    fn test_get_energy_ctrl() {
        assert_std(&get_energy_ctrl(), c::SPP_GET_ENERGY_CTRL, &[]);
    }

    // -- Eye Guard -----------------------------------------------------------

    #[test]
    fn test_set_eye_guard() {
        assert_std(&set_eye_guard(true), c::SPP_EYE_GUARD_INFO, &[1]);
        assert_std(&set_eye_guard(false), c::SPP_EYE_GUARD_INFO, &[0]);
    }

    #[test]
    fn test_get_eye_guard() {
        assert_std(&get_eye_guard(), c::SPP_EYE_GUARD_INFO, &[0xFF]);
    }

    // -- Temperature ---------------------------------------------------------

    #[test]
    fn test_set_temp_type_celsius() {
        assert_std(&set_temp_type(true), c::SPP_SET_TEMP_TYPE, &[1]);
    }

    #[test]
    fn test_set_temp_type_fahrenheit() {
        assert_std(&set_temp_type(false), c::SPP_SET_TEMP_TYPE, &[0]);
    }

    #[test]
    fn test_get_temp_type() {
        assert_std(&get_temp_type(), c::SPP_SET_TEMP_TYPE, &[0xFF]);
    }

    #[test]
    fn test_get_device_temp() {
        assert_std(&get_device_temp(), c::SPP_GET_DEVICE_TEMP_INFO, &[]);
    }

    // -- Game ----------------------------------------------------------------

    #[test]
    fn test_game_control() {
        assert_std(&game_control(1), c::SPP_SEND_GAME_CTRL_INFO, &[1]);
    }

    #[test]
    fn test_game_key_up() {
        assert_std(&game_key_up(2), c::SPP_SEND_GAME_CTRL_KEY_UP_INFO, &[2]);
    }

    #[test]
    fn test_set_game() {
        assert_std(&set_game(true, 5), c::SPP_SET_GAME, &[1, 5]);
        assert_std(&set_game(false, 0), c::SPP_SET_GAME, &[0, 0]);
    }

    // -- Tool Info -----------------------------------------------------------

    #[test]
    fn test_get_tool_info() {
        assert_std(&get_tool_info(0), c::SPP_GET_TOOL_INFO, &[0]);
        assert_std(&get_tool_info(1), c::SPP_GET_TOOL_INFO, &[1]);
    }

    #[test]
    fn test_set_tool_countdown() {
        let pkt = set_tool_countdown(true, 5, 30);
        assert_std(
            &pkt,
            c::SPP_SET_TOOL_INFO,
            &[0x01, 1, 5, 0, 30, 0],
        );
    }

    #[test]
    fn test_set_tool_countdown_disabled() {
        let pkt = set_tool_countdown(false, 0, 0);
        assert_eq!(pkt.payload[1], 0);
    }

    #[test]
    fn test_set_tool_countdown_large_values() {
        let pkt = set_tool_countdown(true, 0x0102, 0x0304);
        assert_eq!(pkt.payload[2], 0x02); // minutes lo
        assert_eq!(pkt.payload[3], 0x01); // minutes hi
        assert_eq!(pkt.payload[4], 0x04); // seconds lo
        assert_eq!(pkt.payload[5], 0x03); // seconds hi
    }

    #[test]
    fn test_set_tool_noise() {
        assert_std(&set_tool_noise(1), c::SPP_SET_TOOL_INFO, &[0x02, 1]);
    }

    #[test]
    fn test_set_tool_scoreboard() {
        let pkt = set_tool_scoreboard(true, 10, 20);
        assert_std(&pkt, c::SPP_SET_TOOL_INFO, &[0x03, 1, 10, 20]);
    }

    #[test]
    fn test_set_tool_scoreboard_disabled() {
        let pkt = set_tool_scoreboard(false, 0, 0);
        assert_eq!(pkt.payload[1], 0);
    }

    // -- Language ------------------------------------------------------------

    #[test]
    fn test_set_language() {
        assert_ext(&set_language(0), ec::SPP_SECOND_SET_LANGUAGE, &[0]);
        assert_ext(&set_language(3), ec::SPP_SECOND_SET_LANGUAGE, &[3]);
    }

    // -- Config --------------------------------------------------------------

    #[test]
    fn test_set_auto_connect() {
        assert_ext(
            &set_auto_connect(true),
            ec::SPP_SECOND_SET_AUTO_CONNECT_CFG,
            &[1],
        );
        assert_ext(
            &set_auto_connect(false),
            ec::SPP_SECOND_SET_AUTO_CONNECT_CFG,
            &[0],
        );
    }

    #[test]
    fn test_get_auto_connect() {
        assert_ext(
            &get_auto_connect(),
            ec::SPP_SECOND_SET_AUTO_CONNECT_CFG,
            &[0xFF],
        );
    }

    #[test]
    fn test_set_save_volume() {
        assert_ext(
            &set_save_volume(true),
            ec::SPP_SECOND_SET_SAVE_VOLUME_CFG,
            &[1],
        );
        assert_ext(
            &set_save_volume(false),
            ec::SPP_SECOND_SET_SAVE_VOLUME_CFG,
            &[0],
        );
    }

    // -- Peripheral ----------------------------------------------------------

    #[test]
    fn test_set_peripheral_ctrl() {
        assert_std(
            &set_peripheral_ctrl(true),
            c::SPP_SET_PERIPHERALS_DEVICE_CTRL,
            &[0x01, 1],
        );
        assert_std(
            &set_peripheral_ctrl(false),
            c::SPP_SET_PERIPHERALS_DEVICE_CTRL,
            &[0x01, 0],
        );
    }

    // -- Talk ----------------------------------------------------------------

    #[test]
    fn test_set_talk() {
        assert_std(&set_talk(true, 3), c::SPP_SET_TALK, &[1, 3]);
        assert_std(&set_talk(false, 0), c::SPP_SET_TALK, &[0, 0]);
    }

    // -- Wire encoding round-trips -------------------------------------------

    #[test]
    fn test_encode_framing() {
        let packets = [
            set_brightness(100),
            get_brightness(),
            set_system_time(2026, 4, 13, 10, 30, 45, 0),
            get_system_time(),
            set_power_on_off(1, true, 0xFF, 0x80, 0x00),
            set_device_name("Test"),
            set_game(true, 1),
            set_tool_countdown(true, 10, 30),
            set_language(0),
            set_auto_connect(true),
        ];
        for pkt in &packets {
            let wire = pkt.encode();
            assert_eq!(wire[0], 0x01, "start marker");
            assert_eq!(*wire.last().unwrap(), 0x02, "end marker");
            for &b in &wire[1..wire.len() - 1] {
                assert!(b != 0x01 && b != 0x02, "unescaped marker in payload");
            }
        }
    }
}
