use std::ffi::CString;

use super::{MhyContext, MhyModule, ModuleType};
use crate::marshal;
use anyhow::Result;
use ilhook::x64::Registers;

const UNITY_WEB_REQUEST_SET_URL: usize = 0x0D9442D0;

pub struct Http;

impl MhyModule for MhyContext<Http> {
    unsafe fn init(&mut self) -> Result<()> {
        self.interceptor.attach(
            self.assembly_base + UNITY_WEB_REQUEST_SET_URL,
            on_uwr_set_url,
        )
    }

    unsafe fn de_init(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::Http
    }
}

unsafe extern "win64" fn on_uwr_set_url(reg: *mut Registers, _: usize) {
    
    
}
