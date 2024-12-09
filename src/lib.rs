//! # lego_device 设备驱动接口
//! 
//! lego标准接口组件，设备驱动接口
//! 
//! ---
//! 
//! 计算机中的设备按数据传输的方式，总体上来讲可以分为两大类，分别是字符设备和块设备。
//! 
//! **字符设备**通常是按**字符流（byte stream）**的方式进行传输，不支持随机访问，通常是顺序读取或写入。常见的字符设备例如：串口、键盘、鼠标、网络设备……
//! 
//! **块设备**数据以**块（block）**为单位进行传输，支持随机访问，一般用于存储设备，每块大小通常是 512 字节或 4 KB，常见的例子：硬盘、SSD、闪存……
//! 
//! ## 设计
//! 
//! 开发人员需要为不同类型设备编写驱动。我们参考Linux中的设备驱动设计，将设备驱动接口设计为四种`DeviceType`，同时对应四种接口类型：
//! 
//! - `CharDevice`：适用于字符设备驱动的接口，如诸如串口、键盘、鼠标的驱动程序。
//! - `BlockDevice`：适用于块设备驱动的接口，如硬盘、固态、SD卡、virtio ……
//! - `NetDevice`：适用于网络设备驱动的接口
//! - `BusDevice`：适用于总线设备驱动的接口
//! 
//! 这四种接口中定义了数据传输的行为，开发人员需要实现具体的函数逻辑。并且四种接口皆继承了`Device`接口，Device接口定义了设备类型、初始化、关闭等行为。
//! 
//! `DeviceStatus`标识了设备当前的状态，开发者在编写接口中定义的行为时，需要为设备标识当前设备的状态，设备的状态可以分为：
//! 
//! - Uninitialized：设备正处于未初始化状态，此时不允许进行其他的操作
//! - Idle：设备已经初始化，处于空闲状态
//! - Transfer：设备处于工作状态（可能在传输数据）
//! - Suspended：设备处于挂起或低功耗状态
//! - Stop：由于用户操作或系统配置原因导致停止了设备的使用
//! - Error：设备在运行时发生了错误，可能是硬件问题或软件异常
//! 
//! `DeviceInfo`接口用于保存设备信息，比如设备名称、vendor、类型……，目前改接口还在设计中。
#![no_std]
#![feature(error_generic_member_access)]
#[cfg(feature = "block")]
mod block_dev;
#[cfg(feature = "bus")]
mod bus_dev;
#[cfg(feature = "char")]
mod char_dev;
mod dev;
mod err;
#[cfg(feature = "net")]
mod net_dev;
mod register;
#[cfg(feature = "block")]
pub use block_dev::*;
#[cfg(feature = "char")]
pub use char_dev::*;
pub use dev::*;
pub use err::DeviceError;
pub use register::*;