/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


#include "RefCounted.h"


class RefCountTaskWrapper : public gmp_args_base {
public:
  RefCountTaskWrapper(GMPTask* aTask, RefCounted* aRefCounted)
    : mTask(aTask)
    , mRefCounted(aRefCounted)
  {}
  virtual void Run() {
    mTask->Run();
  }
  virtual void Destroy() {
    mTask->Destroy();
    gmp_args_base::Destroy();
  }
private:
  ~RefCountTaskWrapper() {}

  GMPTask* mTask;
  RefPtr<RefCounted> mRefCounted;
};


// 0 arguments --
template<typename M> class gmp_args_nm_0 : public gmp_args_base {
 public:
  gmp_args_nm_0 (M m) :
    m_ (m)  {}

  void Run() {
    m_ ();
  }

 private:
  M m_;
};



// 0 arguments --
template<typename M, typename R> class gmp_args_nm_0_ret : public gmp_args_base {
 public:
  gmp_args_nm_0_ret (M m, R* r) :
    m_ (m), r_ (r)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ ();
  }

 private:
  M m_;
  R* r_;
};



// 0 arguments --
template<typename C, typename M> class gmp_args_m_0 : public gmp_args_base {
 public:
  gmp_args_m_0 (C o, M m) :
    o_ (o), m_ (m)  {}

  void Run() {
    ((*o_).*m_) ();
  }

 private:
  C o_;
  M m_;
};



// 0 arguments --
template<typename C, typename M, typename R> class gmp_args_m_0_ret : public gmp_args_base {
 public:
  gmp_args_m_0_ret (C o, M m, R* r) :
    o_ (o), m_ (m), r_ (r)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) ();
  }

 private:
  C o_;
  M m_;
  R* r_;
};



// 1 arguments --
template<typename M, typename A0> class gmp_args_nm_1 : public gmp_args_base {
 public:
  gmp_args_nm_1 (M m, A0 a0) :
    m_ (m), a0_ (a0)  {}

  void Run() {
    m_ (a0_);
  }

 private:
  M m_;
  A0 a0_;
};



// 1 arguments --
template<typename M, typename A0, typename R> class gmp_args_nm_1_ret : public gmp_args_base {
 public:
  gmp_args_nm_1_ret (M m, A0 a0, R* r) :
    m_ (m), r_ (r), a0_ (a0)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
};



// 1 arguments --
template<typename C, typename M, typename A0> class gmp_args_m_1 : public gmp_args_base {
 public:
  gmp_args_m_1 (C o, M m, A0 a0) :
    o_ (o), m_ (m), a0_ (a0)  {}

  void Run() {
    ((*o_).*m_) (a0_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
};



// 1 arguments --
template<typename C, typename M, typename A0, typename R> class gmp_args_m_1_ret : public gmp_args_base {
 public:
  gmp_args_m_1_ret (C o, M m, A0 a0, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
};



// 2 arguments --
template<typename M, typename A0, typename A1> class gmp_args_nm_2 : public gmp_args_base {
 public:
  gmp_args_nm_2 (M m, A0 a0, A1 a1) :
    m_ (m), a0_ (a0), a1_ (a1)  {}

  void Run() {
    m_ (a0_, a1_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
};



// 2 arguments --
template<typename M, typename A0, typename A1, typename R> class gmp_args_nm_2_ret : public gmp_args_base {
 public:
  gmp_args_nm_2_ret (M m, A0 a0, A1 a1, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
};



// 2 arguments --
template<typename C, typename M, typename A0, typename A1> class gmp_args_m_2 : public gmp_args_base {
 public:
  gmp_args_m_2 (C o, M m, A0 a0, A1 a1) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
};



// 2 arguments --
template<typename C, typename M, typename A0, typename A1, typename R> class gmp_args_m_2_ret : public gmp_args_base {
 public:
  gmp_args_m_2_ret (C o, M m, A0 a0, A1 a1, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
};



// 3 arguments --
template<typename M, typename A0, typename A1, typename A2> class gmp_args_nm_3 : public gmp_args_base {
 public:
  gmp_args_nm_3 (M m, A0 a0, A1 a1, A2 a2) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2)  {}

  void Run() {
    m_ (a0_, a1_, a2_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
};



// 3 arguments --
template<typename M, typename A0, typename A1, typename A2, typename R> class gmp_args_nm_3_ret : public gmp_args_base {
 public:
  gmp_args_nm_3_ret (M m, A0 a0, A1 a1, A2 a2, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
};



// 3 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2> class gmp_args_m_3 : public gmp_args_base {
 public:
  gmp_args_m_3 (C o, M m, A0 a0, A1 a1, A2 a2) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
};



// 3 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename R> class gmp_args_m_3_ret : public gmp_args_base {
 public:
  gmp_args_m_3_ret (C o, M m, A0 a0, A1 a1, A2 a2, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
};



// 4 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3> class gmp_args_nm_4 : public gmp_args_base {
 public:
  gmp_args_nm_4 (M m, A0 a0, A1 a1, A2 a2, A3 a3) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
};



// 4 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename R> class gmp_args_nm_4_ret : public gmp_args_base {
 public:
  gmp_args_nm_4_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
};



// 4 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3> class gmp_args_m_4 : public gmp_args_base {
 public:
  gmp_args_m_4 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
};



// 4 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename R> class gmp_args_m_4_ret : public gmp_args_base {
 public:
  gmp_args_m_4_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
};



// 5 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4> class gmp_args_nm_5 : public gmp_args_base {
 public:
  gmp_args_nm_5 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
};



// 5 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename R> class gmp_args_nm_5_ret : public gmp_args_base {
 public:
  gmp_args_nm_5_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
};



// 5 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4> class gmp_args_m_5 : public gmp_args_base {
 public:
  gmp_args_m_5 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
};



// 5 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename R> class gmp_args_m_5_ret : public gmp_args_base {
 public:
  gmp_args_m_5_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
};



// 6 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5> class gmp_args_nm_6 : public gmp_args_base {
 public:
  gmp_args_nm_6 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
};



// 6 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename R> class gmp_args_nm_6_ret : public gmp_args_base {
 public:
  gmp_args_nm_6_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
};



// 6 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5> class gmp_args_m_6 : public gmp_args_base {
 public:
  gmp_args_m_6 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
};



// 6 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename R> class gmp_args_m_6_ret : public gmp_args_base {
 public:
  gmp_args_m_6_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
};



// 7 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6> class gmp_args_nm_7 : public gmp_args_base {
 public:
  gmp_args_nm_7 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
};



// 7 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename R> class gmp_args_nm_7_ret : public gmp_args_base {
 public:
  gmp_args_nm_7_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
};



// 7 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6> class gmp_args_m_7 : public gmp_args_base {
 public:
  gmp_args_m_7 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
};



// 7 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename R> class gmp_args_m_7_ret : public gmp_args_base {
 public:
  gmp_args_m_7_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
};



// 8 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7> class gmp_args_nm_8 : public gmp_args_base {
 public:
  gmp_args_nm_8 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
};



// 8 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename R> class gmp_args_nm_8_ret : public gmp_args_base {
 public:
  gmp_args_nm_8_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
};



// 8 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7> class gmp_args_m_8 : public gmp_args_base {
 public:
  gmp_args_m_8 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
};



// 8 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename R> class gmp_args_m_8_ret : public gmp_args_base {
 public:
  gmp_args_m_8_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
};



// 9 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8> class gmp_args_nm_9 : public gmp_args_base {
 public:
  gmp_args_nm_9 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
};



// 9 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename R> class gmp_args_nm_9_ret : public gmp_args_base {
 public:
  gmp_args_nm_9_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
};



// 9 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8> class gmp_args_m_9 : public gmp_args_base {
 public:
  gmp_args_m_9 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
};



// 9 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename R> class gmp_args_m_9_ret : public gmp_args_base {
 public:
  gmp_args_m_9_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
};



// 10 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9> class gmp_args_nm_10 : public gmp_args_base {
 public:
  gmp_args_nm_10 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
};



// 10 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename R> class gmp_args_nm_10_ret : public gmp_args_base {
 public:
  gmp_args_nm_10_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
};



// 10 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9> class gmp_args_m_10 : public gmp_args_base {
 public:
  gmp_args_m_10 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
};



// 10 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename R> class gmp_args_m_10_ret : public gmp_args_base {
 public:
  gmp_args_m_10_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
};



// 11 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10> class gmp_args_nm_11 : public gmp_args_base {
 public:
  gmp_args_nm_11 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
};



// 11 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename R> class gmp_args_nm_11_ret : public gmp_args_base {
 public:
  gmp_args_nm_11_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
};



// 11 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10> class gmp_args_m_11 : public gmp_args_base {
 public:
  gmp_args_m_11 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
};



// 11 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename R> class gmp_args_m_11_ret : public gmp_args_base {
 public:
  gmp_args_m_11_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
};



// 12 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11> class gmp_args_nm_12 : public gmp_args_base {
 public:
  gmp_args_nm_12 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
};



// 12 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename R> class gmp_args_nm_12_ret : public gmp_args_base {
 public:
  gmp_args_nm_12_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
};



// 12 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11> class gmp_args_m_12 : public gmp_args_base {
 public:
  gmp_args_m_12 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
};



// 12 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename R> class gmp_args_m_12_ret : public gmp_args_base {
 public:
  gmp_args_m_12_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
};



// 13 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12> class gmp_args_nm_13 : public gmp_args_base {
 public:
  gmp_args_nm_13 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11), a12_ (a12)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
  A12 a12_;
};



// 13 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename R> class gmp_args_nm_13_ret : public gmp_args_base {
 public:
  gmp_args_nm_13_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11), a12_ (a12)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
  A12 a12_;
};



// 13 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12> class gmp_args_m_13 : public gmp_args_base {
 public:
  gmp_args_m_13 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11), a12_ (a12)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
  A12 a12_;
};



// 13 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename R> class gmp_args_m_13_ret : public gmp_args_base {
 public:
  gmp_args_m_13_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11), a12_ (a12)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
  A12 a12_;
};



// 14 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13> class gmp_args_nm_14 : public gmp_args_base {
 public:
  gmp_args_nm_14 (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13) :
    m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11), a12_ (a12), a13_ (a13)  {}

  void Run() {
    m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_, a13_);
  }

 private:
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
  A12 a12_;
  A13 a13_;
};



// 14 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13, typename R> class gmp_args_nm_14_ret : public gmp_args_base {
 public:
  gmp_args_nm_14_ret (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13, R* r) :
    m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11), a12_ (a12), a13_ (a13)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = m_ (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_, a13_);
  }

 private:
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
  A12 a12_;
  A13 a13_;
};



// 14 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13> class gmp_args_m_14 : public gmp_args_base {
 public:
  gmp_args_m_14 (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13) :
    o_ (o), m_ (m), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11), a12_ (a12), a13_ (a13)  {}

  void Run() {
    ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_, a13_);
  }

 private:
  C o_;
  M m_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
  A12 a12_;
  A13 a13_;
};



// 14 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13, typename R> class gmp_args_m_14_ret : public gmp_args_base {
 public:
  gmp_args_m_14_ret (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13, R* r) :
    o_ (o), m_ (m), r_ (r), a0_ (a0), a1_ (a1), a2_ (a2), a3_ (a3), a4_ (a4), a5_ (a5), a6_ (a6), a7_ (a7), a8_ (a8), a9_ (a9), a10_ (a10), a11_ (a11), a12_ (a12), a13_ (a13)  {}
  virtual bool returns_value() const {
    return true;
  }

  void Run() {
    *r_ = ((*o_).*m_) (a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_, a13_);
  }

 private:
  C o_;
  M m_;
  R* r_;
  A0 a0_;
  A1 a1_;
  A2 a2_;
  A3 a3_;
  A4 a4_;
  A5 a5_;
  A6 a6_;
  A7 a7_;
  A8 a8_;
  A9 a9_;
  A10 a10_;
  A11 a11_;
  A12 a12_;
  A13 a13_;
};






// 0 arguments --
template<typename M>
gmp_args_nm_0<M>* WrapTaskNM (M m) {
  return new gmp_args_nm_0<M>
    (m);
}

// 0 arguments --
template<typename M, typename R>
gmp_args_nm_0_ret<M, R>* WrapTaskNMRet (M m, R* r) {
  return new gmp_args_nm_0_ret<M, R>
    (m, r);
}

// 0 arguments --
template<typename C, typename M>
gmp_args_m_0<C, M>* WrapTask (C o, M m) {
  return new gmp_args_m_0<C, M>
    (o, m);
}

template<typename C, typename M>
GMPTask*
WrapTaskRefCounted (C o, M m) {
  GMPTask *t = WrapTask (o, m);
  return new RefCountTaskWrapper(t, o);
}

// 0 arguments --
template<typename C, typename M, typename R>
gmp_args_m_0_ret<C, M, R>* WrapTaskRet (C o, M m, R* r) {
  return new gmp_args_m_0_ret<C, M, R>
    (o, m, r);
}

// 1 arguments --
template<typename M, typename A0>
gmp_args_nm_1<M, A0>* WrapTaskNM (M m, A0 a0) {
  return new gmp_args_nm_1<M, A0>
    (m, a0);
}

// 1 arguments --
template<typename M, typename A0, typename R>
gmp_args_nm_1_ret<M, A0, R>* WrapTaskNMRet (M m, A0 a0, R* r) {
  return new gmp_args_nm_1_ret<M, A0, R>
    (m, a0, r);
}

// 1 arguments --
template<typename C, typename M, typename A0>
gmp_args_m_1<C, M, A0>* WrapTask (C o, M m, A0 a0) {
  return new gmp_args_m_1<C, M, A0>
    (o, m, a0);
}

template<typename C, typename M, typename A0>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0) {
  GMPTask *t = WrapTask (o, m, a0);
  return new RefCountTaskWrapper(t, o);
}

// 1 arguments --
template<typename C, typename M, typename A0, typename R>
gmp_args_m_1_ret<C, M, A0, R>* WrapTaskRet (C o, M m, A0 a0, R* r) {
  return new gmp_args_m_1_ret<C, M, A0, R>
    (o, m, a0, r);
}

// 2 arguments --
template<typename M, typename A0, typename A1>
gmp_args_nm_2<M, A0, A1>* WrapTaskNM (M m, A0 a0, A1 a1) {
  return new gmp_args_nm_2<M, A0, A1>
    (m, a0, a1);
}

// 2 arguments --
template<typename M, typename A0, typename A1, typename R>
gmp_args_nm_2_ret<M, A0, A1, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, R* r) {
  return new gmp_args_nm_2_ret<M, A0, A1, R>
    (m, a0, a1, r);
}

// 2 arguments --
template<typename C, typename M, typename A0, typename A1>
gmp_args_m_2<C, M, A0, A1>* WrapTask (C o, M m, A0 a0, A1 a1) {
  return new gmp_args_m_2<C, M, A0, A1>
    (o, m, a0, a1);
}

template<typename C, typename M, typename A0, typename A1>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1) {
  GMPTask *t = WrapTask (o, m, a0, a1);
  return new RefCountTaskWrapper(t, o);
}

// 2 arguments --
template<typename C, typename M, typename A0, typename A1, typename R>
gmp_args_m_2_ret<C, M, A0, A1, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, R* r) {
  return new gmp_args_m_2_ret<C, M, A0, A1, R>
    (o, m, a0, a1, r);
}

// 3 arguments --
template<typename M, typename A0, typename A1, typename A2>
gmp_args_nm_3<M, A0, A1, A2>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2) {
  return new gmp_args_nm_3<M, A0, A1, A2>
    (m, a0, a1, a2);
}

// 3 arguments --
template<typename M, typename A0, typename A1, typename A2, typename R>
gmp_args_nm_3_ret<M, A0, A1, A2, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, R* r) {
  return new gmp_args_nm_3_ret<M, A0, A1, A2, R>
    (m, a0, a1, a2, r);
}

// 3 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2>
gmp_args_m_3<C, M, A0, A1, A2>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2) {
  return new gmp_args_m_3<C, M, A0, A1, A2>
    (o, m, a0, a1, a2);
}

template<typename C, typename M, typename A0, typename A1, typename A2>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2);
  return new RefCountTaskWrapper(t, o);
}

// 3 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename R>
gmp_args_m_3_ret<C, M, A0, A1, A2, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, R* r) {
  return new gmp_args_m_3_ret<C, M, A0, A1, A2, R>
    (o, m, a0, a1, a2, r);
}

// 4 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3>
gmp_args_nm_4<M, A0, A1, A2, A3>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3) {
  return new gmp_args_nm_4<M, A0, A1, A2, A3>
    (m, a0, a1, a2, a3);
}

// 4 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename R>
gmp_args_nm_4_ret<M, A0, A1, A2, A3, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, R* r) {
  return new gmp_args_nm_4_ret<M, A0, A1, A2, A3, R>
    (m, a0, a1, a2, a3, r);
}

// 4 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3>
gmp_args_m_4<C, M, A0, A1, A2, A3>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3) {
  return new gmp_args_m_4<C, M, A0, A1, A2, A3>
    (o, m, a0, a1, a2, a3);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3);
  return new RefCountTaskWrapper(t, o);
}

// 4 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename R>
gmp_args_m_4_ret<C, M, A0, A1, A2, A3, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, R* r) {
  return new gmp_args_m_4_ret<C, M, A0, A1, A2, A3, R>
    (o, m, a0, a1, a2, a3, r);
}

// 5 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4>
gmp_args_nm_5<M, A0, A1, A2, A3, A4>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4) {
  return new gmp_args_nm_5<M, A0, A1, A2, A3, A4>
    (m, a0, a1, a2, a3, a4);
}

// 5 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename R>
gmp_args_nm_5_ret<M, A0, A1, A2, A3, A4, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, R* r) {
  return new gmp_args_nm_5_ret<M, A0, A1, A2, A3, A4, R>
    (m, a0, a1, a2, a3, a4, r);
}

// 5 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4>
gmp_args_m_5<C, M, A0, A1, A2, A3, A4>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4) {
  return new gmp_args_m_5<C, M, A0, A1, A2, A3, A4>
    (o, m, a0, a1, a2, a3, a4);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4);
  return new RefCountTaskWrapper(t, o);
}

// 5 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename R>
gmp_args_m_5_ret<C, M, A0, A1, A2, A3, A4, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, R* r) {
  return new gmp_args_m_5_ret<C, M, A0, A1, A2, A3, A4, R>
    (o, m, a0, a1, a2, a3, a4, r);
}

// 6 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5>
gmp_args_nm_6<M, A0, A1, A2, A3, A4, A5>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5) {
  return new gmp_args_nm_6<M, A0, A1, A2, A3, A4, A5>
    (m, a0, a1, a2, a3, a4, a5);
}

// 6 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename R>
gmp_args_nm_6_ret<M, A0, A1, A2, A3, A4, A5, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, R* r) {
  return new gmp_args_nm_6_ret<M, A0, A1, A2, A3, A4, A5, R>
    (m, a0, a1, a2, a3, a4, a5, r);
}

// 6 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5>
gmp_args_m_6<C, M, A0, A1, A2, A3, A4, A5>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5) {
  return new gmp_args_m_6<C, M, A0, A1, A2, A3, A4, A5>
    (o, m, a0, a1, a2, a3, a4, a5);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5);
  return new RefCountTaskWrapper(t, o);
}

// 6 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename R>
gmp_args_m_6_ret<C, M, A0, A1, A2, A3, A4, A5, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, R* r) {
  return new gmp_args_m_6_ret<C, M, A0, A1, A2, A3, A4, A5, R>
    (o, m, a0, a1, a2, a3, a4, a5, r);
}

// 7 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6>
gmp_args_nm_7<M, A0, A1, A2, A3, A4, A5, A6>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6) {
  return new gmp_args_nm_7<M, A0, A1, A2, A3, A4, A5, A6>
    (m, a0, a1, a2, a3, a4, a5, a6);
}

// 7 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename R>
gmp_args_nm_7_ret<M, A0, A1, A2, A3, A4, A5, A6, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, R* r) {
  return new gmp_args_nm_7_ret<M, A0, A1, A2, A3, A4, A5, A6, R>
    (m, a0, a1, a2, a3, a4, a5, a6, r);
}

// 7 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6>
gmp_args_m_7<C, M, A0, A1, A2, A3, A4, A5, A6>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6) {
  return new gmp_args_m_7<C, M, A0, A1, A2, A3, A4, A5, A6>
    (o, m, a0, a1, a2, a3, a4, a5, a6);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5, a6);
  return new RefCountTaskWrapper(t, o);
}

// 7 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename R>
gmp_args_m_7_ret<C, M, A0, A1, A2, A3, A4, A5, A6, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, R* r) {
  return new gmp_args_m_7_ret<C, M, A0, A1, A2, A3, A4, A5, A6, R>
    (o, m, a0, a1, a2, a3, a4, a5, a6, r);
}

// 8 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7>
gmp_args_nm_8<M, A0, A1, A2, A3, A4, A5, A6, A7>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7) {
  return new gmp_args_nm_8<M, A0, A1, A2, A3, A4, A5, A6, A7>
    (m, a0, a1, a2, a3, a4, a5, a6, a7);
}

// 8 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename R>
gmp_args_nm_8_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, R* r) {
  return new gmp_args_nm_8_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, R>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, r);
}

// 8 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7>
gmp_args_m_8<C, M, A0, A1, A2, A3, A4, A5, A6, A7>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7) {
  return new gmp_args_m_8<C, M, A0, A1, A2, A3, A4, A5, A6, A7>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5, a6, a7);
  return new RefCountTaskWrapper(t, o);
}

// 8 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename R>
gmp_args_m_8_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, R* r) {
  return new gmp_args_m_8_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, R>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, r);
}

// 9 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8>
gmp_args_nm_9<M, A0, A1, A2, A3, A4, A5, A6, A7, A8>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8) {
  return new gmp_args_nm_9<M, A0, A1, A2, A3, A4, A5, A6, A7, A8>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8);
}

// 9 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename R>
gmp_args_nm_9_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, R* r) {
  return new gmp_args_nm_9_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, R>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, r);
}

// 9 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8>
gmp_args_m_9<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8) {
  return new gmp_args_m_9<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8);
  return new RefCountTaskWrapper(t, o);
}

// 9 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename R>
gmp_args_m_9_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, R* r) {
  return new gmp_args_m_9_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, R>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, r);
}

// 10 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9>
gmp_args_nm_10<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9) {
  return new gmp_args_nm_10<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
}

// 10 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename R>
gmp_args_nm_10_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, R* r) {
  return new gmp_args_nm_10_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, r);
}

// 10 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9>
gmp_args_m_10<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9) {
  return new gmp_args_m_10<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
  return new RefCountTaskWrapper(t, o);
}

// 10 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename R>
gmp_args_m_10_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, R* r) {
  return new gmp_args_m_10_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, r);
}

// 11 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10>
gmp_args_nm_11<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10) {
  return new gmp_args_nm_11<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10);
}

// 11 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename R>
gmp_args_nm_11_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, R* r) {
  return new gmp_args_nm_11_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, r);
}

// 11 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10>
gmp_args_m_11<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10) {
  return new gmp_args_m_11<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10);
  return new RefCountTaskWrapper(t, o);
}

// 11 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename R>
gmp_args_m_11_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, R* r) {
  return new gmp_args_m_11_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, r);
}

// 12 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11>
gmp_args_nm_12<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11) {
  return new gmp_args_nm_12<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11);
}

// 12 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename R>
gmp_args_nm_12_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, R* r) {
  return new gmp_args_nm_12_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, r);
}

// 12 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11>
gmp_args_m_12<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11) {
  return new gmp_args_m_12<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11);
  return new RefCountTaskWrapper(t, o);
}

// 12 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename R>
gmp_args_m_12_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, R* r) {
  return new gmp_args_m_12_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, r);
}

// 13 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12>
gmp_args_nm_13<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12) {
  return new gmp_args_nm_13<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12);
}

// 13 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename R>
gmp_args_nm_13_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, R* r) {
  return new gmp_args_nm_13_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, r);
}

// 13 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12>
gmp_args_m_13<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12) {
  return new gmp_args_m_13<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12);
  return new RefCountTaskWrapper(t, o);
}

// 13 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename R>
gmp_args_m_13_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, R* r) {
  return new gmp_args_m_13_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, r);
}

// 14 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13>
gmp_args_nm_14<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13>* WrapTaskNM (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13) {
  return new gmp_args_nm_14<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13);
}

// 14 arguments --
template<typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13, typename R>
gmp_args_nm_14_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R>* WrapTaskNMRet (M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13, R* r) {
  return new gmp_args_nm_14_ret<M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R>
    (m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, r);
}

// 14 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13>
gmp_args_m_14<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13>* WrapTask (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13) {
  return new gmp_args_m_14<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13);
}

template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13>
GMPTask*
WrapTaskRefCounted (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13) {
  GMPTask *t = WrapTask (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13);
  return new RefCountTaskWrapper(t, o);
}

// 14 arguments --
template<typename C, typename M, typename A0, typename A1, typename A2, typename A3, typename A4, typename A5, typename A6, typename A7, typename A8, typename A9, typename A10, typename A11, typename A12, typename A13, typename R>
gmp_args_m_14_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R>* WrapTaskRet (C o, M m, A0 a0, A1 a1, A2 a2, A3 a3, A4 a4, A5 a5, A6 a6, A7 a7, A8 a8, A9 a9, A10 a10, A11 a11, A12 a12, A13 a13, R* r) {
  return new gmp_args_m_14_ret<C, M, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R>
    (o, m, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, r);
}

