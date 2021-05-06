#[cxx::bridge(namespace = "wrapper")]
mod ffi {
    unsafe extern "C++" {
        include!("server/wrapper/wrapper.h");

        type ReplaySession_Flags;

        fn ReplaySession_Flags_default() -> UniquePtr<ReplaySession_Flags>;

        fn ReplaySession_Flags_redirect_stdio(flags: Pin<&mut ReplaySession_Flags>, value: bool);

        type ReplaySession;

        fn ReplaySession_create(
            dir: &CxxString,
            flags: &ReplaySession_Flags,
        ) -> SharedPtr<ReplaySession>;
    }

    impl SharedPtr<ReplaySession> {}
}

pub(crate) use ffi::*;
