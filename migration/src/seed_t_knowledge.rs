use crate::create_t_knowledge::TKnowledge;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let init_system_rule = vec![
            "1###EnumProcesses### 它可以用来在系统上枚举运行进程的函数，恶意代码经常枚举进程来找到一个可以注入的进程",
            "2###connect###它可以用来连接一个远程套接字。恶意代码经常使用底层功能函数来连接一个命令控制服务器",
            "3###CreateProcess### 它可以创建并启动一个新进程。如果恶意代码创建了一个新进程，需要同时分析这个新进程",
            "4###CreateMutex### 它可以创建一个互斥对象，可以被恶意代码用来确保在给定时刻只有一个实例在系统上运行。恶意代码经常使用固定名字为互斥对象命名，这样它们就可以成为一个很好的主机特征，来检测系统是否感染了这个恶意代码",
            "5###CreateRemoteThread### 它可以用来在一个远程进程（也就是与调用进程不同的进程）中启动一个线程。启动器和隐蔽性恶意代码通常使用这个函数，将代码注入到其他进程中执行",
            "6###AdjustTokenPrivileges### 它可以用来启用或禁用特定的访问权限。执行进程注入的恶意代码会经常调用这个函数 ，来取得额外的权限",
            "7###CryptAcquireContext### 它可以这经常是恶意代码用来初始化使用Windows加密库的第一个函数。还有很多其他函数是和加密相关的，绝大多数的函数名都以Crypt开头",
            "8###CreateService### 它可以创建一个可以在启动时刻运行的服务。恶意代码使用它来实现持久化、 隐藏，或是启动内核驱动",
            "9###EnableExecuteProtectionSupport### 它是一个未经文档化的API函数， 用来修改宿主上的数据执行保护（DEP）设置，使得系统更容易被攻击。",
            "10###DeviceIoControl### 它可以从用户空间向设备驱动发送一 个控制消息。它在驱动级的恶意代码中是非常普遍使用的，因为它是一种最简单和灵话的方式，在用户空间和内核空间之间传递信息",
            "11###CertOpenSystemStore### 它可以用来访问在本地系统中的证书。",
        ];
        for rule in init_system_rule {
            let on_conflict = OnConflict::column(TKnowledge::Id).do_nothing().to_owned();
            let value: Vec<&str> = rule.split("###").collect();
            let insert = Query::insert()
                .into_table(TKnowledge::Table)
                .columns([
                    TKnowledge::Id,
                    TKnowledge::FuncName,
                    TKnowledge::FuncDesc,
                    TKnowledge::IsSensitive,
                    TKnowledge::ModifyTime,
                    TKnowledge::CreateTime,
                ])
                .values_panic([
                    value[0].into(),
                    value[1].into(),
                    value[2].into(),
                    true.into(),
                    chrono::Local::now().naive_local().into(),
                    chrono::Local::now().naive_local().into(),
                ])
                .on_conflict(on_conflict)
                .to_owned();
            manager.exec_stmt(insert).await?;
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete().from_table(TKnowledge::Table).to_owned();
        _manager.exec_stmt(delete).await?;
        Ok(())
    }
}
