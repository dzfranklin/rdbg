#include <string>
#include <ReplaySession.h>

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

/* Setup: git submodule update --init &&
    cd vendor/rr && mkdir obj &&
    CC=clang CXX=clang++ cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug -DRR_BUILD_SHARED=ON ../ &&
    sudo cmake --build . --target install

    Run: g++ repro.cc -I vendor/rr/src -I vendor/rr/obj -lrr; ./a.out
*/

int main()
{
    auto flags = rr::ReplaySession::Flags();
    flags.redirect_stdio = false;
    flags.cpu_unbound = false;
    flags.share_private_mappings = true;
    auto session = rr::ReplaySession::create("/home/daniel/rr/writ-on-530", flags);
    return 0;
}
