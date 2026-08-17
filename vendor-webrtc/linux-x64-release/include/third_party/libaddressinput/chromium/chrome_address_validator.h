// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_ADDRESS_VALIDATOR_H_
#define THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_ADDRESS_VALIDATOR_H_

#include <stddef.h>

#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "third_party/libaddressinput/src/cpp/include/libaddressinput/address_field.h"
#include "third_party/libaddressinput/src/cpp/include/libaddressinput/address_validator.h"
#include "third_party/libaddressinput/src/cpp/include/libaddressinput/callback.h"
#include "third_party/libaddressinput/src/cpp/include/libaddressinput/preload_supplier.h"

namespace i18n {
namespace addressinput {
class AddressNormalizer;
class Source;
class Storage;
struct AddressData;
}
}

namespace autofill {

class InputSuggester;

// The object to be notified when loading of address validation rules is
// finished.
class LoadRulesListener {
 public:
  virtual ~LoadRulesListener() {}

  // Called when the validation rules for the |region_code| have been loaded.
  // The validation rules include the generic rules for the |region_code| and
  // specific rules for the country's administrative areas, localities, and
  // dependent localities. If a country has language-specific validation rules,
  // then these are also loaded.
  //
  // The |success| parameter is true when the rules were loaded successfully.
  virtual void OnAddressValidationRulesLoaded(const std::string& region_code,
                                              bool success) = 0;
};

// Interface to the libaddressinput AddressValidator for Chromium Autofill. The
// class is named AddressValidator to simplify switching between libaddressinput
// and this version.
//
// It's not possible to name this file address_validator.h because some
// compilers do not handle multiple files with the same name (although in
// different directories) gracefully. This class is a shim between upstream
// libaddressinput API and the API that Chrome expects, hence the file name
// chrome_address_validator.h.
class AddressValidator {
 public:
  // The status of address validation.
  enum Status {
    // Address validation completed successfully. Check |problems| to see if any
    // problems were found.
    SUCCESS,

    // The validation rules are not available, because LoadRules() was not
    // called or failed. Reload the rules.
    RULES_UNAVAILABLE,

    // The validation rules are being loaded. Try again later.
    RULES_NOT_READY
  };

  // Takes ownership of |source| and |storage|.
  AddressValidator(std::unique_ptr<::i18n::addressinput::Source> source,
                   std::unique_ptr<::i18n::addressinput::Storage> storage,
                   LoadRulesListener* load_rules_listener);

  AddressValidator(const AddressValidator&) = delete;
  AddressValidator& operator=(const AddressValidator&) = delete;

  virtual ~AddressValidator();

  // Loads the generic validation rules for |region_code| and specific rules
  // for the region's administrative areas, localities, and dependent
  // localities. A typical data size is 10KB. The largest is 250KB. If a region
  // has language-specific validation rules, then these are also loaded.
  //
  // Example rule:
  // https://i18napis.appspot.com/ssl-aggregate-address/data/US
  //
  // If the rules are already in progress of being loaded, it does nothing.
  // Invokes |load_rules_listener| when the loading has finished.
  virtual void LoadRules(const std::string& region_code);

  // Returns the list of sub-regions (recorded as sub-keys) of the region
  // (recorded as rule) indicated by |region_code|, while the device language
  // is set to |language|. So, if the |region_code| is
  // a country code, sub-region means the country's admin area.
  // This function should be called when the rules are loaded.
  virtual std::vector<std::pair<std::string, std::string>> GetRegionSubKeys(
      const std::string& region_code,
      const std::string& language);

  // Validates the |address| and populates |problems| with the validation
  // problems, filtered according to the |filter| parameter.
  //
  // If the |filter| is empty, then all discovered validation problems are
  // returned. If the |filter| contains problem elements, then only the problems
  // in the |filter| may be returned.
  virtual Status ValidateAddress(
      const ::i18n::addressinput::AddressData& address,
      const ::i18n::addressinput::FieldProblemMap* filter,
      ::i18n::addressinput::FieldProblemMap* problems) const;

  // Normalizes the |address_data|. For example, "texas" changes to "TX".
  // Returns true on success, otherwise leaves |address_data| alone and returns
  // false.
  virtual bool NormalizeAddress(
      ::i18n::addressinput::AddressData* address) const;

  // Returns whether the rules associated with the |region_code| are loaded.
  virtual bool AreRulesLoadedForRegion(const std::string& region_code);

 protected:
  // Constructor used only for MockAddressValidator.
  AddressValidator();

  // Returns the period of time to wait between the first attempt's failure and
  // the second attempt's initiation to load rules. Exposed for testing.
  virtual base::TimeDelta GetBaseRetryPeriod() const;

 private:
  // Verifies that |validator_| succeeded. Invoked by |validated_| callback.
  void Validated(bool success,
                 const ::i18n::addressinput::AddressData&,
                 const ::i18n::addressinput::FieldProblemMap&);

  // Invokes the |load_rules_listener_|, if it's not NULL. Called by
  // |rules_loaded_| callback.
  void RulesLoaded(bool success, const std::string& region_code, int);

  // Retries loading rules without resetting the retry counter.
  void RetryLoadRules(const std::string& region_code);

  // Loads and stores aggregate rules at COUNTRY level.
  const std::unique_ptr<::i18n::addressinput::PreloadSupplier> supplier_;

  // Normalizes addresses into a canonical form.
  const std::unique_ptr<::i18n::addressinput::AddressNormalizer> normalizer_;

  // Validates addresses.
  const std::unique_ptr<const ::i18n::addressinput::AddressValidator>
      validator_;

  // The callback that |validator_| invokes when it finished validating an
  // address.
  const std::unique_ptr<const ::i18n::addressinput::AddressValidator::Callback>
      validated_;

  // The callback that |supplier_| invokes when it finished loading rules.
  const std::unique_ptr<const ::i18n::addressinput::PreloadSupplier::Callback>
      rules_loaded_;

  // Not owned delegate to invoke when |suppler_| finished loading rules. Can be
  // NULL.
  LoadRulesListener* const load_rules_listener_;

  // A mapping of region codes to the number of attempts to retry loading rules.
  std::map<std::string, int> attempts_number_;

  // Member variables should appear before the WeakPtrFactory, to ensure that
  // any WeakPtrs to AddressValidator are invalidated before its members
  // variable's destructors are executed, rendering them invalid.
  base::WeakPtrFactory<AddressValidator> weak_factory_{this};
};

}  // namespace autofill

#endif  // THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_ADDRESS_VALIDATOR_H_
