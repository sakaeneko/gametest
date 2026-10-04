use memflow::prelude::v1::*;
use crate::offsets::offsets;
use crate::translation::decode_position;

#[derive(Debug, Clone, Default)]
pub struct EntityData {
    pub world_pos: (f32, f32, f32),
    pub screen_pos: Option<(f32, f32)>,
    pub health: i32,
    pub armor: i32,
    pub team: u8,
    pub weapon_name: String,
    pub is_local: bool,
    pub is_dormant: bool,
}

#[derive(Debug, Clone, Default)]
pub struct EspFrame {
    pub entities: Vec<EntityData>,
    pub local_team: u8,
    pub view_matrix: [[f32; 4]; 4],
}

pub fn read_frame(
    process: &mut impl MemoryView,
    base: Address,
    max_actors: u32,
) -> Option<EspFrame> {
    // 1. 视图矩阵
    let vm_addr = base + offsets::VIEW_MATRIX;
    let mut view_matrix = [[0f32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            let v: f32 = process.read(vm_addr + (i * 4 + j) as u64 * 4).ok()?;
            view_matrix[i][j] = v;
        }
    }

    // 2. GWorld → Level
    let gworld_ptr: Address = process.read(base + offsets::GWORLD).ok()?;
    if gworld_ptr.is_null() { return None; }
    let gworld: Address = process.read(gworld_ptr).ok()?;
    if gworld.is_null() { return None; }
    let level: Address = process.read(gworld + offsets::PERSISTENT_LEVEL).ok()?;
    if level.is_null() { return None; }

    // 3. Actors 数组
    let actors_data: Address = process.read(level + offsets::ACTORS_ARRAY).ok()?;
    let actors_count: u32 = process.read(
        level + offsets::ACTORS_ARRAY + offsets::ACTORS_COUNT_OFF
    ).ok()?;
    let count = actors_count.min(max_actors);

    // 4. 本地玩家
    let local_pawn: Address = process.read(base + offsets::LOCAL_PLAYER).ok()?;
    let local_team: u8 = process
        .read(local_pawn + offsets::ENTITY_TEAM)
        .unwrap_or(0);

    // 5. 遍历 Actor
    let mut entities = Vec::with_capacity(count as usize);
    for i in 0..count {
        let actor: Address = match process.read(actors_data + i as u64 * 8) {
            Ok(a) if !a.is_null() => a,
            _ => continue,
        };

        let dormant: bool = process
            .read(actor + offsets::ENTITY_DORMANT)
            .unwrap_or(true);
        if dormant { continue; }

        let health: i32 = process
            .read(actor + offsets::ENTITY_HEALTH)
            .unwrap_or(0);
        if health <= 0 { continue; }

        let team: u8 = process
            .read(actor + offsets::ENTITY_TEAM)
            .unwrap_or(255);
        if team == local_team { continue; }

        let mut raw = [0u8; 12];
        for k in 0..12 {
            raw[k] = process
                .read(actor + offsets::ENTITY_POS + k as u64)
                .unwrap_or(0);
        }
        let world_pos = decode_position(raw);

        let armor: i32 = process
            .read(actor + offsets::ENTITY_ARMOR)
            .unwrap_or(0);

        let weapon_name = String::from("?");

        entities.push(EntityData {
            world_pos,
            screen_pos: None,
            health,
            armor,
            team,
            weapon_name,
            is_local: false,
            is_dormant: false,
        });
    }

    Some(EspFrame { entities, local_team, view_matrix })
}
