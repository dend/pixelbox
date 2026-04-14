use crate::protocol::{cmd, ext_cmd, Packet};

// ---------------------------------------------------------------------------
// Volume
// ---------------------------------------------------------------------------

pub fn set_volume(level: u8) -> Packet {
    Packet::new(cmd::SPP_SET_VOL, vec![level])
}

pub fn get_volume() -> Packet {
    Packet::new(cmd::SPP_GET_VOL, vec![])
}

pub fn set_volume_ext(level: u8) -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_APP_SET_VOL, vec![level])
}

pub fn get_volume_ext() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_APP_GET_VOL, vec![])
}

// ---------------------------------------------------------------------------
// Playback
// ---------------------------------------------------------------------------

pub fn set_play_status(playing: bool) -> Packet {
    Packet::new(cmd::SPP_SET_PLAY_STATUS, vec![if playing { 1 } else { 0 }])
}

pub fn get_play_status() -> Packet {
    Packet::new(cmd::SPP_GET_PLAY_STATUS, vec![])
}

pub fn play_pause() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_APP_SET_MUSIC_PLAY_PAUSE, vec![])
}

pub fn skip_next() -> Packet {
    Packet::new(cmd::SPP_SET_LAST_NEXT, vec![1])
}

pub fn skip_prev() -> Packet {
    Packet::new(cmd::SPP_SET_LAST_NEXT, vec![0])
}

// ---------------------------------------------------------------------------
// SD Card Music
// ---------------------------------------------------------------------------

pub fn set_sd_music_id(id: u16) -> Packet {
    Packet::new(
        cmd::SPP_SET_SD_PLAY_MUSIC_ID,
        vec![(id & 0xFF) as u8, (id >> 8) as u8],
    )
}

pub fn set_sd_music_position(pos: u16) -> Packet {
    Packet::new(
        cmd::SPP_SET_SD_MUSIC_POSITION,
        vec![(pos & 0xFF) as u8, (pos >> 8) as u8],
    )
}

pub fn set_sd_music_play_mode(mode: u8) -> Packet {
    Packet::new(cmd::SPP_SET_SD_MUSIC_PLAY_MODE, vec![mode])
}

pub fn get_sd_music_list(start: u16, end: u16) -> Packet {
    Packet::new(
        cmd::SPP_GET_SD_MUSIC_LIST,
        vec![
            (start & 0xFF) as u8,
            (start >> 8) as u8,
            (end & 0xFF) as u8,
            (end >> 8) as u8,
        ],
    )
}

pub fn get_sd_music_info() -> Packet {
    Packet::new(cmd::SPP_GET_SD_MUSIC_INFO, vec![])
}

// ---------------------------------------------------------------------------
// Sound Control
// ---------------------------------------------------------------------------

pub fn set_sound_ctrl(enabled: bool) -> Packet {
    Packet::new(cmd::SPP_SET_SOUND_CTRL, vec![if enabled { 1 } else { 0 }])
}

pub fn get_sound_ctrl() -> Packet {
    Packet::new(cmd::SPP_GET_SOUND_CTRL, vec![])
}

// ---------------------------------------------------------------------------
// Equalizer
// ---------------------------------------------------------------------------

pub fn set_equalizer(mode: u8, bands: &[u8; 10]) -> Packet {
    let mut payload = Vec::with_capacity(12);
    payload.push(0x01);
    payload.push(mode);
    payload.extend_from_slice(bands);
    Packet::ext(ext_cmd::SPP_SECOND_SEND_DEVICE_EQUALIZER_CTRL, payload)
}

pub fn reset_equalizer() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_SEND_DEVICE_EQUALIZER_CTRL, vec![0x00])
}

// ---------------------------------------------------------------------------
// Voice
// ---------------------------------------------------------------------------

/// Control voice playback: 0 = record, 1 = play, 2 = stop.
pub fn set_voice_playback(control: u8) -> Packet {
    Packet::new(cmd::SPP_SET_PLAY_STOP_VOICE, vec![control])
}

pub fn get_voice_status() -> Packet {
    Packet::new(cmd::SPP_GET_PLAY_VOICE_STATUS, vec![])
}

// ---------------------------------------------------------------------------
// Mixer
// ---------------------------------------------------------------------------

pub fn set_mixer_mode(mode: u8, param: u8) -> Packet {
    Packet::new(cmd::SPP_SET_NEW_MIX_MUSIC_MODE, vec![mode, param])
}

pub fn set_mix_music_mode(mode: u8, param: u8, enabled: bool) -> Packet {
    Packet::new(
        cmd::SPP_SET_MIX_MUISE_MODE,
        vec![mode, param, if enabled { 1 } else { 0 }],
    )
}

// ---------------------------------------------------------------------------
// Mic & Karaoke
// ---------------------------------------------------------------------------

pub fn set_mic_switch(on: bool) -> Packet {
    Packet::new(cmd::SPP_SET_SPP_SET_MIC_SWITCH, vec![if on { 1 } else { 0 }])
}

pub fn karaoke_ctrl(params: &[u8]) -> Packet {
    let mut payload = vec![0x01];
    payload.extend_from_slice(params);
    Packet::ext(ext_cmd::SPP_SECOND_KARAOKE_CTRL, payload)
}

pub fn karaoke_reset() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_KARAOKE_CTRL, vec![0x00])
}

pub fn record_ctrl(control: u8) -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_RECORD_CTRL, vec![control])
}

// ---------------------------------------------------------------------------
// Power-on Sound
// ---------------------------------------------------------------------------

pub fn set_poweron_voice_volume(volume: u8) -> Packet {
    Packet::new(cmd::SPP_SET_POWERON_VOICE_VOL, vec![0x01, volume])
}

pub fn mute_poweron_voice() -> Packet {
    Packet::new(cmd::SPP_SET_POWERON_VOICE_VOL, vec![0x00, 0x00])
}

pub fn set_poweron_voice_ctrl(control: u8) -> Packet {
    Packet::new(cmd::SPP_SET_POWERON_VOICE_CTRL, vec![control])
}

// ---------------------------------------------------------------------------
// Song Display
// ---------------------------------------------------------------------------

pub fn set_song_display(enabled: bool) -> Packet {
    Packet::new(cmd::SPP_SET_SONG_DIS_CTRL, vec![if enabled { 1 } else { 0 }])
}

pub fn get_song_display() -> Packet {
    Packet::new(cmd::SPP_SET_SONG_DIS_CTRL, vec![0xFF])
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::cmd as c;
    use crate::protocol::ext_cmd as ec;

    // Helper: for a standard command, verify command byte and payload.
    fn assert_std(pkt: &Packet, expected_cmd: u8, expected_payload: &[u8]) {
        assert_eq!(pkt.command, expected_cmd);
        assert_eq!(pkt.payload, expected_payload);
    }

    // Helper: for an extended command, the wire command is SPP_DIVOOM_EXTERN_CMD
    // and the payload starts with the ext_cmd id followed by the logical payload.
    fn assert_ext(pkt: &Packet, expected_ext: u8, expected_tail: &[u8]) {
        assert_eq!(pkt.command, c::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(pkt.payload[0], expected_ext);
        assert_eq!(&pkt.payload[1..], expected_tail);
    }

    // -- Volume ---------------------------------------------------------------

    #[test]
    fn test_set_volume() {
        assert_std(&set_volume(42), c::SPP_SET_VOL, &[42]);
    }

    #[test]
    fn test_get_volume() {
        assert_std(&get_volume(), c::SPP_GET_VOL, &[]);
    }

    #[test]
    fn test_set_volume_ext() {
        assert_ext(&set_volume_ext(80), ec::SPP_SECOND_APP_SET_VOL, &[80]);
    }

    #[test]
    fn test_get_volume_ext() {
        assert_ext(&get_volume_ext(), ec::SPP_SECOND_APP_GET_VOL, &[]);
    }

    // -- Playback -------------------------------------------------------------

    #[test]
    fn test_set_play_status() {
        assert_std(&set_play_status(true), c::SPP_SET_PLAY_STATUS, &[1]);
        assert_std(&set_play_status(false), c::SPP_SET_PLAY_STATUS, &[0]);
    }

    #[test]
    fn test_get_play_status() {
        assert_std(&get_play_status(), c::SPP_GET_PLAY_STATUS, &[]);
    }

    #[test]
    fn test_play_pause() {
        assert_ext(&play_pause(), ec::SPP_SECOND_APP_SET_MUSIC_PLAY_PAUSE, &[]);
    }

    #[test]
    fn test_skip_next() {
        assert_std(&skip_next(), c::SPP_SET_LAST_NEXT, &[1]);
    }

    #[test]
    fn test_skip_prev() {
        assert_std(&skip_prev(), c::SPP_SET_LAST_NEXT, &[0]);
    }

    // -- SD Card Music --------------------------------------------------------

    #[test]
    fn test_set_sd_music_id() {
        // 0x0102 => lo=0x02, hi=0x01
        let pkt = set_sd_music_id(0x0102);
        assert_std(&pkt, c::SPP_SET_SD_PLAY_MUSIC_ID, &[0x02, 0x01]);
    }

    #[test]
    fn test_set_sd_music_position() {
        let pkt = set_sd_music_position(0x0304);
        assert_std(&pkt, c::SPP_SET_SD_MUSIC_POSITION, &[0x04, 0x03]);
    }

    #[test]
    fn test_set_sd_music_play_mode() {
        assert_std(&set_sd_music_play_mode(2), c::SPP_SET_SD_MUSIC_PLAY_MODE, &[2]);
    }

    #[test]
    fn test_get_sd_music_list() {
        let pkt = get_sd_music_list(0x0001, 0x00FF);
        assert_std(
            &pkt,
            c::SPP_GET_SD_MUSIC_LIST,
            &[0x01, 0x00, 0xFF, 0x00],
        );
    }

    #[test]
    fn test_get_sd_music_info() {
        assert_std(&get_sd_music_info(), c::SPP_GET_SD_MUSIC_INFO, &[]);
    }

    // -- Sound Control --------------------------------------------------------

    #[test]
    fn test_set_sound_ctrl() {
        assert_std(&set_sound_ctrl(true), c::SPP_SET_SOUND_CTRL, &[1]);
        assert_std(&set_sound_ctrl(false), c::SPP_SET_SOUND_CTRL, &[0]);
    }

    #[test]
    fn test_get_sound_ctrl() {
        assert_std(&get_sound_ctrl(), c::SPP_GET_SOUND_CTRL, &[]);
    }

    // -- Equalizer ------------------------------------------------------------

    #[test]
    fn test_set_equalizer() {
        let bands = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let pkt = set_equalizer(3, &bands);
        assert_ext(
            &pkt,
            ec::SPP_SECOND_SEND_DEVICE_EQUALIZER_CTRL,
            &[0x01, 3, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
        );
    }

    #[test]
    fn test_reset_equalizer() {
        assert_ext(
            &reset_equalizer(),
            ec::SPP_SECOND_SEND_DEVICE_EQUALIZER_CTRL,
            &[0x00],
        );
    }

    // -- Voice ----------------------------------------------------------------

    #[test]
    fn test_set_voice_playback() {
        assert_std(&set_voice_playback(0), c::SPP_SET_PLAY_STOP_VOICE, &[0]);
        assert_std(&set_voice_playback(1), c::SPP_SET_PLAY_STOP_VOICE, &[1]);
        assert_std(&set_voice_playback(2), c::SPP_SET_PLAY_STOP_VOICE, &[2]);
    }

    #[test]
    fn test_get_voice_status() {
        assert_std(&get_voice_status(), c::SPP_GET_PLAY_VOICE_STATUS, &[]);
    }

    // -- Mixer ----------------------------------------------------------------

    #[test]
    fn test_set_mixer_mode() {
        assert_std(&set_mixer_mode(1, 5), c::SPP_SET_NEW_MIX_MUSIC_MODE, &[1, 5]);
    }

    #[test]
    fn test_set_mix_music_mode() {
        assert_std(
            &set_mix_music_mode(2, 7, true),
            c::SPP_SET_MIX_MUISE_MODE,
            &[2, 7, 1],
        );
        assert_std(
            &set_mix_music_mode(2, 7, false),
            c::SPP_SET_MIX_MUISE_MODE,
            &[2, 7, 0],
        );
    }

    // -- Mic & Karaoke --------------------------------------------------------

    #[test]
    fn test_set_mic_switch() {
        assert_std(&set_mic_switch(true), c::SPP_SET_SPP_SET_MIC_SWITCH, &[1]);
        assert_std(&set_mic_switch(false), c::SPP_SET_SPP_SET_MIC_SWITCH, &[0]);
    }

    #[test]
    fn test_karaoke_ctrl() {
        let pkt = karaoke_ctrl(&[0xAA, 0xBB]);
        assert_ext(&pkt, ec::SPP_SECOND_KARAOKE_CTRL, &[0x01, 0xAA, 0xBB]);
    }

    #[test]
    fn test_karaoke_ctrl_empty() {
        let pkt = karaoke_ctrl(&[]);
        assert_ext(&pkt, ec::SPP_SECOND_KARAOKE_CTRL, &[0x01]);
    }

    #[test]
    fn test_karaoke_reset() {
        assert_ext(&karaoke_reset(), ec::SPP_SECOND_KARAOKE_CTRL, &[0x00]);
    }

    #[test]
    fn test_record_ctrl() {
        assert_ext(&record_ctrl(1), ec::SPP_SECOND_RECORD_CTRL, &[1]);
    }

    // -- Power-on Sound -------------------------------------------------------

    #[test]
    fn test_set_poweron_voice_volume() {
        assert_std(
            &set_poweron_voice_volume(75),
            c::SPP_SET_POWERON_VOICE_VOL,
            &[0x01, 75],
        );
    }

    #[test]
    fn test_mute_poweron_voice() {
        assert_std(&mute_poweron_voice(), c::SPP_SET_POWERON_VOICE_VOL, &[0, 0]);
    }

    #[test]
    fn test_set_poweron_voice_ctrl() {
        assert_std(&set_poweron_voice_ctrl(1), c::SPP_SET_POWERON_VOICE_CTRL, &[1]);
    }

    // -- Song Display ---------------------------------------------------------

    #[test]
    fn test_set_song_display() {
        assert_std(&set_song_display(true), c::SPP_SET_SONG_DIS_CTRL, &[1]);
        assert_std(&set_song_display(false), c::SPP_SET_SONG_DIS_CTRL, &[0]);
    }

    #[test]
    fn test_get_song_display() {
        assert_std(&get_song_display(), c::SPP_SET_SONG_DIS_CTRL, &[0xFF]);
    }

    // -- Wire encoding round-trips --------------------------------------------

    #[test]
    fn test_encode_framing() {
        // Verify every packet produces valid wire framing (0x01 ... 0x02).
        let packets = [
            set_volume(50),
            get_volume(),
            play_pause(),
            set_equalizer(0, &[0; 10]),
            karaoke_ctrl(&[1, 2, 3]),
        ];
        for pkt in &packets {
            let wire = pkt.encode();
            assert_eq!(wire[0], 0x01, "start marker");
            assert_eq!(*wire.last().unwrap(), 0x02, "end marker");
            // No bare 0x01/0x02/0x03 in the inner bytes.
            for &b in &wire[1..wire.len() - 1] {
                assert!(b != 0x01 && b != 0x02, "unescaped marker in payload");
            }
        }
    }
}
