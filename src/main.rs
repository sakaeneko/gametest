mod config;
mod mem;

use anyhow::{Context, Result};
use clap::Parser;
use memflow::prelude::v1::*;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "kvm_mem_test", version, about = "KVM 内存读取测试")]
struct Cli {
    /// 配置文件路径
    #[arg(short, long, default_value = "offsets.toml")]
    config: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = config::Config::load(&cli.config)?;

    println!("[*] 目标进程: {}", cfg.target.process_name);
    println!("[*] 目标模块: {}", cfg.target.module_name);

    // 1. 初始化 KVM + Win32
    let mut kernel = mem::init_kernel()?;

    // 2. 附加到目标进程
    let mut process = mem::attach(&mut kernel, &cfg.target.process_name)?;

    // 3. 获取模块基址
    let module = process
        .module_by_name(&cfg.target.module_name)
        .with_context(|| format!("找不到模块 {}", cfg.target.module_name))?;
    let base = module.base;
    println!("[+] 模块基址: {:x}", base);

    // 4. 计算实际地址（模块基址 + 偏移）
    let secret_addr  = Address::from(base.to_umem() + cfg.offsets.secret_value()?);
    let counter_addr = Address::from(base.to_umem() + cfg.offsets.counter()?);
    println!("[+] secret_value 地址: {:x}", secret_addr);
    println!("[+] counter 地址:      {:x}", counter_addr);

    // 5. 循环读取
    println!("\n[*] 开始读取，Ctrl+C 退出\n");
    loop {
        let secret:  i32 = process.read(secret_addr)?;
        let counter: i32 = process.read(counter_addr)?;
        println!("secret_value = {:<8} counter = {}", secret, counter);
        thread::sleep(Duration::from_secs(1));
    }
}
