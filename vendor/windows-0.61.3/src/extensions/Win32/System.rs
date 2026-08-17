#[cfg(feature = "Win32_System_Rpc")]
mod Rpc;
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
mod StructuredStorage;
#[cfg(all(feature = "Win32_System_Variant", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
mod Variant;
