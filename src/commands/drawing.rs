use crate::protocol::{cmd, ext_cmd, Packet};

// ---------------------------------------------------------------------------
// Static images
// ---------------------------------------------------------------------------

pub fn encode_pic(pic_id: u8, encoded_data: &[u8]) -> Packet {
    let len = encoded_data.len();
    let mut payload = Vec::with_capacity(3 + len);
    payload.push(pic_id);
    payload.push((len & 0xFF) as u8);
    payload.push((len >> 8) as u8);
    payload.extend_from_slice(encoded_data);
    Packet::new(cmd::SPP_DRAWING_ENCODE_PIC, payload)
}

pub fn encode_play(pic_id: u8, frame_data: &[u8], speed: u16) -> Packet {
    let total_len = frame_data.len();
    let mut payload = Vec::with_capacity(5 + frame_data.len());
    payload.push(pic_id);
    payload.push((total_len & 0xFF) as u8);
    payload.push((total_len >> 8) as u8);
    payload.push((speed & 0xFF) as u8);
    payload.push((speed >> 8) as u8);
    payload.extend_from_slice(frame_data);
    Packet::new(cmd::SPP_DRAWING_ENCODE_PLAY, payload)
}

// ---------------------------------------------------------------------------
// Drawing pad (real-time)
// ---------------------------------------------------------------------------

pub fn drawing_pad_enter(r: u8, g: u8, b: u8) -> Packet {
    Packet::new(cmd::SPP_DRAWING_MUL_PAD_ENTER, vec![r, g, b])
}

pub fn drawing_pad_ctrl(x: u8, y: u8, r: u8, g: u8, b: u8) -> Packet {
    Packet::new(cmd::SPP_DRAWING_PAD_CTRL, vec![0, x, y, 1, r, g, b])
}

pub fn drawing_pad_exit() -> Packet {
    Packet::new(cmd::SPP_DRAWING_PAD_EXIT, vec![])
}

// ---------------------------------------------------------------------------
// Animation / movie
// ---------------------------------------------------------------------------

pub fn ctrl_movie_play(play: bool) -> Packet {
    Packet::new(
        cmd::SPP_DRAWING_CTRL_MOVIE_PLAY,
        vec![if play { 1 } else { 0 }],
    )
}

pub fn encode_movie_play(file_id: u16, data: &[u8]) -> Packet {
    let data_len = data.len();
    let mut payload = Vec::with_capacity(4 + data_len);
    payload.push((file_id & 0xFF) as u8);
    payload.push((file_id >> 8) as u8);
    payload.push((data_len & 0xFF) as u8);
    payload.push((data_len >> 8) as u8);
    payload.extend_from_slice(data);
    Packet::new(cmd::SPP_DRAWING_ENCODE_MOVIE_PLAY, payload)
}

// ---------------------------------------------------------------------------
// User GIF
// ---------------------------------------------------------------------------

pub fn set_user_gif_start(anim_id: u8) -> Packet {
    Packet::new(cmd::SPP_SET_USER_GIF, vec![0x00, 0x02, anim_id])
}

pub fn set_user_gif_end() -> Packet {
    Packet::new(cmd::SPP_APP_NEW_USER_DEFINE2020, vec![0x02])
}

pub fn stop_send_gif() -> Packet {
    Packet::new(cmd::SPP_STOP_SEND_GIF, vec![])
}

// ---------------------------------------------------------------------------
// GIF speed / play time
// ---------------------------------------------------------------------------

pub fn set_gif_speed(speed: u16) -> Packet {
    Packet::new(
        cmd::SPP_SET_GIF_SPEED_CMD,
        vec![(speed & 0xFF) as u8, (speed >> 8) as u8],
    )
}

pub fn set_gif_play_time(time: u16) -> Packet {
    Packet::ext(
        ext_cmd::SPP_SECOND_SET_GIF_PLAY_TIME_CFG,
        vec![(time & 0xFF) as u8, (time >> 8) as u8],
    )
}

pub fn reset_gif_play_time() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_SET_GIF_PLAY_TIME_CFG, vec![0xFF])
}

// ---------------------------------------------------------------------------
// Sand paint
// ---------------------------------------------------------------------------

pub fn sand_paint_start() -> Packet {
    Packet::new(cmd::SPP_SAND_PAINT_CTRL, vec![0x01])
}

pub fn sand_paint_data(pic_id: u8, data: &[u8]) -> Packet {
    let data_len = data.len();
    let mut payload = Vec::with_capacity(4 + data_len);
    payload.push(0x00);
    payload.push(pic_id);
    payload.push((data_len & 0xFF) as u8);
    payload.push((data_len >> 8) as u8);
    payload.extend_from_slice(data);
    Packet::new(cmd::SPP_SAND_PAINT_CTRL, payload)
}

// ---------------------------------------------------------------------------
// Scroll
// ---------------------------------------------------------------------------

pub fn scroll(mode: u8, speed: u16) -> Packet {
    Packet::new(
        cmd::SPP_SCROLL,
        vec![0x00, mode, (speed & 0xFF) as u8, (speed >> 8) as u8],
    )
}

// ---------------------------------------------------------------------------
// Boot animation
// ---------------------------------------------------------------------------

pub fn set_boot_gif(enabled: bool) -> Packet {
    Packet::new(
        cmd::SPP_SET_BOOT_GIF,
        vec![if enabled { 1 } else { 0 }],
    )
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Flatten a 16x16 grid of RGB pixels into a 768-byte buffer
/// suitable for sending to the device.
pub fn encode_rgb_pixels(pixels: &[[u8; 3]; 256]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(768);
    for rgb in pixels {
        buf.extend_from_slice(rgb);
    }
    buf
}

// ---------------------------------------------------------------------------
// Extended: send GIF start
// ---------------------------------------------------------------------------

pub fn send_gif_start() -> Packet {
    Packet::ext(ext_cmd::SPP_SECOND_APP_SEND_GIF_START, vec![])
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_pic() {
        let data = [0xAA, 0xBB, 0xCC];
        let pkt = encode_pic(1, &data);
        assert_eq!(pkt.command, cmd::SPP_DRAWING_ENCODE_PIC);
        assert_eq!(pkt.payload, vec![1, 3, 0, 0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_encode_pic_large_data() {
        let data = vec![0x42; 300];
        let pkt = encode_pic(0, &data);
        assert_eq!(pkt.payload[0], 0);
        assert_eq!(pkt.payload[1], (300 & 0xFF) as u8); // 0x2C
        assert_eq!(pkt.payload[2], (300 >> 8) as u8); // 0x01
        assert_eq!(pkt.payload.len(), 3 + 300);
    }

    #[test]
    fn test_encode_play() {
        let frame = [0x10, 0x20];
        let pkt = encode_play(5, &frame, 100);
        assert_eq!(pkt.command, cmd::SPP_DRAWING_ENCODE_PLAY);
        assert_eq!(
            pkt.payload,
            vec![5, 2, 0, 100, 0, 0x10, 0x20]
        );
    }

    #[test]
    fn test_encode_play_speed_le() {
        let pkt = encode_play(0, &[], 0x0302);
        assert_eq!(pkt.payload[3], 0x02); // speed lo
        assert_eq!(pkt.payload[4], 0x03); // speed hi
    }

    #[test]
    fn test_drawing_pad_enter() {
        let pkt = drawing_pad_enter(255, 0, 128);
        assert_eq!(pkt.command, cmd::SPP_DRAWING_MUL_PAD_ENTER);
        assert_eq!(pkt.payload, vec![255, 0, 128]);
    }

    #[test]
    fn test_drawing_pad_ctrl() {
        let pkt = drawing_pad_ctrl(3, 7, 10, 20, 30);
        assert_eq!(pkt.command, cmd::SPP_DRAWING_PAD_CTRL);
        assert_eq!(pkt.payload, vec![0, 3, 7, 1, 10, 20, 30]);
    }

    #[test]
    fn test_drawing_pad_exit() {
        let pkt = drawing_pad_exit();
        assert_eq!(pkt.command, cmd::SPP_DRAWING_PAD_EXIT);
        assert!(pkt.payload.is_empty());
    }

    #[test]
    fn test_ctrl_movie_play() {
        let pkt_on = ctrl_movie_play(true);
        assert_eq!(pkt_on.command, cmd::SPP_DRAWING_CTRL_MOVIE_PLAY);
        assert_eq!(pkt_on.payload, vec![1]);

        let pkt_off = ctrl_movie_play(false);
        assert_eq!(pkt_off.payload, vec![0]);
    }

    #[test]
    fn test_encode_movie_play() {
        let data = [0xDE, 0xAD];
        let pkt = encode_movie_play(0x1234, &data);
        assert_eq!(pkt.command, cmd::SPP_DRAWING_ENCODE_MOVIE_PLAY);
        assert_eq!(
            pkt.payload,
            vec![0x34, 0x12, 2, 0, 0xDE, 0xAD]
        );
    }

    #[test]
    fn test_set_user_gif_start() {
        let pkt = set_user_gif_start(42);
        assert_eq!(pkt.command, cmd::SPP_SET_USER_GIF);
        assert_eq!(pkt.payload, vec![0x00, 0x02, 42]);
    }

    #[test]
    fn test_set_user_gif_end() {
        let pkt = set_user_gif_end();
        assert_eq!(pkt.command, cmd::SPP_APP_NEW_USER_DEFINE2020);
        assert_eq!(pkt.payload, vec![0x02]);
    }

    #[test]
    fn test_stop_send_gif() {
        let pkt = stop_send_gif();
        assert_eq!(pkt.command, cmd::SPP_STOP_SEND_GIF);
        assert!(pkt.payload.is_empty());
    }

    #[test]
    fn test_set_gif_speed() {
        let pkt = set_gif_speed(500);
        assert_eq!(pkt.command, cmd::SPP_SET_GIF_SPEED_CMD);
        assert_eq!(pkt.payload, vec![0xF4, 0x01]); // 500 = 0x01F4
    }

    #[test]
    fn test_set_gif_play_time() {
        let pkt = set_gif_play_time(1000);
        // ext wraps with SPP_DIVOOM_EXTERN_CMD
        assert_eq!(pkt.command, cmd::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(
            pkt.payload,
            vec![ext_cmd::SPP_SECOND_SET_GIF_PLAY_TIME_CFG, 0xE8, 0x03]
        );
    }

    #[test]
    fn test_reset_gif_play_time() {
        let pkt = reset_gif_play_time();
        assert_eq!(pkt.command, cmd::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(
            pkt.payload,
            vec![ext_cmd::SPP_SECOND_SET_GIF_PLAY_TIME_CFG, 0xFF]
        );
    }

    #[test]
    fn test_sand_paint_start() {
        let pkt = sand_paint_start();
        assert_eq!(pkt.command, cmd::SPP_SAND_PAINT_CTRL);
        assert_eq!(pkt.payload, vec![0x01]);
    }

    #[test]
    fn test_sand_paint_data() {
        let data = [0x11, 0x22, 0x33];
        let pkt = sand_paint_data(2, &data);
        assert_eq!(pkt.command, cmd::SPP_SAND_PAINT_CTRL);
        assert_eq!(pkt.payload, vec![0x00, 2, 3, 0, 0x11, 0x22, 0x33]);
    }

    #[test]
    fn test_scroll() {
        let pkt = scroll(1, 256);
        assert_eq!(pkt.command, cmd::SPP_SCROLL);
        assert_eq!(pkt.payload, vec![0x00, 1, 0, 1]);
    }

    #[test]
    fn test_set_boot_gif() {
        let pkt = set_boot_gif(true);
        assert_eq!(pkt.command, cmd::SPP_SET_BOOT_GIF);
        assert_eq!(pkt.payload, vec![1]);

        let pkt = set_boot_gif(false);
        assert_eq!(pkt.payload, vec![0]);
    }

    #[test]
    fn test_encode_rgb_pixels() {
        let mut pixels = [[0u8; 3]; 256];
        pixels[0] = [255, 0, 0];
        pixels[1] = [0, 255, 0];
        pixels[255] = [0, 0, 255];

        let buf = encode_rgb_pixels(&pixels);
        assert_eq!(buf.len(), 768);
        assert_eq!(&buf[0..3], &[255, 0, 0]);
        assert_eq!(&buf[3..6], &[0, 255, 0]);
        assert_eq!(&buf[765..768], &[0, 0, 255]);
    }

    #[test]
    fn test_encode_rgb_pixels_all_white() {
        let pixels = [[255u8; 3]; 256];
        let buf = encode_rgb_pixels(&pixels);
        assert_eq!(buf.len(), 768);
        assert!(buf.iter().all(|&b| b == 255));
    }

    #[test]
    fn test_send_gif_start() {
        let pkt = send_gif_start();
        assert_eq!(pkt.command, cmd::SPP_DIVOOM_EXTERN_CMD);
        assert_eq!(pkt.payload, vec![ext_cmd::SPP_SECOND_APP_SEND_GIF_START]);
    }
}
