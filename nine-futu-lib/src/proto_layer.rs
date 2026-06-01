use bytes::{Buf, BufMut, Bytes, BytesMut};
use sha1::{Digest, Sha1};
use std::io::Read;
use std::sync::atomic::{AtomicU32, Ordering};

pub const HEADER_SIZE: usize = 44;
pub const HEADER_FLAG: [u8; 2] = [0x46, 0x54]; // "FT"

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtoFmt {
    Protobuf = 0,
    Json = 1,
}

impl ProtoFmt {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Protobuf,
            1 => Self::Json,
            _ => Self::Protobuf,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FutuHeader {
    pub proto_id: u32,
    pub proto_fmt: ProtoFmt,
    pub proto_ver: u8,
    pub serial_no: u32,
    pub body_len: u32,
    pub body_sha1: [u8; 20],
}

impl FutuHeader {
    pub fn new(proto_id: u32, serial_no: u32, body_len: u32, body: &[u8]) -> Self {
        let mut hasher = Sha1::new();
        hasher.update(body);
        let result = hasher.finalize();
        let mut body_sha1 = [0u8; 20];
        body_sha1.copy_from_slice(&result);

        Self {
            proto_id,
            proto_fmt: ProtoFmt::Protobuf,
            proto_ver: 0,
            serial_no,
            body_len,
            body_sha1,
        }
    }

    pub fn parse(buf: &mut BytesMut) -> Result<Self, crate::error::FutuError> {
        if buf.len() < HEADER_SIZE {
            return Err(crate::error::FutuError::PacketDataErr);
        }

        let header_bytes = buf.split_to(HEADER_SIZE);
        let mut cursor = &header_bytes[..];

        // Check header flag
        let flag = [cursor.get_u8(), cursor.get_u8()];
        if flag != HEADER_FLAG {
            return Err(crate::error::FutuError::PacketDataErr);
        }

        let proto_id = cursor.get_u32_le();
        let proto_fmt = ProtoFmt::from_u8(cursor.get_u8());
        let proto_ver = cursor.get_u8();
        let serial_no = cursor.get_u32_le();
        let body_len = cursor.get_u32_le();

        let mut body_sha1 = [0u8; 20];
        cursor.read_exact(&mut body_sha1)?;

        // Skip reserved 8 bytes
        let _reserved = [cursor.get_u8(); 8];

        Ok(Self {
            proto_id,
            proto_fmt,
            proto_ver,
            serial_no,
            body_len,
            body_sha1,
        })
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_slice(&HEADER_FLAG);
        buf.put_u32_le(self.proto_id);
        buf.put_u8(self.proto_fmt as u8);
        buf.put_u8(self.proto_ver);
        buf.put_u32_le(self.serial_no);
        buf.put_u32_le(self.body_len);
        buf.put_slice(&self.body_sha1);
        buf.put_slice(&[0u8; 8]); // reserved
    }

    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(HEADER_SIZE);
        self.serialize(&mut buf);
        buf
    }
}

pub struct SerialManager {
    next: AtomicU32,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            next: AtomicU32::new(1),
        }
    }

    pub fn next(&self) -> u32 {
        self.next.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for SerialManager {
    fn default() -> Self {
        Self::new()
    }
}

pub const ALL_PUSH_IDS: &[u32] = &[
    1003,  // Notify
    2208,  // Trd_UpdateOrder
    2218,  // Trd_UpdateOrderFill
    3005,  // Qot_UpdateBasicQot
    3007,  // Qot_UpdateKL
    3009,  // Qot_UpdateRT
    3011,  // Qot_UpdateTicker
    3013,  // Qot_UpdateOrderBook
    3015,  // Qot_UpdateBroker
    3019,  // Qot_UpdatePriceReminder
];

pub fn is_push_proto_id(proto_id: u32) -> bool {
    ALL_PUSH_IDS.contains(&proto_id)
}

pub struct ProtoRequest {
    pub proto_id: u32,
    pub serial_no: u32,
    pub body: Bytes,
}

impl ProtoRequest {
    pub fn new(proto_id: u32, serial_no: u32, body: Bytes) -> Self {
        Self {
            proto_id,
            serial_no,
            body,
        }
    }

    pub fn to_bytes(&self) -> BytesMut {
        let header = FutuHeader::new(self.proto_id, self.serial_no, self.body.len() as u32, &self.body);
        let mut buf = BytesMut::with_capacity(HEADER_SIZE + self.body.len());
        header.serialize(&mut buf);
        buf.put_slice(&self.body);
        buf
    }
}

pub struct ProtoResponse {
    pub header: FutuHeader,
    pub body: Bytes,
}

impl ProtoResponse {
    pub fn parse(buf: &mut BytesMut) -> Result<Self, crate::error::FutuError> {
        let header = FutuHeader::parse(buf)?;

        if buf.len() < header.body_len as usize {
            return Err(crate::error::FutuError::PacketDataErr);
        }

        let body = buf.split_to(header.body_len as usize).freeze();

        Ok(Self { header, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_new() {
        let body = b"test body data";
        let header = FutuHeader::new(3203, 42, body.len() as u32, body);
        
        assert_eq!(header.proto_id, 3203);
        assert_eq!(header.serial_no, 42);
        assert_eq!(header.body_len, body.len() as u32);
        assert_eq!(header.proto_fmt, ProtoFmt::Protobuf);
        assert_eq!(header.proto_ver, 0);
        assert_eq!(header.body_sha1.len(), 20);
    }

    #[test]
    fn test_header_sha1_hash() {
        let body1 = b"test body data";
        let body2 = b"different body";
        
        let header1 = FutuHeader::new(1001, 1, body1.len() as u32, body1);
        let header2 = FutuHeader::new(1001, 1, body2.len() as u32, body2);
        
        // Different bodies should produce different SHA1 hashes
        assert_ne!(header1.body_sha1, header2.body_sha1);
    }

    #[test]
    fn test_header_same_body_same_hash() {
        let body = b"same body data";
        
        let header1 = FutuHeader::new(1001, 1, body.len() as u32, body);
        let header2 = FutuHeader::new(1001, 2, body.len() as u32, body);
        
        // Same body should produce same SHA1 hash
        assert_eq!(header1.body_sha1, header2.body_sha1);
    }

    #[test]
    fn test_header_serialize_parse_roundtrip() {
        let body = b"hello world";
        let original = FutuHeader::new(3203, 42, body.len() as u32, body);
        
        let mut buf = original.to_bytes();
        assert_eq!(buf.len(), HEADER_SIZE);
        
        let parsed = FutuHeader::parse(&mut buf).unwrap();
        
        assert_eq!(parsed.proto_id, original.proto_id);
        assert_eq!(parsed.serial_no, original.serial_no);
        assert_eq!(parsed.body_len, original.body_len);
        assert_eq!(parsed.proto_fmt, original.proto_fmt);
        assert_eq!(parsed.proto_ver, original.proto_ver);
        assert_eq!(parsed.body_sha1, original.body_sha1);
    }

    #[test]
    fn test_header_parse_insufficient_data() {
        let mut buf = BytesMut::from(&[0u8; 10][..]);
        let result = FutuHeader::parse(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_header_parse_invalid_flag() {
        let mut buf = BytesMut::with_capacity(HEADER_SIZE);
        buf.put_slice(&[0x00, 0x00]); // Invalid flag
        buf.put_slice(&[0u8; HEADER_SIZE - 2]);
        
        let result = FutuHeader::parse(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_header_parse_correct_flag() {
        let mut buf = BytesMut::with_capacity(HEADER_SIZE);
        buf.put_slice(&HEADER_FLAG); // "FT"
        buf.put_slice(&[0u8; HEADER_SIZE - 2]);
        
        let result = FutuHeader::parse(&mut buf);
        assert!(result.is_ok());
    }

    #[test]
    fn test_header_proto_id_values() {
        let body = b"";
        
        // InitConnect
        let header = FutuHeader::new(1001, 1, 0, body);
        assert_eq!(header.proto_id, 1001);
        
        // GetSecuritySnapshot
        let header = FutuHeader::new(3203, 2, 0, body);
        assert_eq!(header.proto_id, 3203);
        
        // GetKL
        let header = FutuHeader::new(3006, 3, 0, body);
        assert_eq!(header.proto_id, 3006);
    }

    #[test]
    fn test_serial_manager_sequential() {
        let mgr = SerialManager::new();
        
        let s1 = mgr.next();
        let s2 = mgr.next();
        let s3 = mgr.next();
        
        assert_eq!(s1, 1);
        assert_eq!(s2, 2);
        assert_eq!(s3, 3);
    }

    #[test]
    fn test_serial_manager_overflow() {
        let mgr = SerialManager::new();
        
        // Set to near max
        mgr.next.store(u32::MAX - 1, Ordering::Relaxed);
        
        let s1 = mgr.next();
        let s2 = mgr.next();
        
        assert_eq!(s1, u32::MAX - 1);
        assert_eq!(s2, u32::MAX); // Should wrap to 0 on next call
    }

    #[test]
    fn test_push_detection_all_ids() {
        let push_ids = [1003, 2208, 2218, 3005, 3007, 3009, 3011, 3013, 3015, 3019];
        
        for id in push_ids {
            assert!(is_push_proto_id(id), "Proto ID {} should be push", id);
        }
    }

    #[test]
    fn test_push_detection_non_push_ids() {
        let non_push_ids = [1001, 3001, 3003, 3004, 3006, 3010, 3012, 3203];
        
        for id in non_push_ids {
            assert!(!is_push_proto_id(id), "Proto ID {} should not be push", id);
        }
    }

    #[test]
    fn test_proto_fmt_from_u8() {
        assert_eq!(ProtoFmt::from_u8(0), ProtoFmt::Protobuf);
        assert_eq!(ProtoFmt::from_u8(1), ProtoFmt::Json);
        assert_eq!(ProtoFmt::from_u8(2), ProtoFmt::Protobuf); // Default
        assert_eq!(ProtoFmt::from_u8(255), ProtoFmt::Protobuf); // Default
    }

    #[test]
    fn test_proto_request_to_bytes() {
        let body = Bytes::from(vec![1, 2, 3, 4, 5]);
        let request = ProtoRequest::new(3203, 42, body.clone());
        
        let buf = request.to_bytes();
        
        // Should be header + body
        assert_eq!(buf.len(), HEADER_SIZE + body.len());
        
        // Parse header from the buffer
        let mut parse_buf = buf.clone();
        let header = FutuHeader::parse(&mut parse_buf).unwrap();
        
        assert_eq!(header.proto_id, 3203);
        assert_eq!(header.serial_no, 42);
        assert_eq!(header.body_len, 5);
    }

    #[test]
    fn test_proto_response_parse() {
        let body = Bytes::from(vec![10, 20, 30]);
        let request = ProtoRequest::new(3006, 99, body.clone());
        
        let mut buf = request.to_bytes();
        let response = ProtoResponse::parse(&mut buf).unwrap();
        
        assert_eq!(response.header.proto_id, 3006);
        assert_eq!(response.header.serial_no, 99);
        assert_eq!(response.body.as_ref(), body.as_ref());
    }

    #[test]
    fn test_header_empty_body() {
        let body = b"";
        let header = FutuHeader::new(1001, 1, 0, body);
        
        assert_eq!(header.body_len, 0);
        assert_eq!(header.body_sha1.len(), 20);
        
        let mut buf = header.to_bytes();
        let parsed = FutuHeader::parse(&mut buf).unwrap();
        assert_eq!(parsed.body_len, 0);
    }

    #[test]
    fn test_header_large_body() {
        let body = vec![0u8; 1024 * 1024]; // 1MB body
        let header = FutuHeader::new(3203, 1, body.len() as u32, &body);
        
        assert_eq!(header.body_len, 1024 * 1024);
    }
}
