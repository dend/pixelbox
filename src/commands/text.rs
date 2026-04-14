//! LED text, notification, and message commands for the Divoom Ditoo Pro.

use crate::protocol::{cmd, Packet};

/// Encode a string as UTF-16LE bytes.
fn encode_utf16le(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .flat_map(|code_unit| code_unit.to_le_bytes())
        .collect()
}

// ---------------------------------------------------------------------------
// LED Text
// ---------------------------------------------------------------------------

/// Send text to the LED word display.
///
/// The text is encoded as UTF-16LE. Payload: `[0x01, byte_len, ...utf16le_bytes]`.
pub fn send_led_text(text: &str) -> Packet {
    let utf16_bytes = encode_utf16le(text);
    let mut payload = Vec::with_capacity(2 + utf16_bytes.len());
    payload.push(0x01); // enable flag
    payload.push(utf16_bytes.len() as u8);
    payload.extend_from_slice(&utf16_bytes);
    Packet::new(cmd::SPP_SEND_LED_WORD_CMD, payload)
}

/// Clear the LED word display.
pub fn clear_led_text() -> Packet {
    Packet::new(cmd::SPP_SEND_LED_WORD_CMD, vec![0x00, 0x00])
}

// ---------------------------------------------------------------------------
// 32-pixel text attributes
// ---------------------------------------------------------------------------

/// Set the text color for 32-pixel word display.
pub fn set_text_color(r: u8, g: u8, b: u8) -> Packet {
    Packet::new(
        cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR,
        vec![0x05, r, g, b],
    )
}

/// Set the text scrolling/animation effect for 32-pixel word display.
pub fn set_text_effect(effect: u8) -> Packet {
    Packet::new(cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR, vec![0x02, effect])
}

/// Set the text size for 32-pixel word display.
pub fn set_text_size(size: u8) -> Packet {
    Packet::new(cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR, vec![0x04, size])
}

/// Set the text scrolling speed for 32-pixel word display (little-endian u16).
pub fn set_text_speed(speed: u16) -> Packet {
    Packet::new(
        cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR,
        vec![0x01, (speed & 0xFF) as u8, (speed >> 8) as u8],
    )
}

/// Set the text frame parameters for 32-pixel word display.
pub fn set_text_frame(p1: u8, p2: u8, p3: u8, p4: u8) -> Packet {
    Packet::new(
        cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR,
        vec![0x03, p1, p2, p3, p4],
    )
}

// ---------------------------------------------------------------------------
// Notifications (ANCS)
// ---------------------------------------------------------------------------

/// Set the notification icon color for a given notification type.
pub fn set_notification_pic(notify_id: u8, r: u8, g: u8, b: u8) -> Packet {
    Packet::new(cmd::SPP_SET_ANCS_NOTICE_PIC, vec![notify_id, r, g, b])
}

/// Send a notification text to the device (Android ANCS style).
///
/// The text is encoded as UTF-8. Payload: `[notify_type, text_len, ...text_bytes]`.
pub fn set_notification_text(notify_type: u8, text: &str) -> Packet {
    let text_bytes = text.as_bytes();
    let mut payload = Vec::with_capacity(2 + text_bytes.len());
    payload.push(notify_type);
    payload.push(text_bytes.len() as u8);
    payload.extend_from_slice(text_bytes);
    Packet::new(cmd::SPP_SET_ANDROID_ANCS, payload)
}

/// Reset all notification settings to their defaults.
pub fn reset_notifications() -> Packet {
    Packet::new(cmd::SPP_RESET_NOTIFICATIONS, vec![])
}

// ---------------------------------------------------------------------------
// Leave Message
// ---------------------------------------------------------------------------

/// Delete the stored leave message GIF from the device.
pub fn delete_leave_message() -> Packet {
    Packet::new(cmd::SPP_DEL_LEAVE_MSG_GIF, vec![])
}

// ---------------------------------------------------------------------------
// Scrolling text (convenience)
// ---------------------------------------------------------------------------

/// Set scrolling text with a given speed.
///
/// Returns two packets: the first sends the LED text, the second sets the
/// scroll speed.
pub fn set_scroll_text(text: &str, speed: u16) -> Vec<Packet> {
    vec![send_led_text(text), set_text_speed(speed)]
}

// ---------------------------------------------------------------------------
// Daily Time Message
// ---------------------------------------------------------------------------

/// Query the current daily time message configuration.
pub fn get_daily_time() -> Packet {
    Packet::new(cmd::SPP_GET_DIALY_TIME_EXT2, vec![])
}

/// Set a daily time message.
///
/// `params` should be 7 bytes of configuration. The text is encoded as UTF-8
/// and zero-padded to exactly 32 bytes.
pub fn set_daily_time(params: &[u8], text: &str) -> Packet {
    let text_bytes = text.as_bytes();
    let mut payload = Vec::with_capacity(7 + 32);
    payload.extend_from_slice(&params[..7]);
    payload.extend_from_slice(text_bytes);
    payload.resize(7 + 32, 0);
    Packet::new(cmd::SPP_SET_DIALY_TIME_EXT2, payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- UTF-16LE encoding ---------------------------------------------------

    #[test]
    fn test_encode_utf16le_ascii() {
        let bytes = encode_utf16le("Hi");
        // 'H' = 0x0048 -> [0x48, 0x00], 'i' = 0x0069 -> [0x69, 0x00]
        assert_eq!(bytes, vec![0x48, 0x00, 0x69, 0x00]);
    }

    #[test]
    fn test_encode_utf16le_empty() {
        assert!(encode_utf16le("").is_empty());
    }

    #[test]
    fn test_encode_utf16le_non_ascii() {
        // CJK character U+4E16 ('世') -> [0x16, 0x4E]
        let bytes = encode_utf16le("世");
        assert_eq!(bytes, vec![0x16, 0x4E]);
    }

    #[test]
    fn test_encode_utf16le_surrogate_pair() {
        // U+1F600 (😀) requires a surrogate pair: D83D DE00
        let bytes = encode_utf16le("\u{1F600}");
        assert_eq!(bytes, vec![0x3D, 0xD8, 0x00, 0xDE]);
    }

    // -- LED text ------------------------------------------------------------

    #[test]
    fn test_send_led_text() {
        let pkt = send_led_text("AB");
        assert_eq!(pkt.command, cmd::SPP_SEND_LED_WORD_CMD);
        // 'A' = [0x41, 0x00], 'B' = [0x42, 0x00] -> 4 bytes
        assert_eq!(pkt.payload[0], 0x01); // enable
        assert_eq!(pkt.payload[1], 4);    // byte length
        assert_eq!(&pkt.payload[2..], &[0x41, 0x00, 0x42, 0x00]);
    }

    #[test]
    fn test_send_led_text_empty() {
        let pkt = send_led_text("");
        assert_eq!(pkt.payload, vec![0x01, 0x00]);
    }

    #[test]
    fn test_clear_led_text() {
        let pkt = clear_led_text();
        assert_eq!(pkt.command, cmd::SPP_SEND_LED_WORD_CMD);
        assert_eq!(pkt.payload, vec![0x00, 0x00]);
    }

    // -- 32-pixel text attributes --------------------------------------------

    #[test]
    fn test_set_text_color() {
        let pkt = set_text_color(0xFF, 0x80, 0x00);
        assert_eq!(pkt.command, cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR);
        assert_eq!(pkt.payload, vec![0x05, 0xFF, 0x80, 0x00]);
    }

    #[test]
    fn test_set_text_effect() {
        let pkt = set_text_effect(3);
        assert_eq!(pkt.command, cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR);
        assert_eq!(pkt.payload, vec![0x02, 3]);
    }

    #[test]
    fn test_set_text_size() {
        let pkt = set_text_size(16);
        assert_eq!(pkt.command, cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR);
        assert_eq!(pkt.payload, vec![0x04, 16]);
    }

    #[test]
    fn test_set_text_speed() {
        let pkt = set_text_speed(0x1234);
        assert_eq!(pkt.command, cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR);
        assert_eq!(pkt.payload, vec![0x01, 0x34, 0x12]);
    }

    #[test]
    fn test_set_text_speed_small() {
        let pkt = set_text_speed(100);
        assert_eq!(pkt.payload, vec![0x01, 100, 0]);
    }

    #[test]
    fn test_set_text_frame() {
        let pkt = set_text_frame(10, 20, 30, 40);
        assert_eq!(pkt.command, cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR);
        assert_eq!(pkt.payload, vec![0x03, 10, 20, 30, 40]);
    }

    // -- Notifications -------------------------------------------------------

    #[test]
    fn test_set_notification_pic() {
        let pkt = set_notification_pic(5, 0xAA, 0xBB, 0xCC);
        assert_eq!(pkt.command, cmd::SPP_SET_ANCS_NOTICE_PIC);
        assert_eq!(pkt.payload, vec![5, 0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_set_notification_text() {
        let pkt = set_notification_text(1, "hello");
        assert_eq!(pkt.command, cmd::SPP_SET_ANDROID_ANCS);
        assert_eq!(pkt.payload[0], 1);
        assert_eq!(pkt.payload[1], 5); // "hello".len()
        assert_eq!(&pkt.payload[2..], b"hello");
    }

    #[test]
    fn test_set_notification_text_empty() {
        let pkt = set_notification_text(0, "");
        assert_eq!(pkt.payload, vec![0, 0]);
    }

    #[test]
    fn test_reset_notifications() {
        let pkt = reset_notifications();
        assert_eq!(pkt.command, cmd::SPP_RESET_NOTIFICATIONS);
        assert!(pkt.payload.is_empty());
    }

    // -- Leave message -------------------------------------------------------

    #[test]
    fn test_delete_leave_message() {
        let pkt = delete_leave_message();
        assert_eq!(pkt.command, cmd::SPP_DEL_LEAVE_MSG_GIF);
        assert!(pkt.payload.is_empty());
    }

    // -- Scrolling text ------------------------------------------------------

    #[test]
    fn test_set_scroll_text() {
        let packets = set_scroll_text("Hi", 500);
        assert_eq!(packets.len(), 2);

        // First packet: LED text
        assert_eq!(packets[0].command, cmd::SPP_SEND_LED_WORD_CMD);
        assert_eq!(packets[0].payload[0], 0x01);

        // Second packet: speed
        assert_eq!(packets[1].command, cmd::SPP_LIEGHT_PHONE_GIF32_WORD_ATTR);
        assert_eq!(packets[1].payload[0], 0x01);
        assert_eq!(packets[1].payload[1], (500 & 0xFF) as u8);
        assert_eq!(packets[1].payload[2], (500 >> 8) as u8);
    }

    // -- Daily time message --------------------------------------------------

    #[test]
    fn test_get_daily_time() {
        let pkt = get_daily_time();
        assert_eq!(pkt.command, cmd::SPP_GET_DIALY_TIME_EXT2);
        assert!(pkt.payload.is_empty());
    }

    #[test]
    fn test_set_daily_time() {
        let params = [1, 2, 3, 4, 5, 6, 7];
        let pkt = set_daily_time(&params, "Hey");
        assert_eq!(pkt.command, cmd::SPP_SET_DIALY_TIME_EXT2);
        assert_eq!(pkt.payload.len(), 7 + 32);
        assert_eq!(&pkt.payload[..7], &params);
        assert_eq!(&pkt.payload[7..10], b"Hey");
        // Remaining bytes are zero-padded
        assert!(pkt.payload[10..].iter().all(|&b| b == 0));
    }

    #[test]
    fn test_set_daily_time_long_text_truncated() {
        let params = [0; 7];
        let long_text = "abcdefghijklmnopqrstuvwxyz012345XX"; // 34 chars
        let pkt = set_daily_time(&params, long_text);
        // Total payload is capped to 7 + 32 = 39 bytes
        assert_eq!(pkt.payload.len(), 39);
    }

    // -- Wire encoding sanity ------------------------------------------------

    #[test]
    fn test_led_text_encodes_without_panic() {
        let pkt = send_led_text("test");
        let encoded = pkt.encode();
        assert_eq!(encoded[0], 0x01);
        assert_eq!(*encoded.last().unwrap(), 0x02);
    }

    #[test]
    fn test_notification_text_encodes_without_panic() {
        let pkt = set_notification_text(2, "msg");
        let encoded = pkt.encode();
        assert_eq!(encoded[0], 0x01);
        assert_eq!(*encoded.last().unwrap(), 0x02);
    }
}
