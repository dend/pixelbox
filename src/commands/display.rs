//! Display and light mode commands for the Divoom Ditoo Pro.

use crate::protocol::{Packet, cmd, ext_cmd};

/// Display light modes selectable via [`set_box_mode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LightMode {
    Clock = 0,
    Temperature = 1,
    ColorLight = 2,
    SpecialLight = 3,
    SoundLight = 4,
    SoundUser = 5,
    Music = 6,
}

/// Device work/input modes selectable via [`change_mode`].
///
/// The Ditoo Pro does not have FM radio hardware, so `Fm` is excluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkMode {
    Bluetooth = 0,
    LineIn = 2,
    Sd = 3,
    Uac = 7,
}

/// Set the box display mode with optional parameters.
///
/// The payload is `[light_mode, ...params]` padded to 9 bytes total.
pub fn set_box_mode(light_mode: u8, params: &[u8]) -> Packet {
    let mut payload = Vec::with_capacity(9);
    payload.push(light_mode);
    payload.extend_from_slice(params);
    payload.resize(9, 0);
    Packet::new(cmd::SPP_SET_BOX_MODE, payload)
}

/// Query the current box display mode.
pub fn get_box_mode() -> Packet {
    Packet::new(cmd::SPP_GET_BOX_MODE, vec![])
}

/// Switch to the clock face.
pub fn set_box_mode_clock() -> Packet {
    set_box_mode(LightMode::Clock as u8, &[])
}

/// Switch to the temperature display.
pub fn set_box_mode_temperature() -> Packet {
    set_box_mode(LightMode::Temperature as u8, &[])
}

/// Switch to a solid/static colour light with adjustable brightness and speed toggle.
///
/// Payload: `[2, brightness, r, g, b, 0, speed_flag]` padded to 10 bytes.
pub fn set_box_mode_color_light(r: u8, g: u8, b: u8, brightness: u8, speed: bool) -> Packet {
    let speed_flag = if speed { 1u8 } else { 0u8 };
    let mut payload = vec![
        LightMode::ColorLight as u8,
        brightness,
        r,
        g,
        b,
        0,
        speed_flag,
    ];
    payload.resize(10, 0);
    Packet::new(cmd::SPP_SET_BOX_MODE, payload)
}

/// Switch to a built-in special light effect.
pub fn set_box_mode_special(effect_id: u8) -> Packet {
    set_box_mode(LightMode::SpecialLight as u8, &[effect_id])
}

/// Switch to a sound-reactive visualiser mode.
pub fn set_box_mode_sound_reactive(mode: u8) -> Packet {
    Packet::new(
        cmd::SPP_SET_BOX_MODE,
        vec![LightMode::SoundLight as u8, mode, 0, 0, 0, 0],
    )
}

/// Switch to a music visualiser mode.
pub fn set_box_mode_music(mode: u8) -> Packet {
    Packet::new(
        cmd::SPP_SET_BOX_MODE,
        vec![LightMode::Music as u8, mode, 0, 0, 0, 0],
    )
}

/// Query the current scene/animation.
pub fn get_scene() -> Packet {
    Packet::new(cmd::SPP_GET_SCENE, vec![])
}

/// Preview a colour on the display without persisting it.
pub fn try_color(r: u8, g: u8, b: u8) -> Packet {
    Packet::new(cmd::SPP_SYSTEM_TRY_COLOR, vec![r, g, b])
}

/// Set the screen rotation direction (extended command).
pub fn set_screen_direction(direction: u8) -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_SET_SCREEN_DIR_CFG, vec![direction])
}

/// Query the current screen direction (extended command).
///
/// The device replies with the active direction when it receives `0xFF`.
pub fn get_screen_direction() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_SET_SCREEN_DIR_CFG, vec![0xFF])
}

/// Enable or disable screen mirroring (extended command).
pub fn set_screen_mirror(enabled: bool) -> Packet {
    Packet::ext(
        ext_cmd::SPP_SECOND_SET_SCREEN_MIRROR_CFG,
        vec![if enabled { 1 } else { 0 }],
    )
}

/// Turn the screen on or off (extended command).
pub fn set_screen_on(on: bool) -> Packet {
    Packet::ext(
        ext_cmd::SPP_SECOND_OPEN_SCREEN_CTRL,
        vec![if on { 1 } else { 0 }],
    )
}

/// Apply a custom light effect described by seven parameter bytes (extended command).
///
/// The packet payload is `[0x01, params[0..7]]`.
pub fn set_light_effect(params: &[u8; 7]) -> Packet {
    let mut payload = Vec::with_capacity(8);
    payload.push(0x01);
    payload.extend_from_slice(params);
    Packet::ext(ext_cmd::SPP_SECOND_SEND_DEVICE_LIGHT_EFFECT_CTRL, payload)
}

/// Reset the light effect to the device default (extended command).
pub fn reset_light_effect() -> Packet {
    Packet::ext(
        ext_cmd::SPP_SECOND_SEND_DEVICE_LIGHT_EFFECT_CTRL,
        vec![0x00],
    )
}

/// Switch the light arrow indicator mode.
pub fn set_light_arrow(mode: u8) -> Packet {
    Packet::new(cmd::SPP_SPP_LIGHT_ARROW_SWITCH, vec![mode])
}

/// Change the device work/input mode (Bluetooth, FM, line-in, etc.).
pub fn change_mode(mode: WorkMode) -> Packet {
    Packet::new(cmd::SPP_CHANGE_MODE, vec![mode as u8])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_box_mode_padding() {
        let pkt = set_box_mode(0, &[1, 2]);
        assert_eq!(pkt.command, cmd::SPP_SET_BOX_MODE);
        assert_eq!(pkt.payload.len(), 9);
        assert_eq!(pkt.payload[0], 0); // light_mode
        assert_eq!(pkt.payload[1], 1);
        assert_eq!(pkt.payload[2], 2);
        // remaining bytes are zero-padded
        assert!(pkt.payload[3..].iter().all(|&b| b == 0));
    }

    #[test]
    fn test_set_box_mode_truncates_nothing_when_exact() {
        let params = [10, 20, 30, 40, 50, 60, 70, 80];
        let pkt = set_box_mode(3, &params);
        assert_eq!(pkt.payload.len(), 9);
        assert_eq!(pkt.payload[0], 3);
        assert_eq!(&pkt.payload[1..], &params);
    }

    #[test]
    fn test_get_box_mode() {
        let pkt = get_box_mode();
        assert_eq!(pkt.command, cmd::SPP_GET_BOX_MODE);
        assert!(pkt.payload.is_empty());
    }

    #[test]
    fn test_set_box_mode_clock() {
        let pkt = set_box_mode_clock();
        assert_eq!(pkt.command, cmd::SPP_SET_BOX_MODE);
        assert_eq!(pkt.payload[0], LightMode::Clock as u8);
    }

    #[test]
    fn test_set_box_mode_temperature() {
        let pkt = set_box_mode_temperature();
        assert_eq!(pkt.command, cmd::SPP_SET_BOX_MODE);
        assert_eq!(pkt.payload[0], LightMode::Temperature as u8);
    }

    #[test]
    fn test_set_box_mode_color_light() {
        let pkt = set_box_mode_color_light(0xFF, 0x80, 0x00, 75, true);
        assert_eq!(pkt.command, cmd::SPP_SET_BOX_MODE);
        assert_eq!(pkt.payload.len(), 10);
        assert_eq!(pkt.payload[0], 2); // ColorLight
        assert_eq!(pkt.payload[1], 75); // brightness
        assert_eq!(pkt.payload[2], 0xFF); // r
        assert_eq!(pkt.payload[3], 0x80); // g
        assert_eq!(pkt.payload[4], 0x00); // b
        assert_eq!(pkt.payload[5], 0); // reserved
        assert_eq!(pkt.payload[6], 1); // speed = true
    }

    #[test]
    fn test_set_box_mode_color_light_no_speed() {
        let pkt = set_box_mode_color_light(10, 20, 30, 50, false);
        assert_eq!(pkt.payload[6], 0); // speed = false
    }

    #[test]
    fn test_set_box_mode_special() {
        let pkt = set_box_mode_special(5);
        assert_eq!(pkt.command, cmd::SPP_SET_BOX_MODE);
        assert_eq!(pkt.payload[0], LightMode::SpecialLight as u8);
        assert_eq!(pkt.payload[1], 5);
    }

    #[test]
    fn test_set_box_mode_sound_reactive() {
        let pkt = set_box_mode_sound_reactive(2);
        assert_eq!(pkt.command, cmd::SPP_SET_BOX_MODE);
        assert_eq!(pkt.payload, vec![4, 2, 0, 0, 0, 0]);
    }

    #[test]
    fn test_set_box_mode_music() {
        let pkt = set_box_mode_music(3);
        assert_eq!(pkt.command, cmd::SPP_SET_BOX_MODE);
        assert_eq!(pkt.payload, vec![6, 3, 0, 0, 0, 0]);
    }

    #[test]
    fn test_get_scene() {
        let pkt = get_scene();
        assert_eq!(pkt.command, cmd::SPP_GET_SCENE);
        assert!(pkt.payload.is_empty());
    }

    #[test]
    fn test_try_color() {
        let pkt = try_color(0xAA, 0xBB, 0xCC);
        assert_eq!(pkt.command, cmd::SPP_SYSTEM_TRY_COLOR);
        assert_eq!(pkt.payload, vec![0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_set_screen_direction() {
        let pkt = set_screen_direction(2);
        // Extended commands are wrapped in SPP_DIVOOM_EXTERN_CMD
        assert_eq!(pkt.command, cmd::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(pkt.payload, vec![ext_cmd::SPP_SECOND_SET_SCREEN_DIR_CFG, 2]);
    }

    #[test]
    fn test_get_screen_direction() {
        let pkt = get_screen_direction();
        assert_eq!(pkt.command, cmd::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(
            pkt.payload,
            vec![ext_cmd::SPP_SECOND_SET_SCREEN_DIR_CFG, 0xFF]
        );
    }

    #[test]
    fn test_set_screen_mirror() {
        let on = set_screen_mirror(true);
        let off = set_screen_mirror(false);
        assert_eq!(on.payload[1], 1);
        assert_eq!(off.payload[1], 0);
        assert_eq!(on.payload[0], ext_cmd::SPP_SECOND_SET_SCREEN_MIRROR_CFG);
    }

    #[test]
    fn test_set_screen_on() {
        let on = set_screen_on(true);
        let off = set_screen_on(false);
        assert_eq!(on.payload, vec![ext_cmd::SPP_SECOND_OPEN_SCREEN_CTRL, 1]);
        assert_eq!(off.payload, vec![ext_cmd::SPP_SECOND_OPEN_SCREEN_CTRL, 0]);
    }

    #[test]
    fn test_set_light_effect() {
        let params: [u8; 7] = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70];
        let pkt = set_light_effect(&params);
        assert_eq!(pkt.command, cmd::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(
            pkt.payload[0],
            ext_cmd::SPP_SECOND_SEND_DEVICE_LIGHT_EFFECT_CTRL
        );
        assert_eq!(pkt.payload[1], 0x01);
        assert_eq!(&pkt.payload[2..], &params);
    }

    #[test]
    fn test_reset_light_effect() {
        let pkt = reset_light_effect();
        assert_eq!(pkt.command, cmd::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(
            pkt.payload,
            vec![ext_cmd::SPP_SECOND_SEND_DEVICE_LIGHT_EFFECT_CTRL, 0x00]
        );
    }

    #[test]
    fn test_set_light_arrow() {
        let pkt = set_light_arrow(1);
        assert_eq!(pkt.command, cmd::SPP_SPP_LIGHT_ARROW_SWITCH);
        assert_eq!(pkt.payload, vec![1]);
    }

    #[test]
    fn test_change_mode() {
        let pkt = change_mode(WorkMode::Bluetooth);
        assert_eq!(pkt.command, cmd::SPP_CHANGE_MODE);
        assert_eq!(pkt.payload, vec![0]);

        let pkt = change_mode(WorkMode::Uac);
        assert_eq!(pkt.payload, vec![7]);
    }

    #[test]
    fn test_light_mode_repr() {
        assert_eq!(LightMode::Clock as u8, 0);
        assert_eq!(LightMode::Temperature as u8, 1);
        assert_eq!(LightMode::ColorLight as u8, 2);
        assert_eq!(LightMode::SpecialLight as u8, 3);
        assert_eq!(LightMode::SoundLight as u8, 4);
        assert_eq!(LightMode::SoundUser as u8, 5);
        assert_eq!(LightMode::Music as u8, 6);
    }

    #[test]
    fn test_work_mode_repr() {
        assert_eq!(WorkMode::Bluetooth as u8, 0);
        assert_eq!(WorkMode::LineIn as u8, 2);
        assert_eq!(WorkMode::Sd as u8, 3);
        assert_eq!(WorkMode::Uac as u8, 7);
    }

    #[test]
    fn test_color_light_encodes_correctly() {
        // Verify the full wire encoding round-trips without panic
        let pkt = set_box_mode_color_light(0x01, 0x02, 0x03, 100, false);
        let encoded = pkt.encode();
        assert_eq!(encoded[0], 0x01); // start marker
        assert_eq!(*encoded.last().unwrap(), 0x02); // end marker
    }

    #[test]
    fn test_set_box_mode_empty_params() {
        let pkt = set_box_mode(0, &[]);
        assert_eq!(pkt.payload.len(), 9);
        assert_eq!(pkt.payload[0], 0);
        assert!(pkt.payload[1..].iter().all(|&b| b == 0));
    }
}
