
#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_EVENT_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_EVENT_H_

#include <cstdint>

// Renaming the user space API types to fixed integer typedefs of C++.

enum perf_type_id {
  PERF_TYPE_HARDWARE = 0,
  PERF_TYPE_SOFTWARE = 1,
  PERF_TYPE_TRACEPOINT = 2,
  PERF_TYPE_HW_CACHE = 3,
  PERF_TYPE_RAW = 4,
  PERF_TYPE_BREAKPOINT = 5,

  PERF_TYPE_MAX, /* non-ABI */
};

/*
 * Hardware event_id to monitor via a performance monitoring event:
 *
 * @sample_max_stack: Max number of frame pointers in a callchain,
 * should be < /proc/sys/kernel/perf_event_max_stack
 */
struct perf_event_attr {
  /*
   * Major type: hardware/software/tracepoint/etc.
   */
  uint32_t type;

  /*
   * Size of the attr structure, for fwd/bwd compat.
   */
  uint32_t size;

  /*
   * Type specific configuration information.
   */
  uint64_t config;

  union {
    uint64_t sample_period;
    uint64_t sample_freq;
  };

  uint64_t sample_type;
  uint64_t read_format;

  uint64_t disabled : 1,  /* off by default        */
      inherit : 1,        /* children inherit it   */
      pinned : 1,         /* must always be on PMU */
      exclusive : 1,      /* only group on PMU     */
      exclude_user : 1,   /* don't count user      */
      exclude_kernel : 1, /* ditto kernel          */
      exclude_hv : 1,     /* ditto hypervisor      */
      exclude_idle : 1,   /* don't count when idle */
      mmap : 1,           /* include mmap data     */
      comm : 1,           /* include comm data     */
      freq : 1,           /* use freq, not period  */
      inherit_stat : 1,   /* per task counts       */
      enable_on_exec : 1, /* next exec enables     */
      task : 1,           /* trace fork/exit       */
      watermark : 1,      /* wakeup_watermark      */
      /*
       * precise_ip:
       *
       *  0 - SAMPLE_IP can have arbitrary skid
       *  1 - SAMPLE_IP must have constant skid
       *  2 - SAMPLE_IP requested to have 0 skid
       *  3 - SAMPLE_IP must have 0 skid
       *
       *  See also PERF_RECORD_MISC_EXACT_IP
       */
      precise_ip : 2,    /* skid constraint       */
      mmap_data : 1,     /* non-exec mmap data    */
      sample_id_all : 1, /* sample_type all events */

      exclude_host : 1,  /* don't count in host   */
      exclude_guest : 1, /* don't count in guest  */

      exclude_callchain_kernel : 1, /* exclude kernel callchains */
      exclude_callchain_user : 1,   /* exclude user callchains */
      mmap2 : 1,                    /* include mmap with inode data     */
      comm_exec : 1,      /* flag comm events that are due to an exec */
      use_clockid : 1,    /* use @clockid for time fields */
      context_switch : 1, /* context switch data */
      write_backward : 1, /* Write ring buffer from end to beginning */
      namespaces : 1,     /* include namespaces data */
      ksymbol : 1,        /* include ksymbol events */
      bpf_event : 1,      /* include bpf events */
      aux_output : 1,     /* generate AUX records instead of events */
      cgroup : 1,         /* include cgroup events */
      text_poke : 1,      /* include text poke events */
      build_id : 1,       /* use build id in mmap2 events */
      inherit_thread : 1, /* children only inherit if cloned with CLONE_THREAD
                           */
      remove_on_exec : 1, /* event is removed from task on exec */
      sigtrap : 1,        /* send synchronous SIGTRAP on event */
      __reserved_1 : 26;

  union {
    uint32_t wakeup_events;    /* wakeup every n events */
    uint32_t wakeup_watermark; /* bytes before wakeup   */
  };

  uint32_t bp_type;
  union {
    uint64_t bp_addr;
    uint64_t kprobe_func; /* for perf_kprobe */
    uint64_t uprobe_path; /* for perf_uprobe */
    uint64_t config1;     /* extension of config */
  };
  union {
    uint64_t bp_len;
    uint64_t kprobe_addr;  /* when kprobe_func == NULL */
    uint64_t probe_offset; /* for perf_[k,u]probe */
    uint64_t config2;      /* extension of config1 */
  };
  uint64_t branch_sample_type; /* enum perf_branch_sample_type */

  /*
   * Defines set of user regs to dump on samples.
   * See asm/perf_regs.h for details.
   */
  uint64_t sample_regs_user;

  /*
   * Defines size of the user stack to dump on samples.
   */
  uint32_t sample_stack_user;

  int32_t clockid;
  /*
   * Defines set of regs to dump for each sample
   * state captured on:
   *  - precise = 0: PMU interrupt
   *  - precise > 0: sampled instruction
   *
   * See asm/perf_regs.h for details.
   */
  uint64_t sample_regs_intr;

  /*
   * Wakeup watermark for AUX area
   */
  uint32_t aux_watermark;
  uint16_t sample_max_stack;
  uint16_t __reserved_2;
  uint32_t aux_sample_size;
  uint32_t __reserved_3;

  /*
   * User provided data if sigtrap=1, passed back to user via
   * siginfo_t::si_perf_data, e.g. to permit user to identify the event.
   * Note, siginfo_t::si_perf_data is long-sized, and sig_data will be
   * truncated accordingly on 32 bit architectures.
   */
  uint64_t sig_data;

  uint64_t config3; /* extension of config2 */
};

struct perf_event_header {
  // Value from `perf_event_type`
  uint32_t type;
  uint16_t misc;
  uint16_t size;
};

enum perf_event_type {
  PERF_RECORD_MMAP = 1,
  PERF_RECORD_LOST = 2,
  PERF_RECORD_COMM = 3,
  PERF_RECORD_EXIT = 4,
  PERF_RECORD_THROTTLE = 5,
  PERF_RECORD_UNTHROTTLE = 6,
  PERF_RECORD_FORK = 7,
  PERF_RECORD_READ = 8,
  PERF_RECORD_SAMPLE = 9,
  PERF_RECORD_MMAP2 = 10,
  PERF_RECORD_AUX = 11,
  PERF_RECORD_ITRACE_START = 12,
  PERF_RECORD_LOST_SAMPLES = 13,
  PERF_RECORD_SWITCH = 14,
  PERF_RECORD_SWITCH_CPU_WIDE = 15,
  PERF_RECORD_NAMESPACES = 16,
  PERF_RECORD_KSYMBOL = 17,
  PERF_RECORD_BPF_EVENT = 18,
  PERF_RECORD_CGROUP = 19,
  PERF_RECORD_TEXT_POKE = 20,
  PERF_RECORD_AUX_OUTPUT_HW_ID = 21,

  PERF_RECORD_USER_TYPE_START = 64,
  PERF_RECORD_AUXTRACE_INFO = 70,
  PERF_RECORD_AUXTRACE = 71,
  PERF_RECORD_TIME_CONV = 79,
  PERF_RECORD_MAX, /* non-ABI */
};

enum perf_event_sample_format {
  PERF_SAMPLE_IP = 1U << 0,
  PERF_SAMPLE_TID = 1U << 1,
  PERF_SAMPLE_TIME = 1U << 2,
  PERF_SAMPLE_ADDR = 1U << 3,
  PERF_SAMPLE_READ = 1U << 4,
  PERF_SAMPLE_CALLCHAIN = 1U << 5,
  PERF_SAMPLE_ID = 1U << 6,
  PERF_SAMPLE_CPU = 1U << 7,
  PERF_SAMPLE_PERIOD = 1U << 8,
  PERF_SAMPLE_STREAM_ID = 1U << 9,
  PERF_SAMPLE_RAW = 1U << 10,
  PERF_SAMPLE_BRANCH_STACK = 1U << 11,
  PERF_SAMPLE_REGS_USER = 1U << 12,
  PERF_SAMPLE_STACK_USER = 1U << 13,
  PERF_SAMPLE_WEIGHT = 1U << 14,
  PERF_SAMPLE_DATA_SRC = 1U << 15,
  PERF_SAMPLE_IDENTIFIER = 1U << 16,
  PERF_SAMPLE_TRANSACTION = 1U << 17,
  PERF_SAMPLE_REGS_INTR = 1U << 18,
  PERF_SAMPLE_PHYS_ADDR = 1U << 19,
  PERF_SAMPLE_AUX = 1U << 20,
  PERF_SAMPLE_CGROUP = 1U << 21,
  PERF_SAMPLE_DATA_PAGE_SIZE = 1U << 22,
  PERF_SAMPLE_CODE_PAGE_SIZE = 1U << 23,
  PERF_SAMPLE_WEIGHT_STRUCT = 1U << 24,

  PERF_SAMPLE_MAX = 1U << 25, /* non-ABI */
};

constexpr auto kPerfRecordMiscCpumodeMask = 0x7;

enum perf_record_misc {
  PERF_RECORD_MISC_CPUMODE_UNKNOWN = 0,
  PERF_RECORD_MISC_KERNEL = 1,
  PERF_RECORD_MISC_USER = 2,
  PERF_RECORD_MISC_HYPERVISOR = 3,
  PERF_RECORD_MISC_GUEST_KERNEL = 4,
  PERF_RECORD_MISC_GUEST_USER = 5,

  PERF_RECORD_MISC_MMAP_BUILD_ID = 1U << 14,
  PERF_RECORD_MISC_EXT_RESERVED = 1U << 15,
};

enum perf_event_read_format {
  PERF_FORMAT_TOTAL_TIME_ENABLED = 1U << 0,
  PERF_FORMAT_TOTAL_TIME_RUNNING = 1U << 1,
  PERF_FORMAT_ID = 1U << 2,
  PERF_FORMAT_GROUP = 1U << 3,
  PERF_FORMAT_LOST = 1U << 4,

  PERF_FORMAT_MAX = 1U << 5, /* non-ABI */
};

enum perf_callchain_context : uint64_t {
  PERF_CONTEXT_HV = static_cast<uint64_t>(-32),
  PERF_CONTEXT_KERNEL = static_cast<uint64_t>(-128),
  PERF_CONTEXT_USER = static_cast<uint64_t>(-512),

  PERF_CONTEXT_GUEST = static_cast<uint64_t>(-2048),
  PERF_CONTEXT_GUEST_KERNEL = static_cast<uint64_t>(-2176),
  PERF_CONTEXT_GUEST_USER = static_cast<uint64_t>(-2560),

  PERF_CONTEXT_MAX = static_cast<uint64_t>(-4095),
};

enum auxtrace_type {
  PERF_AUXTRACE_UNKNOWN,
  PERF_AUXTRACE_INTEL_PT,
  PERF_AUXTRACE_INTEL_BTS,
  PERF_AUXTRACE_CS_ETM,
  PERF_AUXTRACE_ARM_SPE,
  PERF_AUXTRACE_S390_CPUMSF,
  PERF_AUXTRACE_HISI_PTT,
};

enum perf_aux_flag {
  PERF_AUX_FLAG_TRUNCATED = 1U << 0,
  PERF_AUX_FLAG_OVERWRITE = 1U << 1,
  PERF_AUX_FLAG_PARTIAL = 1U << 2,
  PERF_AUX_FLAG_COLLISION = 1U << 3,
  PERF_AUX_FLAG_CORESIGHT_FORMAT_RAW = 1U << 8,
};

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_EVENT_H_
