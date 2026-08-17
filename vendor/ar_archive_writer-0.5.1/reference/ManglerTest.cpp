//===- llvm/unittest/IR/ManglerTest.cpp - Mangler unit tests --------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "llvm/IR/Mangler.h"
#include "llvm/IR/CallingConv.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/GlobalValue.h"
#include "llvm/IR/Module.h"
#include "gtest/gtest.h"

using namespace llvm;

static std::string mangleStr(StringRef IRName, Mangler &Mang,
                             const DataLayout &DL) {
  std::string Mangled;
  raw_string_ostream SS(Mangled);
  Mang.getNameWithPrefix(SS, IRName, DL);
  return Mangled;
}

static std::string mangleFunc(StringRef IRName,
                              GlobalValue::LinkageTypes Linkage,
                              llvm::CallingConv::ID CC, Module &Mod,
                              Mangler &Mang) {
  Type *VoidTy = Type::getVoidTy(Mod.getContext());
  Type *I32Ty = Type::getInt32Ty(Mod.getContext());
  FunctionType *FTy =
      FunctionType::get(VoidTy, {I32Ty, I32Ty, I32Ty}, /*isVarArg=*/false);
  Function *F = Function::Create(FTy, Linkage, IRName, &Mod);
  F->setCallingConv(CC);
  std::string Mangled;
  raw_string_ostream SS(Mangled);
  Mang.getNameWithPrefix(SS, F, false);
  F->eraseFromParent();
  return Mangled;
}

namespace {

TEST(ManglerTest, MachO) {
  LLVMContext Ctx;
  DataLayout DL("m:o"); // macho
  Module Mod("test", Ctx);
  Mod.setDataLayout(DL);
  Mangler Mang;
  EXPECT_EQ(mangleStr("foo", Mang, DL), "_foo");
  EXPECT_EQ(mangleStr("\01foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("?foo", Mang, DL), "_?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "_foo");
  EXPECT_EQ(mangleFunc("?foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "_?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::PrivateLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "L_foo");
}

TEST(ManglerTest, WindowsX86) {
  LLVMContext Ctx;
  DataLayout DL("m:x-p:32:32"); // 32-bit windows
  Module Mod("test", Ctx);
  Mod.setDataLayout(DL);
  Mangler Mang;
  EXPECT_EQ(mangleStr("foo", Mang, DL), "_foo");
  EXPECT_EQ(mangleStr("\01foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("?foo", Mang, DL), "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "_foo");
  EXPECT_EQ(mangleFunc("?foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::PrivateLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "L_foo");

  // Test calling conv mangling.
  EXPECT_EQ(mangleFunc("stdcall", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::X86_StdCall, Mod, Mang),
            "_stdcall@12");
  EXPECT_EQ(mangleFunc("fastcall", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::X86_FastCall, Mod, Mang),
            "@fastcall@12");
  EXPECT_EQ(mangleFunc("vectorcall", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::X86_VectorCall, Mod, Mang),
            "vectorcall@@12");

  // Adding a '?' prefix blocks calling convention mangling.
  EXPECT_EQ(mangleFunc("?fastcall", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::X86_FastCall, Mod, Mang),
            "?fastcall");
}

TEST(ManglerTest, WindowsX64) {
  LLVMContext Ctx;
  DataLayout DL("m:w-p:64:64"); // windows
  Module Mod("test", Ctx);
  Mod.setDataLayout(DL);
  Mangler Mang;
  EXPECT_EQ(mangleStr("foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("\01foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("?foo", Mang, DL), "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "foo");
  EXPECT_EQ(mangleFunc("?foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::PrivateLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            ".Lfoo");

  // Test calling conv mangling.
  EXPECT_EQ(mangleFunc("stdcall", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::X86_StdCall, Mod, Mang),
            "stdcall");
  EXPECT_EQ(mangleFunc("fastcall", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::X86_FastCall, Mod, Mang),
            "fastcall");
  EXPECT_EQ(mangleFunc("vectorcall", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::X86_VectorCall, Mod, Mang),
            "vectorcall@@24");

  // Adding a '?' prefix blocks calling convention mangling.
  EXPECT_EQ(mangleFunc("?vectorcall", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::X86_VectorCall, Mod, Mang),
            "?vectorcall");
}

TEST(ManglerTest, UEFIX64) {
  LLVMContext Ctx;
  DataLayout DL("e-m:w-p270:32:32-p271:32:32-p272:64:64-"
                "i64:64-i128:128-f80:128-n8:16:32:64-S128"); // uefi X86_64
  Module Mod("test", Ctx);
  Mod.setDataLayout(DL);
  Mangler Mang;
  EXPECT_EQ(mangleStr("foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("\01foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("?foo", Mang, DL), "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "foo");
  EXPECT_EQ(mangleFunc("?foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::PrivateLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            ".Lfoo");
}

TEST(ManglerTest, XCOFF) {
  LLVMContext Ctx;
  DataLayout DL("m:a"); // XCOFF/AIX
  Module Mod("test", Ctx);
  Mod.setDataLayout(DL);
  Mangler Mang;
  EXPECT_EQ(mangleStr("foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("\01foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("?foo", Mang, DL), "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "foo");
  EXPECT_EQ(mangleFunc("?foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::PrivateLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "L..foo");
}

TEST(ManglerTest, GOFF) {
  LLVMContext Ctx;
  DataLayout DL("m:l"); // GOFF
  Module Mod("test", Ctx);
  Mod.setDataLayout(DL);
  Mangler Mang;

  EXPECT_EQ(mangleStr("foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("\01foo", Mang, DL), "foo");
  EXPECT_EQ(mangleStr("?foo", Mang, DL), "?foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::ExternalLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "foo");
  EXPECT_EQ(mangleFunc("foo", llvm::GlobalValue::PrivateLinkage,
                       llvm::CallingConv::C, Mod, Mang),
            "L#foo");
}

TEST(ManglerTest, Arm64EC) {
  constexpr std::string_view Arm64ECNames[] = {
      // Basic C name.
      "#Foo",

      // Basic C++ name.
      "?foo@@$$hYAHXZ",

      // Regression test: https://github.com/llvm/llvm-project/issues/115231
      "?GetValue@?$Wrapper@UA@@@@$$hQEBAHXZ",

      // Symbols from:
      // ```
      // namespace A::B::C::D {
      // struct Base {
      //   virtual int f() { return 0; }
      // };
      // }
      // struct Derived : public A::B::C::D::Base {
      //   virtual int f() override { return 1; }
      // };
      // A::B::C::D::Base* MakeObj() { return new Derived(); }
      // ```
      // void * __cdecl operator new(unsigned __int64)
      "??2@$$hYAPEAX_K@Z",
      // public: virtual int __cdecl A::B::C::D::Base::f(void)
      "?f@Base@D@C@B@A@@$$hUEAAHXZ",
      // public: __cdecl A::B::C::D::Base::Base(void)
      "??0Base@D@C@B@A@@$$hQEAA@XZ",
      // public: virtual int __cdecl Derived::f(void)
      "?f@Derived@@$$hUEAAHXZ",
      // public: __cdecl Derived::Derived(void)
      "??0Derived@@$$hQEAA@XZ",
      // struct A::B::C::D::Base * __cdecl MakeObj(void)
      "?MakeObj@@$$hYAPEAUBase@D@C@B@A@@XZ",

      // Symbols from:
      // ```
      // template <typename T> struct WW { struct Z{}; };
      // template <typename X> struct Wrapper {
      //   int GetValue(typename WW<X>::Z) const;
      // };
      // struct A { };
      // template <typename X> int Wrapper<X>::GetValue(typename WW<X>::Z) const
      // { return 3; }
      // template class Wrapper<A>;
      // ```
      // public: int __cdecl Wrapper<struct A>::GetValue(struct WW<struct
      // A>::Z)const
      "?GetValue@?$Wrapper@UA@@@@$$hQEBAHUZ@?$WW@UA@@@@@Z",
  };

  for (const auto &Arm64ECName : Arm64ECNames) {
    // Check that this is a mangled name.
    EXPECT_TRUE(isArm64ECMangledFunctionName(Arm64ECName))
        << "Test case: " << Arm64ECName;
    // Refuse to mangle it again.
    EXPECT_FALSE(getArm64ECMangledFunctionName(Arm64ECName).has_value())
        << "Test case: " << Arm64ECName;

    // Demangle.
    auto Arm64Name = getArm64ECDemangledFunctionName(Arm64ECName);
    EXPECT_TRUE(Arm64Name.has_value()) << "Test case: " << Arm64ECName;
    // Check that it is not mangled.
    EXPECT_FALSE(isArm64ECMangledFunctionName(Arm64Name.value()))
        << "Test case: " << Arm64ECName;
    // Refuse to demangle it again.
    EXPECT_FALSE(getArm64ECDemangledFunctionName(Arm64Name.value()).has_value())
        << "Test case: " << Arm64ECName;

    // Round-trip.
    auto RoundTripArm64ECName =
        getArm64ECMangledFunctionName(Arm64Name.value());
    EXPECT_EQ(RoundTripArm64ECName, Arm64ECName);
  }
}

} // end anonymous namespace