#include <memory>
#include "vendor/rr/src/ReplaySession.h"

namespace wrapper
{
    std::shared_ptr<rr::ReplaySession> ReplaySession_create(
        const std::string &dir, const rr::ReplaySession::Flags &flags)
    {
        return rr::ReplaySession::create(dir, flags);
    }

    std::unique_ptr<rr::ReplaySession::Flags> ReplaySession_Flags_default()
    {
        return std::make_unique<rr::ReplaySession::Flags>();
    }

    void ReplaySession_Flags_redirect_stdio(rr::ReplaySession::Flags &flags, bool value)
    {
        flags.redirect_stdio = value;
    }
} // namespace wrapper
