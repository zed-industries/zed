# r-efi - UEFI Reference Specification Protocol Constants and Definitions

## CHANGES WITH 5.3.0:

        * Remove the optional dependency on `compiler-builtins`, which was
          needed to build r-efi as part of rustc. This is no longer necessary.

        Contributions from: David Rheinsberg, Trevor Gross

        - Dußlingen, 2025-06-17

## CHANGES WITH 5.2.0:

        * Add the memory attribute protocol.

        Contributions from: David Rheinsberg, Oliver Smith-Denny

        - Dußlingen, 2024-12-22

## CHANGES WITH 5.1.0:

        * Mark `Guid::as_bytes` and `Guid::from_bytes` as `const fn`, aligning
          them with the other methods on `Guid`. This helps creating constant
          GUIDs via macros or other external helpers.

        Contributions from: Christopher Zurcher, David Rheinsberg

        - Dußlingen, 2024-09-01

## CHANGES WITH 5.0.0:

        * Change the type of the `unload` function-pointer of the Loaded Image
          Protocol to `Option<Unload>`, given that it can be `NULL` to indicate
          that the image cannot be unloaded.
          This is a major API break, but any users very likely need to adjust
          anyway to avoid NULL-derefs.

        Contributions from: David Rheinsberg, John Schock

        - Dußlingen, 2024-07-30

## CHANGES WITH 4.5.0:

        * Implement or derive more standard traits for UEFI base types. In
          particular, implement `[Partial]Eq`, `Hash`, `[Partial]Ord` for
          `Boolean`, `Status`, `Guid`, and network address types.

        * Fix the signature of `BootUninstallMultipleProtocolInterfaces` to
          match the UEFI specification. Note that it uses var-args and is thus
          not fully usable from stable Rust.

        Contributions from: Ayush Singh, David Rheinsberg, John Schock

        - Dußlingen, 2024-05-23

## CHANGES WITH 4.4.0:

        * Add definitions for `UNACCEPTED_MEMORY_TYPE`, media device subtypes
          for device paths, before-EBS and after-RTB event groups, missing
          memory attributes.

        * Add memory masks for common memory attribute classes. The symbol
          names are takend from EDK2, yet their purpose is defined in the
          specification.

        * New protocols: platform_driver_override, bus_specific_driver_override,
          driver_family_override, load_file, load_file2, pci-io

        Contributions from: David Rheinsberg, Dmitry Mostovenko, John Schock,
                            Michael Kubacki

        - Dußlingen, 2024-03-27

## CHANGES WITH 4.3.0:

        * Change alignment of `Guid` to 4 (was 8 before). This deviates from
          the specification, but aligns with EDK2. This should fix alignment
          mismatches when combining r-efi with EDK2, or other UEFI
          implementations.

        * `Guid` gained a new constructor `from_bytes()` to allow creating
          GUID-abstractions from foreign types based on the standardized
          memory representation.

        * Add all configuration-table GUIDs mentioned in the spec. These are
          often rooted in external specifications, but are strongly related
          to UEFI.

        * Add configuration-table definitions for RT_PROPERTIES and
          CONFORMANCE_PROFILES.

        * New protocols: hii_package_list, absolute_pointer

        Contributions from: David Rheinsberg, John Schock, Michael Kubacki,
                            Nicholas Bishop

        - Dußlingen, 2023-10-18

## CHANGES WITH 4.2.0:

        * Bump required compiler version to: rust-1.68

        * New Protocols: debugport, debug-support, driver-diagnostics2,
                         mp-services, shell, shell-dynamic-command,
                         shell-parameters, udp-4, udp-6

        * Use const-generics instead of ZSTs to represent dynamic trailing
          members in C structs.

        * The `examples` feature has been renamed to `native` (a backwards
          compatible feature is left in place).

        * Add support for riscv64.

        * Use the official rust `efiapi` calling convention. This was
          stabilized with rust-1.68.

        Contributions from: Ayush Singh, David Rheinsberg, Rob Bradford

        - Dußlingen, 2023-03-20

## CHANGES WITH 4.1.0:

        * New Protocols: device-path-{from,to}-text, ip4, ip6, managed-network,
                         rng, service-binding, tcp4, tcp6, timestamp

        * `ImageEntryPoint` is now correctly annotated as `eficall`.

        * `Time` now derives `Default`.

        * Fix nullable function pointers to use `Option<fn ...>`.

        * Function prototypes now have an explicit type definition and can be
          used independent of their protocol definition.

        * The new `rust-dep-of-std` feature option allows pulling in r-efi
          into the rust standard library. It prepares the crate workspace to
          be suitable for the standard library. It has no use outside of this.

        * Adopt the MIT license as 3rd licensing option to allow for
          integration into the rust compiler and ecosystem.

        Contributions from: Ayush Singh, David Rheinsberg, Joe Richey

        - Tübingen, 2022-08-23

## CHANGES WITH 4.0.0:

        * Convert all enums to constants with type-aliases. This is an API
          break, but it is needed for spec-compliance. With the old enums, one
          couldn't encode all the possible values defined by the spec.
          Especially, the vendor-reserved ranges were unable to be encoded in
          a safe manner. Also see commit 401a91901e860 for a detailed
          discussion.
          API users likely need to convert their CamelCase enum usage to the
          new UPPER_CASE constants.

        * Convert all incomplete types to empty arrays. This affects all
          structures that use trailing unbound arrays. These are actually ABI
          incompatible with UEFI, since rust represents raw-pointers to such
          types as fat-pointers. Such arrays have now been converted to empty
          arrays, which should still allow accessing the memory location and
          retaining structure properties, but avoids fat-pointers.
          This is an API break, so you might have to adjust your accessors of
          those trailing structure members.

        * Implement `Clone` and `Copy` for most basic structures. Since these
          are used as plain carriers, no higher clone/copy logic is needed. It
          should be clear from the project-description, that only basic UEFI
          compatibility is provided.

        * Add the console-control vendor protocol. This protocol allows
          controlling console properties. It is not part of the UEFI
          specification, but rather defined by the TianoCore project.

        * Add a new example showing how to use the GOP functions to query the
          active graphics device.

        Contributions from: David Rheinsberg, GGRei, Hiroki Tokunaga,
                            Richard Wiedenhöft

        - Tübingen, 2021-06-23

## CHANGES WITH 3.2.0:

        * Add new protocols: DiskIo, DiskIo2, BlockIo, DriverBinding

        * Extend the Device-Path payload structure and add the HardDriveMedia
          payload.

        * Add HII definitions: A new top-level module `hii` with all the basic
          HII constants, as well as a handful of HII protocols (hii_database,
          hii_font, hii_string)

        * Document new `-Zbuild-std` based cross-compilation, serving as
          official rust alternative to cargo-xbuild.

        Contributions from: Alex James, Bret Barkelew, David Rheinsberg,
                            Michael Kubacki

        - Tübingen, 2020-10-23

## CHANGES WITH 3.1.0:

        * Add the basic networking types to `r_efi::base`. This includes MAC
          and IP address types.

        * Add the EFI_SIMPLE_NETWORK_PROTOCOL definitions and all required
          constants to make basic networking available.

        * Add a new uefi-cross example, which is copied from upstream rustc
          sources, so we can test local modifications to it.

        Contributions from: Alex James, David Rheinsberg

        - Tübingen, 2020-09-10

## CHANGES WITH 3.0.0:

        * Fix a missing parameter in `BootServices::locate_device_path()`. The
          prototype incorrectly had 2 arguments, while the official version
          takes 3. The final `handle` argument was missing.
          This is an API break in `r-efi`. It should have a limited impact,
          since the function was mostly useless without a handle.
          Thanks to Michael Kubacki for catching this!

        * Adjust the `device_path` parameter in a bunch of `BootServices`
          calls. This used to take a `*mut c_void` parameter, since the device
          path protocol was not implemented.
          Since we have to bump the major version anyway, we use this to also
          fix these argument-types to the correct device-path protocol type,
          which has been implemented some time ago.

        Contributions from: David Rheinsberg, Michael Kubacki

        - Tübingen, 2020-04-24

## CHANGES WITH 2.2.0:

        * Provide `as_usize()` accessor for `efi::Status` types. This allows
          accessing the raw underlying value of a status object.

        * The project moved to its new home at: github.com/r-efi/r-efi

        Contributions from: David Rheinsberg, Joe Richey

        - Tübingen, 2020-04-16

## CHANGES WITH 2.1.0:

        * Add the graphics-output-protocol.

        * Expose reserved fields in open structures, otherwise they cannot be
          instantiated from outside the crate itself.

        Contributions from: David Herrmann, Richard Wiedenhöft, Rob Bradford

        - Tübingen, 2019-03-20

## CHANGES WITH 2.0.0:

        * Add a set of UEFI protocols, including simple-text-input,
          file-protocol, simple-file-system, device-path, and more.

        * Fix signature of `BootServices::allocate_pages`.

        Contributions from: David Rheinsberg, Richard Wiedenhöft, Tom Gundersen

        - Tübingen, 2019-03-01

## CHANGES WITH 1.0.0:

        * Enhance the basic UEFI type integration with the rust ecosystem. Add
          `Debug`, `Eq`, `Ord`, ... derivations, provide converters to/from the
          core library, and document the internal workings.

        * Fix `Boolean` to use `newtype(u8)` to make it ABI compatible to UEFI.
          This now accepts any byte value that UEFI accetps without any
          conversion required.

        Contributions from: Boris-Chengbiao Zhou, David Rheinsberg, Tom
                            Gundersen

        - Tübingen, 2019-02-14

## CHANGES WITH 0.1.1:

        * Feature gate examples to make `cargo test` work on non-UEFI systems
          like CI.

        Contributions from: David Herrmann

        - Tübingen, 2018-12-10

## CHANGES WITH 0.1.0:

        * Initial release of r-efi.

        Contributions from: David Herrmann

        - Tübingen, 2018-12-10
