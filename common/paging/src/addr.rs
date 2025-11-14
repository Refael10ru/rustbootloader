#[cfg(target_arch = "aarch64")]
use aarch64::paging::{PhysAddr, VirtAddr};

#[cfg(target_arch = "x86_64")]
use x86_64::{PhysAddr, VirtAddr};
