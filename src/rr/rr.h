#pragma once

#include <memory>
#include <ReplaySession.h>
#include <Wrapper.h>

namespace wrapper
{
    using ReplaySession_Flags = rr::ReplaySession::Flags;

    std::unique_ptr<ReplaySession_Flags> ReplaySession_Flags_default();
    void ReplaySession_Flags_redirect_stdio(const std::unique_ptr<ReplaySession_Flags> &flags, bool value);

    std::shared_ptr<rr::ReplaySession> ReplaySession_create(const std::string &dir, const ReplaySession_Flags &flags);
}
