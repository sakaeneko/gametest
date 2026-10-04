// 运行时配置 —— 坑，家底看完填
pub struct Config {
    pub process_name: &'static str,
    pub screen_w: f32,
    pub screen_h: f32,
    pub tick_ms: u64,
    pub max_actors: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            process_name: "五字进程名.exe", // 坑
            screen_w: 2560.0,
            screen_h: 1440.0,
            tick_ms: 16,
            max_actors: 2000,
        }
    }
}
