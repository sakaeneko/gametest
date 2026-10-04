mod config;
mod offsets;
mod translation;
mod data;
mod projection;
mod output;

use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;
use std::time::Duration;
use config::Config;

fn main() -> Result<()> {
    let cfg = Config::default();
    println!("[RiliS] 线上电...");

    // qemu 连接器：走 procfs + ptrace，不碰内核模块，不配 QMP
    let mut inventory = Inventory::scan();
    let connector = inventory.instantiate_connector(
        "qemu",
        None,
        None,
    )?;
    println!("[RiliS] 线已接 (qemu)");

    let mut kernel = Win32Kernel::builder(connector)
        .build_default_caches()
        .build()?;
    println!("[RiliS] 客户机 OS 层已加载");

    println!("[RiliS] 找进程: {}", cfg.process_name);
    let mut process = kernel.process_by_name(cfg.process_name)?;
    let module = process.module_by_name(cfg.process_name)?;
    let base = module.base;
    println!("[RiliS] 基址: 0x{:X}", base);

    loop {
        match data::read_frame(&mut process, base, cfg.max_actors) {
            Some(mut frame) => {
                for e in frame.entities.iter_mut() {
                    e.screen_pos = projection::world_to_screen(
                        e.world_pos, &frame.view_matrix,
                        cfg.screen_w, cfg.screen_h,
                    );
                }
                output::emit_frame(&frame);
            }
            None => eprintln!("[RiliS] 这帧没读到"),
        }
        std::thread::sleep(Duration::from_millis(cfg.tick_ms));
    }
}
