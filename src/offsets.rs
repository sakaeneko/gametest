// 所有偏移，家底看完再填
pub mod offsets {
    // --- 全局指针链 ---
    pub const GWORLD: u64           = 0x0000_0000; // 坑
    pub const PERSISTENT_LEVEL: u64 = 0x0000_0000; // 坑
    pub const ACTORS_ARRAY: u64     = 0x0000_0000; // 坑
    pub const ACTORS_COUNT_OFF: u64 = 0x0000_0000; // 坑
    pub const LOCAL_PLAYER: u64     = 0x0000_0000; // 坑
    pub const VIEW_MATRIX: u64      = 0x0000_0000; // 坑

    // --- Actor 内部字段 ---
    pub const ENTITY_POS: u64       = 0x0000_0000; // 坑
    pub const ENTITY_HEALTH: u64    = 0x0000_0000; // 坑
    pub const ENTITY_ARMOR: u64     = 0x0000_0000; // 坑
    pub const ENTITY_TEAM: u64      = 0x0000_0000; // 坑
    pub const ENTITY_WEAPON: u64    = 0x0000_0000; // 坑
    pub const ENTITY_DORMANT: u64   = 0x0000_0000; // 坑

    // --- GWorld 特征码 ---
    pub const GWORLD_SIG: &[u8]     = &[]; // 坑
    pub const GWORLD_SIG_MASK: &[u8] = &[]; // 坑
}
