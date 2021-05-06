#pragma once

#include <memory>
#include <ReplaySession.h>

namespace wrapper
{
    using ReplaySession_Flags = rr::ReplaySession::Flags;
    using ReplaySession = rr::ReplaySession;

    std::unique_ptr<ReplaySession_Flags> ReplaySession_Flags_default();
    void ReplaySession_Flags_redirect_stdio(ReplaySession_Flags &flags, bool value);

    std::shared_ptr<rr::ReplaySession> ReplaySession_create(
        const std::string &dir, const ReplaySession_Flags &flags);
} // namespace wrapper

namespace rr
{
    void assert_prerequisites(bool use_syscall_buffer = false);

    void print_global_options(FILE *);
    void print_usage(FILE *);

    bool parse_global_option(std::vector<std::string> &args);
}
