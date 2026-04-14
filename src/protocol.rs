/// Divoom BLE binary protocol implementation.
///
/// Packet format (NewMode):
///   0x01 | length_lo length_hi | cmd_id | payload... | checksum_lo checksum_hi | 0x02
///
/// Escape sequences applied after framing:
///   0x01 in payload -> 0x03 0x04
///   0x02 in payload -> 0x03 0x05
///   0x03 in payload -> 0x03 0x06

pub mod cmd {
    // Main command IDs from SppProc$CMD_TYPE (decompiled values)
    pub const SPP_JSON: u8 = 0;
    pub const SPP_COMMAND_CHECK: u8 = 1;
    pub const SPP_SET_SD_PLAY_MUSIC_ID: u8 = 4;
    pub const SPP_CHANGE_MODE: u8 = 5;
    pub const SPP_SET_LAST_NEXT: u8 = 5;
    pub const SPP_GET_SD_MUSIC_LIST: u8 = 7;
    pub const SPP_APP_EQ_GIF: u8 = 8;
    pub const SPP_GET_VOL: u8 = 9;
    pub const SPP_GET_PLAY_STATUS: u8 = 11;
    pub const SPP_GET_STDB_MODE: u8 = 18;
    pub const SPP_SET_POWERON_VOICE_CTRL: u8 = 19;
    pub const SPP_LIEGHT_SET_25DOTS_PIC: u8 = 20;
    pub const SPP_SET_GIF_SPEED_CMD: u8 = 22;
    pub const SPP_SET_SYSTEM_TIME: u8 = 23;
    pub const SPP_STOP_SEND_GIF: u8 = 25;
    pub const SPP_LIEGHT_SET_25DOTS_ATTR: u8 = 30;
    pub const SPP_SPP_POWER_ON_OFF_INFO: u8 = 31;
    pub const SPP_SEND_GAME_CTRL_INFO: u8 = 32;
    pub const SPP_SEND_GAME_CTRL_KEY_UP_INFO: u8 = 33;
    pub const SPP_EYE_GUARD_INFO: u8 = 34;
    pub const SPP_SPP_LIGHT_ARROW_SWITCH: u8 = 35;
    pub const SPP_SYSTEM_TRY_COLOR: u8 = 36;
    pub const SPP_SEND_APP_NEW_PIC_INFO: u8 = 37;
    pub const SPP_SEND_APP_NEWEST_TIME: u8 = 38;
    pub const SPP_SET_APP_BR_PASSWORD: u8 = 39;
    pub const SPP_SET_TEMP_TYPE: u8 = 43;
    pub const SPP_SET_BOX_MODE: u8 = 69;
    pub const SPP_SET_HPUR_TYPE: u8 = 45;
    pub const SPP_SET_PLAY_STATUS: u8 = 47;
    pub const SPP_SET_VOL: u8 = 49;
    pub const SPP_LIGHT_CURRENT_LEVEL: u8 = 49;
    pub const SPP_LIGHT_ADJUST_LEVEL: u8 = 50;
    pub const SPP_SAND_PAINT_CTRL: u8 = 52;
    pub const SPP_SCROLL: u8 = 53;
    pub const SPP_GET_FILE_VERSION2: u8 = 54;
    pub const SPP_GET_FILE_VERSION2_LIST: u8 = 55;
    pub const SPP_DRAWING_MUL_PAD_CTRL: u8 = 58;
    pub const SPP_DRAWING_BIG_PAD_CTRL: u8 = 59;
    pub const SPP_SET_ANCS_NOTICE_PIC: u8 = 60;
    pub const SPP_MOVE_RESET_IFRAME: u8 = 62;
    pub const SPP_SET_SLEEP_TIME: u8 = 64;
    pub const SPP_SET_SLEEP_SCENE: u8 = 65;
    pub const SPP_GET_ALARM_TIME_SCENE: u8 = 66;
    pub const SPP_SET_ALARM_TIME_SCENE: u8 = 67;
    pub const SPP_SET_BOX_COLOR: u8 = 68;
    pub const SPP_GET_BOX_MODE: u8 = 70;
    pub const SPP_APP_NEED_GET_MUSIC_LIST: u8 = 71;
    pub const SPP_PAUSE_SYS_UPDATE_DATA: u8 = 72;
    pub const SPP_SET_MUL_BOX_COLOR: u8 = 73;
    pub const SPP_SET_ANDROID_ANCS: u8 = 80;
    pub const SPP_SET_ALARM_TIME_GIF: u8 = 81;
    pub const SPP_SET_BOOT_GIF: u8 = 82;
    pub const SPP_GET_DIALY_TIME_EXT2: u8 = 83;
    pub const SPP_SET_DIALY_TIME_EXT2: u8 = 84;
    pub const SPP_SET_DIALY_TIME_GIF: u8 = 85;
    pub const SPP_SET_TIME_MANAGE_INFO: u8 = 86;
    pub const SPP_GET_TIME_MANAGE_CTRL: u8 = 87;
    pub const SPP_DRAWING_PAD_CTRL: u8 = 88;
    pub const SPP_GET_DEVICE_TEMP_INFO: u8 = 89;
    pub const SPP_DRAWING_PAD_EXIT: u8 = 90;
    pub const SPP_DRAWING_ENCODE_PIC: u8 = 91;
    pub const SPP_DRAWING_ENCODE_PLAY: u8 = 92;
    pub const SPP_SEND_NET_TEMP_INFO: u8 = 93;
    pub const SPP_SEND_NET_TEMP_DISP_INFO: u8 = 94;
    pub const SPP_SEND_CUR_NET_TEMP: u8 = 95;
    pub const SPP_GET_FM_CURRENT_FREQ: u8 = 96;
    pub const SPP_SET_FM_CURRENT_FREQ: u8 = 97;
    pub const SPP_SET_FM_AUTOMATIC_SEARCH: u8 = 99;
    pub const SPP_GET_FM_COUNT_OR_FREQ: u8 = 100;
    pub const SPP_SET_FM_FAVOURITE: u8 = 103;
    pub const SPP_GET_FM_AUTOMATIC_SEARCH_STATUS: u8 = 104;
    pub const SPP_SET_FM_REGION: u8 = 105;
    pub const SPP_SET_MIX_MUISE_MODE: u8 = 106;
    pub const SPP_DRAWING_MUL_ENCODE_GIF_PLAY: u8 = 107;
    pub const SPP_DRAWING_ENCODE_MOVIE_PLAY: u8 = 108;
    pub const SPP_DRAWING_MUL_ENCODE_MOVIE_PLAY: u8 = 109;
    pub const SPP_DRAWING_CTRL_MOVIE_PLAY: u8 = 110;
    pub const SPP_DRAWING_MUL_PAD_ENTER: u8 = 111;
    pub const SPP_GET_FM_REGION: u8 = 112;
    pub const SPP_GET_TOOL_INFO: u8 = 113;
    pub const SPP_SET_TOOL_INFO: u8 = 114;
    pub const SPP_GET_NET_TEMP_DISP_INFO: u8 = 115;
    pub const SPP_SET_SYSTEM_BRIGHT: u8 = 116;
    pub const SPP_SET_DEVICE_NAME: u8 = 117;
    pub const SPP_SET_MUL_DEVICE_CTRL: u8 = 119;
    pub const SPP_LED_UPDATE_FONT_INFO: u8 = 124;
    pub const SPP_SET_DIVOOM_LEAVE_MSG_GIF: u8 = 126;
    pub const SPP_DEL_LEAVE_MSG_GIF: u8 = 127;
    pub const SPP_SET_ALARM_VOICE_CTRL: u8 = 130;
    pub const SPP_SET_SONG_DIS_CTRL: u8 = 131;
    pub const SPP_RESET_NOTIFICATIONS: u8 = 132;
    pub const SPP_SEND_HOTCTRL: u8 = 133;
    pub const SPP_SEND_LED_WORD_CMD: u8 = 134;
    pub const SPP_LIEGHT_PHONE_GIF32_WORD_ATTR: u8 = 135;
    pub const SPP_SEND_GAME_SHARK: u8 = 136;
    pub const SPP_SET_PERIPHERALS_DEVICE_CTRL: u8 = 137;
    pub const SPP_SET_POWER_CHANNEL: u8 = 138;
    pub const SPP_APP_NEW_GIF_CMD2020: u8 = 139;
    pub const SPP_APP_NEW_USER_DEFINE2020: u8 = 140;
    pub const SPP_APP_BIG64_USER_DEFINE: u8 = 141;
    pub const SPP_APP_GET_USER_DEFINE_INFO: u8 = 142;
    pub const SPP_LOCAL_PICTURE: u8 = 143;
    pub const SPP_SYS_DEVICE_UPDATE: u8 = 147;
    pub const SPP_SYS_UPDATE_DATA: u8 = 148;
    pub const SPP_SYS_CONTINUE_UPDATE_DATA: u8 = 149;
    pub const SPP_SYS_GET_UPDATE_ADDR: u8 = 150;
    pub const SPP_GET_FILE_VERSION: u8 = 151;
    pub const SPP_APP_SEND_FILE_DATA: u8 = 153;
    pub const SPP_SEND_HOT_FILE_LIST: u8 = 155;
    pub const SPP_HOT_UPDATE_FILE_INFO: u8 = 157;
    pub const SPP_HOT_SEND_FILE_DATA: u8 = 158;
    pub const SPP_HOT_PAUSE_FILE_SEND: u8 = 159;
    pub const SPP_SET_GAME: u8 = 160;
    pub const SPP_SET_TALK: u8 = 161;
    pub const SPP_GET_SCENE: u8 = 162;
    pub const SPP_SET_SCENE_LISTEN: u8 = 163;
    pub const SPP_SET_SCENE_LISTEN_VOLUME: u8 = 164;
    pub const SPP_SET_ALARM_LISTEN: u8 = 165;
    pub const SPP_SET_ALARM_LISTEN_VOLUME: u8 = 166;
    pub const SPP_SET_SOUND_CTRL: u8 = 167;
    pub const SPP_GET_SOUND_CTRL: u8 = 168;
    pub const SPP_SET_PLAY_STOP_VOICE: u8 = 169;
    pub const SPP_GET_PLAY_VOICE_STATUS: u8 = 170;
    pub const SPP_SET_AUTO_POWER_OFF: u8 = 171;
    pub const SPP_GET_AUTO_POWER_OFF: u8 = 172;
    pub const SPP_SET_SLEEP_COLOR: u8 = 173;
    pub const SPP_SET_SLEEP_LIGHT: u8 = 174;
    pub const SPP_SET_CONNECTED_FLAG: u8 = 175;
    pub const SPP_GET_CONNECTED_FLAG: u8 = 176;
    pub const SPP_SET_USER_GIF: u8 = 177;
    pub const SPP_SET_ENERGY_CTRL: u8 = 178;
    pub const SPP_GET_ENERGY_CTRL: u8 = 179;
    pub const SPP_GET_SD_MUSIC_INFO: u8 = 180;
    pub const SPP_MODIFY_RHYTHM_ITEMS: u8 = 182;
    pub const SPP_SET_RHYTHM_GIF: u8 = 183;
    pub const SPP_SET_SD_MUSIC_POSITION: u8 = 184;
    pub const SPP_SET_SD_MUSIC_PLAY_MODE: u8 = 185;
    pub const SPP_SET_POWERON_VOICE_VOL: u8 = 187;
    pub const SPP_SET_NEW_MIX_MUSIC_MODE: u8 = 188;
    pub const SPP_DIVOOM_EXTERN_CMD: u8 = 189;
    pub const SPP_SECOND_SEND_SERVER_FILE_INFO: u8 = 190;
    pub const SPP_SET_SPP_SET_MIC_SWITCH: u8 = 32;
    pub const SPP_WIFI_SERVER_TYPE_COMMAND: u8 = 244;
    pub const START_BOND_BLE_COMMAND: u8 = 240;
    pub const EXIT_BOND_BLE_COMMAND: u8 = 241;
    pub const WIFI_CONFIG_BLE_COMMAND: u8 = 242;
    pub const WIFI_STATUS_BLE_COMMAND: u8 = 243;
    pub const SPP_REQUEST_NEW_FILE_INFO: u8 = 247;
}

pub mod ext_cmd {
    // Extended command IDs from SppProc$EXT_CMD_TYPE
    // These are sent as payload[0] inside SPP_DIVOOM_EXTERN_CMD
    pub const SPP_EXT_CAR_MODE: u8 = 16;
    pub const SPP_SECOND_SET_KEY_PIC: u8 = 17;
    pub const SPP_SECOND_SET_KEY_FUNC: u8 = 18;
    pub const SPP_SECOND_SET_GIF_TYPE: u8 = 19;
    pub const SPP_SECOND_SET_USER_DEFINE_TIME: u8 = 20;
    pub const SPP_SECOND_GET_USER_DEFINE_TIME: u8 = 22;
    pub const SPP_SECOND_CLEAR_USER_DEFINE_INDEX: u8 = 22;
    pub const SPP_SECOND_USE_USER_DEFINE_INDEX: u8 = 23;
    pub const SPP_SECOND_GET_NEW_POWER_ON_CHANNEL: u8 = 24;
    pub const SPP_SECOND_SET_SAVE_VOLUME_CFG: u8 = 25;
    pub const SPP_SECOND_SET_AUTO_CONNECT_CFG: u8 = 26;
    pub const SPP_SECOND_SET_GIF_PLAY_TIME_CFG: u8 = 27;
    pub const SPP_SECOND_SET_MUSIC_NAME_CFG: u8 = 28;
    pub const SPP_SECOND_RECORD_CTRL: u8 = 29;
    pub const SPP_SECOND_KARAOKE_CTRL: u8 = 30;
    pub const SPP_SECOND_WIRELESS_MIC_CTRL: u8 = 32;
    pub const SPP_SECOND_TOUCH: u8 = 33;
    pub const SPP_SECOND_SET_SCREEN_DIR_CFG: u8 = 37;
    pub const SPP_SECOND_SET_SCREEN_MIRROR_CFG: u8 = 38;
    pub const SPP_SECOND_SET_LANGUAGE: u8 = 39;
    pub const SPP_SECOND_GET_DECODE_FILE_INFO: u8 = 40;
    pub const SPP_SECOND_GET_NORMAL_FILE_INFO: u8 = 41;
    pub const SPP_SECOND_GET_CLOCK_ID_INFO: u8 = 42;
    pub const SPP_SECOND_GET_DEVICE_INFO: u8 = 43;
    pub const SPP_SECOND_DEVICE_FORMAT_FLAG: u8 = 44;
    pub const SPP_SECOND_GET_FONT_INFO: u8 = 45;
    pub const SPP_GET_SYSTEM_TIME: u8 = 46;
    pub const SPP_SECOND_OPEN_SCREEN_CTRL: u8 = 47;
    pub const SPP_SECOND_GET_ZSTD_DECODE_FILE_INFO: u8 = 48;
    pub const SPP_SECOND_APP_SEND_GIF_START: u8 = 49;
    pub const SPP_SECOND_SEND_DEVICE_EQUALIZER_CTRL: u8 = 50;
    pub const SPP_SECOND_SEND_DEVICE_LIGHT_EFFECT_CTRL: u8 = 51;
    pub const SPP_SECOND_APP_SET_VOL: u8 = 52;
    pub const SPP_SECOND_APP_GET_VOL: u8 = 53;
    pub const SPP_SECOND_APP_SET_MUSIC_PLAY_PAUSE: u8 = 54;
    pub const SPP_SECOND_GET_EZIP_DECODE_FILE_INFO: u8 = 55;
    pub const SPP_SECOND_GET_EZIP_ROTATE_DECODE_FILE_INFO: u8 = 56;
    pub const SPP_SECOND_GET_JPEG_PIXEL_FILE_INFO: u8 = 58;
}

pub struct Packet {
    pub command: u8,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(command: u8, payload: Vec<u8>) -> Self {
        Self { command, payload }
    }

    /// Build an extended command packet (SPP_DIVOOM_EXTERN_CMD wrapper).
    pub fn ext(ext_cmd_id: u8, payload: Vec<u8>) -> Self {
        let mut full_payload = Vec::with_capacity(1 + payload.len());
        full_payload.push(ext_cmd_id);
        full_payload.extend_from_slice(&payload);
        Self {
            command: cmd::SPP_DIVOOM_EXTERN_CMD,
            payload: full_payload,
        }
    }

    /// Encode into the wire format with framing, length, and checksum.
    pub fn encode(&self) -> Vec<u8> {
        // length field = cmd(1) + payload(N) + checksum(2) = payload.len() + 3
        let data_len = self.payload.len() + 3;
        let len_lo = (data_len & 0xFF) as u8;
        let len_hi = ((data_len >> 8) & 0xFF) as u8;

        // Checksum: sum of len_lo + len_hi + cmd + payload bytes
        let mut checksum: u16 = 0;
        checksum = checksum.wrapping_add(len_lo as u16);
        checksum = checksum.wrapping_add(len_hi as u16);
        checksum = checksum.wrapping_add(self.command as u16);
        for &b in &self.payload {
            checksum = checksum.wrapping_add(b as u16);
        }
        let cksum_lo = (checksum & 0xFF) as u8;
        let cksum_hi = ((checksum >> 8) & 0xFF) as u8;

        // Build raw inner bytes (between start and end markers)
        let mut inner = Vec::with_capacity(self.payload.len() + 5);
        inner.push(len_lo);
        inner.push(len_hi);
        inner.push(self.command);
        inner.extend_from_slice(&self.payload);
        inner.push(cksum_lo);
        inner.push(cksum_hi);

        // Apply escape sequences
        let mut out = Vec::with_capacity(inner.len() + 10);
        out.push(0x01);
        for &b in &inner {
            match b {
                0x01 => { out.push(0x03); out.push(0x04); }
                0x02 => { out.push(0x03); out.push(0x05); }
                0x03 => { out.push(0x03); out.push(0x06); }
                _ => out.push(b),
            }
        }
        out.push(0x02);
        out
    }
}

/// Set the device clock to the current system time.
pub fn set_time_now() -> Packet {
    let now = chrono::Local::now();
    let payload = vec![
        (now.format("%y").to_string().parse::<u8>().unwrap_or(0)),
        now.format("%m").to_string().parse::<u8>().unwrap_or(0),
        now.format("%d").to_string().parse::<u8>().unwrap_or(0),
        now.format("%H").to_string().parse::<u8>().unwrap_or(0),
        now.format("%M").to_string().parse::<u8>().unwrap_or(0),
        now.format("%S").to_string().parse::<u8>().unwrap_or(0),
    ];
    Packet::new(cmd::SPP_SET_SYSTEM_TIME, payload)
}

/// Set the display brightness (0-100).
pub fn set_brightness(level: u8) -> Packet {
    Packet::new(cmd::SPP_SET_SYSTEM_BRIGHT, vec![level])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_encoding_basic() {
        let pkt = Packet::new(cmd::SPP_SET_SYSTEM_BRIGHT, vec![50]);
        let encoded = pkt.encode();
        assert_eq!(encoded[0], 0x01);
        assert_eq!(*encoded.last().unwrap(), 0x02);
        assert!(encoded.len() > 4);
    }

    #[test]
    fn test_packet_checksum() {
        let pkt = Packet::new(0x10, vec![0x20, 0x30]);
        let encoded = pkt.encode();
        assert_eq!(encoded[0], 0x01);
        assert_eq!(*encoded.last().unwrap(), 0x02);
        // checksum = 5 + 0 + 0x10 + 0x20 + 0x30 = 101 = 0x65
        assert_eq!(
            encoded,
            vec![0x01, 0x05, 0x00, 0x10, 0x20, 0x30, 0x65, 0x00, 0x02]
        );
    }

    #[test]
    fn test_escape_sequences() {
        let pkt = Packet::new(0x01, vec![0x02, 0x03]);
        let encoded = pkt.encode();
        assert_eq!(encoded[0], 0x01);
        assert!(!encoded[1..encoded.len() - 1]
            .windows(1)
            .any(|w| w[0] == 0x01 || w[0] == 0x02));
        assert_eq!(*encoded.last().unwrap(), 0x02);
    }

    #[test]
    fn test_ext_command() {
        let pkt = Packet::ext(0x34, vec![0x50]);
        assert_eq!(pkt.command, cmd::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(pkt.payload, vec![0x34, 0x50]);
    }
}
