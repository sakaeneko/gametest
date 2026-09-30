use memflow::prelude::v1::*;
use memflow::plugins::Inventory;
use memflow_win32::prelude::v1::*;

fn main() -> Result<()> {
    // 1. 初始化 memflow 插件管理器（自动扫描 memflowup 安装的连接器）
    let inventory = Inventory::new()?;

    // 2. 创建 KVM 连接器
    let connector = inventory.create_connector("kvm", &ConnectorArgs::default())?;

    // 3. 初始化 Win32 内核抽象
    let mut kernel = Win32Kernel::builder(connector)
        .build_default_caches()
        .build()?;

    // 4. 列出前 10 个进程，确认连接正常
    println!("=== 进程列表（前10个） ===");
    let process_list = kernel.process_info_list()?;
    for p in process_list.iter().take(10) {
        println!("PID: {:<6} 名称: {}", p.pid, p.name);
    }

    // 5. 附加到你的测试进程（请替换为实际进程名）
    let target_process_name = "test_target.exe";
    let mut process = kernel.process(target_process_name)?;
    println!("\n已附加到进程: {}", target_process_name);

    // 6. 读取一个已知地址的 i32 值（请替换为你测试程序中的实际地址）
    //    这个地址可以通过 Cheat Engine 或你自己在测试程序中打印得到
    let target_address = Address::from(0x00007FF6_12345678u64);
    let value: i32 = process.read(target_address)?;
    println!("读取地址 {:x} 的值: {}", target_address, value);

    Ok(())
}
