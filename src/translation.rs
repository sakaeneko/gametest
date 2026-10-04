// 编码密文 → 明文坐标
// 家底看完，拿到 密文/明文 成对样本后，在这里实现
pub fn decode_position(raw: [u8; 12]) -> (f32, f32, f32) {
    // TODO: 坑
    // 现在原样透传，等翻译官填
    let x = f32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    let y = f32::from_le_bytes([raw[4], raw[5], raw[6], raw[7]]);
    let z = f32::from_le_bytes([raw[8], raw[9], raw[10], raw[11]]);
    (x, y, z)
}
