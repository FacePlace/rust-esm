use bytes::Bytes;

pub fn str_from_buffer(buffer: Bytes) -> String {
  String::from_utf8(buffer.to_vec()).expect("Error")
}
