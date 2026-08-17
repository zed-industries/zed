// sigslot.h: Signal/Slot classes
//
// Written by Sarah Thompson (sarah@telergy.com) 2002.
//
// License: Public domain. You are free to use this code however you like, with
// the proviso that the author takes on no responsibility or liability for any
// use.
//
// QUICK DOCUMENTATION
//
//        (see also the full documentation at http://sigslot.sourceforge.net/)
//
//    #define switches
//      SIGSLOT_PURE_ISO:
//        Define this to force ISO C++ compliance. This also disables all of
//        the thread safety support on platforms where it is available.
//
//      SIGSLOT_USE_POSIX_THREADS:
//        Force use of Posix threads when using a C++ compiler other than gcc
//        on a platform that supports Posix threads. (When using gcc, this is
//        the default - use SIGSLOT_PURE_ISO to disable this if necessary)
//
//      SIGSLOT_DEFAULT_MT_POLICY:
//        Where thread support is enabled, this defaults to
//        multi_threaded_global. Otherwise, the default is single_threaded.
//        #define this yourself to override the default. In pure ISO mode,
//        anything other than single_threaded will cause a compiler error.
//
//    PLATFORM NOTES
//
//      Win32:
//        On Win32, the WEBRTC_WIN symbol must be #defined. Most mainstream
//        compilers do this by default, but you may need to define it yourself
//        if your build environment is less standard. This causes the Win32
//        thread support to be compiled in and used automatically.
//
//      Unix/Linux/BSD, etc.:
//        If you're using gcc, it is assumed that you have Posix threads
//        available, so they are used automatically. You can override this (as
//        under Windows) with the SIGSLOT_PURE_ISO switch. If you're using
//        something other than gcc but still want to use Posix threads, you
//        need to #define SIGSLOT_USE_POSIX_THREADS.
//
//      ISO C++:
//        If none of the supported platforms are detected, or if
//        SIGSLOT_PURE_ISO is defined, all multithreading support is turned
//        off, along with any code that might cause a pure ISO C++ environment
//        to complain. Before you ask, gcc -ansi -pedantic won't compile this
//        library, but gcc -ansi is fine. Pedantic mode seems to throw a lot of
//        errors that aren't really there. If you feel like investigating this,
//        please contact the author.
//
//
//    THREADING MODES
//
//      single_threaded:
//        Your program is assumed to be single threaded from the point of view
//        of signal/slot usage (i.e. all objects using signals and slots are
//        created and destroyed from a single thread). Behaviour if objects are
//        destroyed concurrently is undefined (i.e. you'll get the occasional
//        segmentation fault/memory exception).
//
//      multi_threaded_global:
//        Your program is assumed to be multi threaded. Objects using signals
//        and slots can be safely created and destroyed from any thread, even
//        when connections exist. In multi_threaded_global mode, this is
//        achieved by a single global mutex (actually a critical section on
//        Windows because they are faster). This option uses less OS resources,
//        but results in more opportunities for contention, possibly resulting
//        in more context switches than are strictly necessary.
//
//      multi_threaded_local:
//        Behaviour in this mode is essentially the same as
//        multi_threaded_global, except that each signal, and each object that
//        inherits has_slots, all have their own mutex/critical section. In
//        practice, this means that mutex collisions (and hence context
//        switches) only happen if they are absolutely essential. However, on
//        some platforms, creating a lot of mutexes can slow down the whole OS,
//        so use this option with care.
//
//    USING THE LIBRARY
//
//      See the full documentation at http://sigslot.sourceforge.net/
//
// Libjingle specific:
//
// This file has been modified such that has_slots and signalx do not have to be
// using the same threading requirements. E.g. it is possible to connect a
// has_slots<single_threaded> and signal0<multi_threaded_local> or
// has_slots<multi_threaded_local> and signal0<single_threaded>.
// If has_slots is single threaded the user must ensure that it is not trying
// to connect or disconnect to signalx concurrently or data race may occur.
// If signalx is single threaded the user must ensure that disconnect, connect
// or signal is not happening concurrently or data race may occur.

#ifndef RTC_BASE_THIRD_PARTY_SIGSLOT_SIGSLOT_H_
#define RTC_BASE_THIRD_PARTY_SIGSLOT_SIGSLOT_H_

#include <cstring>
#include <list>
#include <set>

// On our copy of sigslot.h, we set single threading as default.
#define SIGSLOT_DEFAULT_MT_POLICY single_threaded

#if defined(SIGSLOT_PURE_ISO) ||                   \
    (!defined(WEBRTC_WIN) && !defined(__GNUG__) && \
     !defined(SIGSLOT_USE_POSIX_THREADS))
#define _SIGSLOT_SINGLE_THREADED
#elif defined(WEBRTC_WIN)
#define _SIGSLOT_HAS_WIN32_THREADS
#include "windows.h"
#elif defined(__GNUG__) || defined(SIGSLOT_USE_POSIX_THREADS)
#define _SIGSLOT_HAS_POSIX_THREADS
#include <pthread.h>
#else
#define _SIGSLOT_SINGLE_THREADED
#endif

#ifndef SIGSLOT_DEFAULT_MT_POLICY
#ifdef _SIGSLOT_SINGLE_THREADED
#define SIGSLOT_DEFAULT_MT_POLICY single_threaded
#else
#define SIGSLOT_DEFAULT_MT_POLICY multi_threaded_local
#endif
#endif

// TODO: change this namespace to rtc?
namespace sigslot {

class single_threaded {
 public:
  void lock() {}
  void unlock() {}
};

#ifdef _SIGSLOT_HAS_WIN32_THREADS
// The multi threading policies only get compiled in if they are enabled.
class multi_threaded_global {
 public:
  multi_threaded_global() {
    static bool isinitialised = false;

    if (!isinitialised) {
      InitializeCriticalSection(get_critsec());
      isinitialised = true;
    }
  }

  void lock() { EnterCriticalSection(get_critsec()); }

  void unlock() { LeaveCriticalSection(get_critsec()); }

 private:
  CRITICAL_SECTION* get_critsec() {
    static CRITICAL_SECTION g_critsec;
    return &g_critsec;
  }
};

class multi_threaded_local {
 public:
  multi_threaded_local() { InitializeCriticalSection(&m_critsec); }

  multi_threaded_local(const multi_threaded_local&) {
    InitializeCriticalSection(&m_critsec);
  }

  ~multi_threaded_local() { DeleteCriticalSection(&m_critsec); }

  void lock() { EnterCriticalSection(&m_critsec); }

  void unlock() { LeaveCriticalSection(&m_critsec); }

 private:
  CRITICAL_SECTION m_critsec;
};
#endif  // _SIGSLOT_HAS_WIN32_THREADS

#ifdef _SIGSLOT_HAS_POSIX_THREADS
// The multi threading policies only get compiled in if they are enabled.
class multi_threaded_global {
 public:
  void lock() { pthread_mutex_lock(get_mutex()); }
  void unlock() { pthread_mutex_unlock(get_mutex()); }

 private:
  static pthread_mutex_t* get_mutex();
};

class multi_threaded_local {
 public:
  multi_threaded_local() { pthread_mutex_init(&m_mutex, nullptr); }
  multi_threaded_local(const multi_threaded_local&) {
    pthread_mutex_init(&m_mutex, nullptr);
  }
  ~multi_threaded_local() { pthread_mutex_destroy(&m_mutex); }
  void lock() { pthread_mutex_lock(&m_mutex); }
  void unlock() { pthread_mutex_unlock(&m_mutex); }

 private:
  pthread_mutex_t m_mutex;
};
#endif  // _SIGSLOT_HAS_POSIX_THREADS

template <class mt_policy>
class lock_block {
 public:
  mt_policy* m_mutex;

  lock_block(mt_policy* mtx) : m_mutex(mtx) { m_mutex->lock(); }

  ~lock_block() { m_mutex->unlock(); }
};

class _signal_base_interface;

class has_slots_interface {
 private:
  typedef void (*signal_connect_t)(has_slots_interface* self,
                                   _signal_base_interface* sender);
  typedef void (*signal_disconnect_t)(has_slots_interface* self,
                                      _signal_base_interface* sender);
  typedef void (*disconnect_all_t)(has_slots_interface* self);

  const signal_connect_t m_signal_connect;
  const signal_disconnect_t m_signal_disconnect;
  const disconnect_all_t m_disconnect_all;

 protected:
  has_slots_interface(signal_connect_t conn,
                      signal_disconnect_t disc,
                      disconnect_all_t disc_all)
      : m_signal_connect(conn),
        m_signal_disconnect(disc),
        m_disconnect_all(disc_all) {}

  // Doesn't really need to be virtual, but is for backwards compatibility
  // (it was virtual in a previous version of sigslot).
  virtual ~has_slots_interface() {}

 public:
  void signal_connect(_signal_base_interface* sender) {
    m_signal_connect(this, sender);
  }

  void signal_disconnect(_signal_base_interface* sender) {
    m_signal_disconnect(this, sender);
  }

  void disconnect_all() { m_disconnect_all(this); }
};

class _signal_base_interface {
 private:
  typedef void (*slot_disconnect_t)(_signal_base_interface* self,
                                    has_slots_interface* pslot);
  typedef void (*slot_duplicate_t)(_signal_base_interface* self,
                                   const has_slots_interface* poldslot,
                                   has_slots_interface* pnewslot);

  const slot_disconnect_t m_slot_disconnect;
  const slot_duplicate_t m_slot_duplicate;

 protected:
  _signal_base_interface(slot_disconnect_t disc, slot_duplicate_t dupl)
      : m_slot_disconnect(disc), m_slot_duplicate(dupl) {}

  ~_signal_base_interface() {}

 public:
  void slot_disconnect(has_slots_interface* pslot) {
    m_slot_disconnect(this, pslot);
  }

  void slot_duplicate(const has_slots_interface* poldslot,
                      has_slots_interface* pnewslot) {
    m_slot_duplicate(this, poldslot, pnewslot);
  }
};

class _opaque_connection {
 private:
  typedef void (*emit_t)(const _opaque_connection*);
  template <typename FromT, typename ToT>
  union union_caster {
    FromT from;
    ToT to;
  };

  emit_t pemit;
  has_slots_interface* pdest;
  // Pointers to member functions may be up to 16 bytes (24 bytes for MSVC)
  // for virtual classes, so make sure we have enough space to store it.
#if defined(_MSC_VER) && !defined(__clang__)
  unsigned char pmethod[24];
#else
  unsigned char pmethod[16];
#endif

 public:
  template <typename DestT, typename... Args>
  _opaque_connection(DestT* pd, void (DestT::*pm)(Args...)) : pdest(pd) {
    typedef void (DestT::*pm_t)(Args...);
    static_assert(sizeof(pm_t) <= sizeof(pmethod),
                  "Size of slot function pointer too large.");

    std::memcpy(pmethod, &pm, sizeof(pm_t));

    typedef void (*em_t)(const _opaque_connection* self, Args...);
    union_caster<em_t, emit_t> caster2;
    caster2.from = &_opaque_connection::emitter<DestT, Args...>;
    pemit = caster2.to;
  }

  has_slots_interface* getdest() const { return pdest; }

  _opaque_connection duplicate(has_slots_interface* newtarget) const {
    _opaque_connection res = *this;
    res.pdest = newtarget;
    return res;
  }

  // Just calls the stored "emitter" function pointer stored at construction
  // time.
  template <typename... Args>
  void emit(Args... args) const {
    typedef void (*em_t)(const _opaque_connection*, Args...);
    union_caster<emit_t, em_t> caster;
    caster.from = pemit;
    (caster.to)(this, args...);
  }

 private:
  template <typename DestT, typename... Args>
  static void emitter(const _opaque_connection* self, Args... args) {
    typedef void (DestT::*pm_t)(Args...);
    pm_t pm;
    static_assert(sizeof(pm_t) <= sizeof(pmethod),
                  "Size of slot function pointer too large.");
    std::memcpy(&pm, self->pmethod, sizeof(pm_t));
    (static_cast<DestT*>(self->pdest)->*(pm))(args...);
  }
};

template <class mt_policy>
class _signal_base : public _signal_base_interface, public mt_policy {
 protected:
  typedef std::list<_opaque_connection> connections_list;

  _signal_base()
      : _signal_base_interface(&_signal_base::do_slot_disconnect,
                               &_signal_base::do_slot_duplicate),
        m_current_iterator(m_connected_slots.end()) {}

  ~_signal_base() { disconnect_all(); }

 private:
  _signal_base& operator=(_signal_base const& that);

 public:
  _signal_base(const _signal_base& o)
      : _signal_base_interface(&_signal_base::do_slot_disconnect,
                               &_signal_base::do_slot_duplicate),
        m_current_iterator(m_connected_slots.end()) {
    lock_block<mt_policy> lock(this);
    for (const auto& connection : o.m_connected_slots) {
      connection.getdest()->signal_connect(this);
      m_connected_slots.push_back(connection);
    }
  }

  bool is_empty() {
    lock_block<mt_policy> lock(this);
    return m_connected_slots.empty();
  }

  void disconnect_all() {
    lock_block<mt_policy> lock(this);

    while (!m_connected_slots.empty()) {
      has_slots_interface* pdest = m_connected_slots.front().getdest();
      m_connected_slots.pop_front();
      pdest->signal_disconnect(static_cast<_signal_base_interface*>(this));
    }
    // If disconnect_all is called while the signal is firing, advance the
    // current slot iterator to the end to avoid an invalidated iterator from
    // being dereferenced.
    m_current_iterator = m_connected_slots.end();
  }

#if !defined(NDEBUG)
  bool connected(has_slots_interface* pclass) {
    lock_block<mt_policy> lock(this);
    connections_list::const_iterator it = m_connected_slots.begin();
    connections_list::const_iterator itEnd = m_connected_slots.end();
    while (it != itEnd) {
      if (it->getdest() == pclass)
        return true;
      ++it;
    }
    return false;
  }
#endif

  void disconnect(has_slots_interface* pclass) {
    lock_block<mt_policy> lock(this);
    connections_list::iterator it = m_connected_slots.begin();
    connections_list::iterator itEnd = m_connected_slots.end();

    while (it != itEnd) {
      if (it->getdest() == pclass) {
        // If we're currently using this iterator because the signal is firing,
        // advance it to avoid it being invalidated.
        if (m_current_iterator == it) {
          m_current_iterator = m_connected_slots.erase(it);
        } else {
          m_connected_slots.erase(it);
        }
        pclass->signal_disconnect(static_cast<_signal_base_interface*>(this));
        return;
      }
      ++it;
    }
  }

 private:
  static void do_slot_disconnect(_signal_base_interface* p,
                                 has_slots_interface* pslot) {
    _signal_base* const self = static_cast<_signal_base*>(p);
    lock_block<mt_policy> lock(self);
    connections_list::iterator it = self->m_connected_slots.begin();
    connections_list::iterator itEnd = self->m_connected_slots.end();

    while (it != itEnd) {
      connections_list::iterator itNext = it;
      ++itNext;

      if (it->getdest() == pslot) {
        // If we're currently using this iterator because the signal is firing,
        // advance it to avoid it being invalidated.
        if (self->m_current_iterator == it) {
          self->m_current_iterator = self->m_connected_slots.erase(it);
        } else {
          self->m_connected_slots.erase(it);
        }
      }

      it = itNext;
    }
  }

  static void do_slot_duplicate(_signal_base_interface* p,
                                const has_slots_interface* oldtarget,
                                has_slots_interface* newtarget) {
    _signal_base* const self = static_cast<_signal_base*>(p);
    lock_block<mt_policy> lock(self);
    connections_list::iterator it = self->m_connected_slots.begin();
    connections_list::iterator itEnd = self->m_connected_slots.end();

    while (it != itEnd) {
      if (it->getdest() == oldtarget) {
        self->m_connected_slots.push_back(it->duplicate(newtarget));
      }

      ++it;
    }
  }

 protected:
  connections_list m_connected_slots;

  // Used to handle a slot being disconnected while a signal is
  // firing (iterating m_connected_slots).
  connections_list::iterator m_current_iterator;
  bool m_erase_current_iterator = false;
};

template <class mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
class has_slots : public has_slots_interface, public mt_policy {
 private:
  typedef std::set<_signal_base_interface*> sender_set;
  typedef sender_set::const_iterator const_iterator;

 public:
  has_slots()
      : has_slots_interface(&has_slots::do_signal_connect,
                            &has_slots::do_signal_disconnect,
                            &has_slots::do_disconnect_all) {}

  has_slots(has_slots const& o)
      : has_slots_interface(&has_slots::do_signal_connect,
                            &has_slots::do_signal_disconnect,
                            &has_slots::do_disconnect_all) {
    lock_block<mt_policy> lock(this);
    for (auto* sender : o.m_senders) {
      sender->slot_duplicate(&o, this);
      m_senders.insert(sender);
    }
  }

  ~has_slots() { this->disconnect_all(); }

 private:
  has_slots& operator=(has_slots const&);

  static void do_signal_connect(has_slots_interface* p,
                                _signal_base_interface* sender) {
    has_slots* const self = static_cast<has_slots*>(p);
    lock_block<mt_policy> lock(self);
    self->m_senders.insert(sender);
  }

  static void do_signal_disconnect(has_slots_interface* p,
                                   _signal_base_interface* sender) {
    has_slots* const self = static_cast<has_slots*>(p);
    lock_block<mt_policy> lock(self);
    self->m_senders.erase(sender);
  }

  static void do_disconnect_all(has_slots_interface* p) {
    has_slots* const self = static_cast<has_slots*>(p);
    lock_block<mt_policy> lock(self);
    while (!self->m_senders.empty()) {
      std::set<_signal_base_interface*> senders;
      senders.swap(self->m_senders);
      const_iterator it = senders.begin();
      const_iterator itEnd = senders.end();

      while (it != itEnd) {
        _signal_base_interface* s = *it;
        ++it;
        s->slot_disconnect(p);
      }
    }
  }

 private:
  sender_set m_senders;
};

template <class mt_policy, typename... Args>
class signal_with_thread_policy : public _signal_base<mt_policy> {
 private:
  typedef _signal_base<mt_policy> base;

 protected:
  typedef typename base::connections_list connections_list;

 public:
  signal_with_thread_policy() {}

  template <class desttype>
  void connect(desttype* pclass, void (desttype::*pmemfun)(Args...)) {
    lock_block<mt_policy> lock(this);
    this->m_connected_slots.push_back(_opaque_connection(pclass, pmemfun));
    pclass->signal_connect(static_cast<_signal_base_interface*>(this));
  }

  void emit(Args... args) {
    lock_block<mt_policy> lock(this);
    this->m_current_iterator = this->m_connected_slots.begin();
    while (this->m_current_iterator != this->m_connected_slots.end()) {
      _opaque_connection const& conn = *this->m_current_iterator;
      ++(this->m_current_iterator);
      conn.emit<Args...>(args...);
    }
  }

  void operator()(Args... args) { emit(args...); }
};

// Alias with default thread policy. Needed because both default arguments
// and variadic template arguments must go at the end of the list, so we
// can't have both at once.
template <typename... Args>
using signal = signal_with_thread_policy<SIGSLOT_DEFAULT_MT_POLICY, Args...>;

// The previous verion of sigslot didn't use variadic templates, so you would
// need to write "sigslot::signal2<Arg1, Arg2>", for example.
// Now you can just write "sigslot::signal<Arg1, Arg2>", but these aliases
// exist for backwards compatibility.
template <typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal0 = signal_with_thread_policy<mt_policy>;

template <typename A1, typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal1 = signal_with_thread_policy<mt_policy, A1>;

template <typename A1,
          typename A2,
          typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal2 = signal_with_thread_policy<mt_policy, A1, A2>;

template <typename A1,
          typename A2,
          typename A3,
          typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal3 = signal_with_thread_policy<mt_policy, A1, A2, A3>;

template <typename A1,
          typename A2,
          typename A3,
          typename A4,
          typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal4 = signal_with_thread_policy<mt_policy, A1, A2, A3, A4>;

template <typename A1,
          typename A2,
          typename A3,
          typename A4,
          typename A5,
          typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal5 = signal_with_thread_policy<mt_policy, A1, A2, A3, A4, A5>;

template <typename A1,
          typename A2,
          typename A3,
          typename A4,
          typename A5,
          typename A6,
          typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal6 = signal_with_thread_policy<mt_policy, A1, A2, A3, A4, A5, A6>;

template <typename A1,
          typename A2,
          typename A3,
          typename A4,
          typename A5,
          typename A6,
          typename A7,
          typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal7 =
    signal_with_thread_policy<mt_policy, A1, A2, A3, A4, A5, A6, A7>;

template <typename A1,
          typename A2,
          typename A3,
          typename A4,
          typename A5,
          typename A6,
          typename A7,
          typename A8,
          typename mt_policy = SIGSLOT_DEFAULT_MT_POLICY>
using signal8 =
    signal_with_thread_policy<mt_policy, A1, A2, A3, A4, A5, A6, A7, A8>;

}  // namespace sigslot

#endif /* RTC_BASE_THIRD_PARTY_SIGSLOT_SIGSLOT_H_ */
