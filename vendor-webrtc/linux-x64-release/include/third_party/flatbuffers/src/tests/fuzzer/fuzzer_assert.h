#ifndef FUZZER_ASSERT_IMPL_H_
#define FUZZER_ASSERT_IMPL_H_

#if defined(_MSC_VER)
extern "C" void __debugbreak();
#define __builtin_trap __debugbreak
#else // Clang
extern "C" void __builtin_trap(void);
#endif

// Declare Debug/Release independed assert macro.
#define fuzzer_assert_impl(x) (!!(x) ? static_cast<void>(0) : __builtin_trap())

#endif // !FUZZER_ASSERT_IMPL_H_
