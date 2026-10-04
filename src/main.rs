mod config;
mod offsets;
mod translation;
mod data;
mod projection;
mod output;

use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;
use memflow_kvm::KVMConnector;
use std::time::Duration;
use config::Config;

fn main() -> Result<()> {
    let cfg = Config::default();
    println!("[RiliS] 线上电...");

    // 1. 连接层：集成 kvm connector，直连 /dev/memflow
    let connector = KVMConnector::try_new(ConnectorArgs::new())?;
    println!("[RiliS] 线已接");
    let os = Win32::try_new(connector, "win32", "")?;
    println!("[RiliS] 客户机 OS 层已加载");

    // 2. 进程层：找五字 [坑: 进程名待确认]
    println!("[RiliS] 找进程: {}", cfg.process_name);
    let mut process = os.process_by_name(cfg.process_name)?;
    let module = process.module_by_name(cfg.process_name)?;
    let base = module.base();
    println!("[RiliS] 基址: 0x{:X}", base);

    // 3. 主循环
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
