unsafe extern "C" {
    fn sqlite_src_version();
}

pub fn sqlite_version() {
    unsafe { sqlite_src_version() }
}
