#pragma once

#include <memory>
#include "vendor/rr/src/ReplaySession.h"

namespace wrapper
{
    using ReplaySession_Flags = rr::ReplaySession::Flags;
    using ReplaySession = rr::ReplaySession;

    std::unique_ptr<ReplaySession_Flags> ReplaySession_Flags_default();
    void ReplaySession_Flags_redirect_stdio(ReplaySession_Flags &flags, bool value);

    std::shared_ptr<rr::ReplaySession> ReplaySession_create(
        const std::string &dir, const ReplaySession_Flags &flags);
} // namespace wrapper
