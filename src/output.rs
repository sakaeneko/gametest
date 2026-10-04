use crate::data::EspFrame;

// 每帧序列化，丢给 overlay
pub fn emit_frame(frame: &EspFrame) {
    // 简单版：stdout，一行一个实体
    for e in &frame.entities {
        if let Some((sx, sy)) = e.screen_pos {
            println!(
                "{{\"x\":{:.1},\"y\":{:.1},\"hp\":{},\"ap\":{},\"wp\":\"{}\"}}",
                sx, sy, e.health, e.armor, e.weapon_name
            );
        }
    }
    // 生产版：写共享内存 / TCP，等 overlay 读 [坑]
}
