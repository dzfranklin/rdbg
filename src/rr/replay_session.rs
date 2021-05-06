use super::bridge;
use camino::Utf8Path;
use cxx::let_cxx_string;
use std::{path::Path, pin::Pin};

pub struct Flags(cxx::UniquePtr<bridge::ReplaySession_Flags>);

pub struct ReplaySession(cxx::SharedPtr<bridge::ReplaySession>);

impl Flags {
    pub fn new() -> Self {
        Self(bridge::ReplaySession_Flags_default())
    }

    fn redirect_stdio(&mut self, value: bool) -> &mut Self {
        bridge::ReplaySession_Flags_redirect_stdio(self.0.pin_mut(), value);
        self
    }

    fn share_private_mappings(&mut self, value: bool) -> &mut Self {
        unimplemented!()
    }

    fn cpu_unbound(&mut self, value: bool) -> &mut Self {
        unimplemented!()
    }
}
impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplaySession {
    pub fn create(trace_dir: &Utf8Path, flags: Flags) -> Self {
        let_cxx_string!(trace_dir = trace_dir.as_str());
        /*Self(*/
        bridge::ReplaySession_create(&*trace_dir, &*flags.0); /*)*/
        unimplemented!()
    }
}
