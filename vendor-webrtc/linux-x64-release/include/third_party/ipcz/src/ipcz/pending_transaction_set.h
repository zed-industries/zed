// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_PENDING_TRANSACTION_SET_
#define IPCZ_SRC_IPCZ_PENDING_TRANSACTION_SET_

#include <memory>
#include <set>

#include "ipcz/ipcz.h"
#include "ipcz/parcel.h"
#include "util/unique_ptr_comparator.h"

namespace ipcz {

// PendingTransactionSet wraps a set of pending Parcel objects with a special
// case for the common scenario where single elements are repeatedly inserted
// and then removed from the set, repeatedly.
//
// Care is taken to ensure that any Parcel owned by this set has a stable
// address throughout its lifetime, exposed as an opaque IpczTransaction value.
class PendingTransactionSet {
 public:
  PendingTransactionSet();
  PendingTransactionSet(const PendingTransactionSet&) = delete;
  PendingTransactionSet& operator=(const PendingTransactionSet&) = delete;
  ~PendingTransactionSet();

  bool empty() const { return transactions_.empty(); }

  // Adds `parcel` to this set, returning an opaque IpczTransaction value to
  // reference it.
  IpczTransaction Add(std::unique_ptr<Parcel> parcel);

  // Finalizes the transaction identified by `transaction`, returning its
  // underlying Parcel. Only succeeds if `transaction` is a valid transaction.
  std::unique_ptr<Parcel> FinalizeForGet(IpczTransaction transaction);

  // Finalizes the transaction identified by `transaction`, returning its
  // underlying Parcel so that data can be committed to it and it can be put
  // into a portal. Only succeeds if `transaction` is a valid transaction and
  // `num_data_bytes` does not exceed the total capacity of the underlying
  // Parcel. Note that this does not actually commit any data to the parcel.
  std::unique_ptr<Parcel> FinalizeForPut(IpczTransaction transaction,
                                         size_t num_data_bytes);

 private:
  using TransactionSet = std::set<std::unique_ptr<Parcel>, UniquePtrComparator>;

  static IpczTransaction AsIpczTransaction(Parcel& parcel) {
    return reinterpret_cast<IpczTransaction>(&parcel);
  }

  std::unique_ptr<Parcel> Extract(TransactionSet::iterator it);

  TransactionSet transactions_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_PENDING_TRANSACTION_SET_
