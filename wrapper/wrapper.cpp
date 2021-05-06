#include <memory>
#include <iostream>
#include <ReplaySession.h>

namespace wrapper
{
    std::shared_ptr<rr::ReplaySession> ReplaySession_create(
        const std::string &dir, const rr::ReplaySession::Flags &flags)
    {
        return rr::ReplaySession::create(dir, flags);
    }

    std::unique_ptr<rr::ReplaySession::Flags> ReplaySession_Flags_default()
    {
        auto flags = std::make_unique<rr::ReplaySession::Flags>();
        flags->redirect_stdio = true;
        return flags;
    }

    void ReplaySession_Flags_redirect_stdio(rr::ReplaySession::Flags &flags, bool value)
    {
        flags.redirect_stdio = value;
    }
} // namespace wrapper

namespace rr
{
    void assert_prerequisites(bool use_syscall_buffer = false) {}

    void print_global_options(FILE *) {}
    void print_usage(FILE *) {}

    bool parse_global_option(std::vector<std::string> &args)
    {
        return true;
    }
}
