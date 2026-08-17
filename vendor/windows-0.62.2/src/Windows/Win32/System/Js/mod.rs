#[inline]
pub unsafe fn JsAddRef(r#ref: *const core::ffi::c_void, count: Option<*mut u32>) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsAddRef(r#ref : *const core::ffi::c_void, count : *mut u32) -> JsErrorCode);
    unsafe { JsAddRef(r#ref, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsBoolToBoolean(value : u8, booleanvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsBoolToBoolean(value, booleanvalue as _) }
}
#[inline]
pub unsafe fn JsBooleanToBool(value: *const core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsBooleanToBool(value : *const core::ffi::c_void, boolvalue : *mut bool) -> JsErrorCode);
    unsafe { JsBooleanToBool(value, boolvalue as _) }
}
#[inline]
pub unsafe fn JsCallFunction(function: *const core::ffi::c_void, arguments: &[*const core::ffi::c_void], result: Option<*mut *mut core::ffi::c_void>) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCallFunction(function : *const core::ffi::c_void, arguments : *const *const core::ffi::c_void, argumentcount : u16, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCallFunction(function, core::mem::transmute(arguments.as_ptr()), arguments.len().try_into().unwrap(), result.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JsCollectGarbage(runtime: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCollectGarbage(runtime : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCollectGarbage(runtime) }
}
#[inline]
pub unsafe fn JsConstructObject(function: *const core::ffi::c_void, arguments: &[*const core::ffi::c_void], result: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsConstructObject(function : *const core::ffi::c_void, arguments : *const *const core::ffi::c_void, argumentcount : u16, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsConstructObject(function, core::mem::transmute(arguments.as_ptr()), arguments.len().try_into().unwrap(), result as _) }
}
#[inline]
pub unsafe fn JsConvertValueToBoolean(value: *const core::ffi::c_void, booleanvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsConvertValueToBoolean(value : *const core::ffi::c_void, booleanvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsConvertValueToBoolean(value, booleanvalue as _) }
}
#[inline]
pub unsafe fn JsConvertValueToNumber(value: *const core::ffi::c_void, numbervalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsConvertValueToNumber(value : *const core::ffi::c_void, numbervalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsConvertValueToNumber(value, numbervalue as _) }
}
#[inline]
pub unsafe fn JsConvertValueToObject(value: *const core::ffi::c_void, object: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsConvertValueToObject(value : *const core::ffi::c_void, object : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsConvertValueToObject(value, object as _) }
}
#[inline]
pub unsafe fn JsConvertValueToString(value: *const core::ffi::c_void, stringvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsConvertValueToString(value : *const core::ffi::c_void, stringvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsConvertValueToString(value, stringvalue as _) }
}
#[inline]
pub unsafe fn JsCreateArray(length: u32, result: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateArray(length : u32, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateArray(length, result as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsCreateContext<P1>(runtime: *const core::ffi::c_void, debugapplication: P1, newcontext: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P1: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IDebugApplication32>,
{
    windows_core::link!("chakra.dll" "system" fn JsCreateContext(runtime : *const core::ffi::c_void, debugapplication : * mut core::ffi::c_void, newcontext : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateContext(runtime, debugapplication.param().abi(), newcontext as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsCreateContext<P1>(runtime: *const core::ffi::c_void, debugapplication: P1, newcontext: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P1: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IDebugApplication64>,
{
    windows_core::link!("chakra.dll" "system" fn JsCreateContext(runtime : *const core::ffi::c_void, debugapplication : * mut core::ffi::c_void, newcontext : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateContext(runtime, debugapplication.param().abi(), newcontext as _) }
}
#[inline]
pub unsafe fn JsCreateError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateError(message, error as _) }
}
#[inline]
pub unsafe fn JsCreateExternalObject(data: Option<*const core::ffi::c_void>, finalizecallback: JsFinalizeCallback, object: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateExternalObject(data : *const core::ffi::c_void, finalizecallback : JsFinalizeCallback, object : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateExternalObject(data.unwrap_or(core::mem::zeroed()) as _, finalizecallback, object as _) }
}
#[inline]
pub unsafe fn JsCreateFunction(nativefunction: JsNativeFunction, callbackstate: Option<*const core::ffi::c_void>, function: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateFunction(nativefunction : JsNativeFunction, callbackstate : *const core::ffi::c_void, function : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateFunction(nativefunction, callbackstate.unwrap_or(core::mem::zeroed()) as _, function as _) }
}
#[inline]
pub unsafe fn JsCreateObject(object: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateObject(object : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateObject(object as _) }
}
#[inline]
pub unsafe fn JsCreateRangeError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateRangeError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateRangeError(message, error as _) }
}
#[inline]
pub unsafe fn JsCreateReferenceError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateReferenceError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateReferenceError(message, error as _) }
}
#[inline]
pub unsafe fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: JsThreadServiceCallback, runtime: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateRuntime(attributes : JsRuntimeAttributes, runtimeversion : JsRuntimeVersion, threadservice : JsThreadServiceCallback, runtime : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateRuntime(attributes, runtimeversion, threadservice, runtime as _) }
}
#[inline]
pub unsafe fn JsCreateSyntaxError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateSyntaxError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateSyntaxError(message, error as _) }
}
#[inline]
pub unsafe fn JsCreateTypeError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateTypeError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateTypeError(message, error as _) }
}
#[inline]
pub unsafe fn JsCreateURIError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsCreateURIError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsCreateURIError(message, error as _) }
}
#[inline]
pub unsafe fn JsDefineProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, propertydescriptor: *const core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsDefineProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, propertydescriptor : *const core::ffi::c_void, result : *mut bool) -> JsErrorCode);
    unsafe { JsDefineProperty(object, propertyid, propertydescriptor, result as _) }
}
#[inline]
pub unsafe fn JsDeleteIndexedProperty(object: *const core::ffi::c_void, index: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsDeleteIndexedProperty(object : *const core::ffi::c_void, index : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsDeleteIndexedProperty(object, index) }
}
#[inline]
pub unsafe fn JsDeleteProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, usestrictrules: u8, result: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsDeleteProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, usestrictrules : u8, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsDeleteProperty(object, propertyid, usestrictrules, result as _) }
}
#[inline]
pub unsafe fn JsDisableRuntimeExecution(runtime: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsDisableRuntimeExecution(runtime : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsDisableRuntimeExecution(runtime) }
}
#[inline]
pub unsafe fn JsDisposeRuntime(runtime: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsDisposeRuntime(runtime : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsDisposeRuntime(runtime) }
}
#[inline]
pub unsafe fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsDoubleToNumber(doublevalue : f64, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsDoubleToNumber(doublevalue, value as _) }
}
#[inline]
pub unsafe fn JsEnableRuntimeExecution(runtime: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsEnableRuntimeExecution(runtime : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsEnableRuntimeExecution(runtime) }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsEnumerateHeap(enumerator: *mut Option<super::Diagnostics::Debug::ActiveScript::IActiveScriptProfilerHeapEnum>) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsEnumerateHeap(enumerator : *mut * mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsEnumerateHeap(core::mem::transmute(enumerator)) }
}
#[inline]
pub unsafe fn JsEquals(object1: *const core::ffi::c_void, object2: *const core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsEquals(object1 : *const core::ffi::c_void, object2 : *const core::ffi::c_void, result : *mut bool) -> JsErrorCode);
    unsafe { JsEquals(object1, object2, result as _) }
}
#[inline]
pub unsafe fn JsGetAndClearException(exception: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetAndClearException(exception : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetAndClearException(exception as _) }
}
#[inline]
pub unsafe fn JsGetCurrentContext(currentcontext: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetCurrentContext(currentcontext : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetCurrentContext(currentcontext as _) }
}
#[inline]
pub unsafe fn JsGetExtensionAllowed(object: *const core::ffi::c_void, value: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetExtensionAllowed(object : *const core::ffi::c_void, value : *mut bool) -> JsErrorCode);
    unsafe { JsGetExtensionAllowed(object, value as _) }
}
#[inline]
pub unsafe fn JsGetExternalData(object: *const core::ffi::c_void, externaldata: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetExternalData(object : *const core::ffi::c_void, externaldata : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetExternalData(object, externaldata as _) }
}
#[inline]
pub unsafe fn JsGetFalseValue(falsevalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetFalseValue(falsevalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetFalseValue(falsevalue as _) }
}
#[inline]
pub unsafe fn JsGetGlobalObject(globalobject: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetGlobalObject(globalobject : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetGlobalObject(globalobject as _) }
}
#[inline]
pub unsafe fn JsGetIndexedProperty(object: *const core::ffi::c_void, index: *const core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetIndexedProperty(object : *const core::ffi::c_void, index : *const core::ffi::c_void, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetIndexedProperty(object, index, result as _) }
}
#[inline]
pub unsafe fn JsGetNullValue(nullvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetNullValue(nullvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetNullValue(nullvalue as _) }
}
#[inline]
pub unsafe fn JsGetOwnPropertyDescriptor(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, propertydescriptor: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetOwnPropertyDescriptor(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, propertydescriptor : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetOwnPropertyDescriptor(object, propertyid, propertydescriptor as _) }
}
#[inline]
pub unsafe fn JsGetOwnPropertyNames(object: *const core::ffi::c_void, propertynames: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetOwnPropertyNames(object : *const core::ffi::c_void, propertynames : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetOwnPropertyNames(object, propertynames as _) }
}
#[inline]
pub unsafe fn JsGetProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetProperty(object, propertyid, value as _) }
}
#[inline]
pub unsafe fn JsGetPropertyIdFromName<P0>(name: P0, propertyid: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("chakra.dll" "system" fn JsGetPropertyIdFromName(name : windows_core::PCWSTR, propertyid : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetPropertyIdFromName(name.param().abi(), propertyid as _) }
}
#[inline]
pub unsafe fn JsGetPropertyNameFromId(propertyid: *const core::ffi::c_void, name: *mut *mut u16) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetPropertyNameFromId(propertyid : *const core::ffi::c_void, name : *mut *mut u16) -> JsErrorCode);
    unsafe { JsGetPropertyNameFromId(propertyid, name as _) }
}
#[inline]
pub unsafe fn JsGetPrototype(object: *const core::ffi::c_void, prototypeobject: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetPrototype(object : *const core::ffi::c_void, prototypeobject : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetPrototype(object, prototypeobject as _) }
}
#[inline]
pub unsafe fn JsGetRuntime(context: *const core::ffi::c_void, runtime: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetRuntime(context : *const core::ffi::c_void, runtime : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetRuntime(context, runtime as _) }
}
#[inline]
pub unsafe fn JsGetRuntimeMemoryLimit(runtime: *const core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetRuntimeMemoryLimit(runtime : *const core::ffi::c_void, memorylimit : *mut usize) -> JsErrorCode);
    unsafe { JsGetRuntimeMemoryLimit(runtime, memorylimit as _) }
}
#[inline]
pub unsafe fn JsGetRuntimeMemoryUsage(runtime: *const core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetRuntimeMemoryUsage(runtime : *const core::ffi::c_void, memoryusage : *mut usize) -> JsErrorCode);
    unsafe { JsGetRuntimeMemoryUsage(runtime, memoryusage as _) }
}
#[inline]
pub unsafe fn JsGetStringLength(stringvalue: *const core::ffi::c_void, length: *mut i32) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetStringLength(stringvalue : *const core::ffi::c_void, length : *mut i32) -> JsErrorCode);
    unsafe { JsGetStringLength(stringvalue, length as _) }
}
#[inline]
pub unsafe fn JsGetTrueValue(truevalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetTrueValue(truevalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetTrueValue(truevalue as _) }
}
#[inline]
pub unsafe fn JsGetUndefinedValue(undefinedvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetUndefinedValue(undefinedvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsGetUndefinedValue(undefinedvalue as _) }
}
#[inline]
pub unsafe fn JsGetValueType(value: *const core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsGetValueType(value : *const core::ffi::c_void, r#type : *mut JsValueType) -> JsErrorCode);
    unsafe { JsGetValueType(value, r#type as _) }
}
#[inline]
pub unsafe fn JsHasException(hasexception: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsHasException(hasexception : *mut bool) -> JsErrorCode);
    unsafe { JsHasException(hasexception as _) }
}
#[inline]
pub unsafe fn JsHasExternalData(object: *const core::ffi::c_void, value: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsHasExternalData(object : *const core::ffi::c_void, value : *mut bool) -> JsErrorCode);
    unsafe { JsHasExternalData(object, value as _) }
}
#[inline]
pub unsafe fn JsHasIndexedProperty(object: *const core::ffi::c_void, index: *const core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsHasIndexedProperty(object : *const core::ffi::c_void, index : *const core::ffi::c_void, result : *mut bool) -> JsErrorCode);
    unsafe { JsHasIndexedProperty(object, index, result as _) }
}
#[inline]
pub unsafe fn JsHasProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsHasProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, hasproperty : *mut bool) -> JsErrorCode);
    unsafe { JsHasProperty(object, propertyid, hasproperty as _) }
}
#[inline]
pub unsafe fn JsIdle(nextidletick: Option<*mut u32>) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsIdle(nextidletick : *mut u32) -> JsErrorCode);
    unsafe { JsIdle(nextidletick.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JsIntToNumber(intvalue: i32, value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsIntToNumber(intvalue : i32, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsIntToNumber(intvalue, value as _) }
}
#[inline]
pub unsafe fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsIsEnumeratingHeap(isenumeratingheap : *mut bool) -> JsErrorCode);
    unsafe { JsIsEnumeratingHeap(isenumeratingheap as _) }
}
#[inline]
pub unsafe fn JsIsRuntimeExecutionDisabled(runtime: *const core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsIsRuntimeExecutionDisabled(runtime : *const core::ffi::c_void, isdisabled : *mut bool) -> JsErrorCode);
    unsafe { JsIsRuntimeExecutionDisabled(runtime, isdisabled as _) }
}
#[inline]
pub unsafe fn JsNumberToDouble(value: *const core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsNumberToDouble(value : *const core::ffi::c_void, doublevalue : *mut f64) -> JsErrorCode);
    unsafe { JsNumberToDouble(value, doublevalue as _) }
}
#[inline]
pub unsafe fn JsParseScript<P0, P2>(script: P0, sourcecontext: usize, sourceurl: P2, result: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("chakra.dll" "system" fn JsParseScript(script : windows_core::PCWSTR, sourcecontext : usize, sourceurl : windows_core::PCWSTR, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsParseScript(script.param().abi(), sourcecontext, sourceurl.param().abi(), result as _) }
}
#[inline]
pub unsafe fn JsParseSerializedScript<P0, P3>(script: P0, buffer: *const u8, sourcecontext: usize, sourceurl: P3, result: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("chakra.dll" "system" fn JsParseSerializedScript(script : windows_core::PCWSTR, buffer : *const u8, sourcecontext : usize, sourceurl : windows_core::PCWSTR, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsParseSerializedScript(script.param().abi(), buffer, sourcecontext, sourceurl.param().abi(), result as _) }
}
#[inline]
pub unsafe fn JsPointerToString(stringvalue: &[u16], value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsPointerToString(stringvalue : windows_core::PCWSTR, stringlength : usize, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsPointerToString(core::mem::transmute(stringvalue.as_ptr()), stringvalue.len().try_into().unwrap(), value as _) }
}
#[inline]
pub unsafe fn JsPreventExtension(object: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsPreventExtension(object : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsPreventExtension(object) }
}
#[inline]
pub unsafe fn JsRelease(r#ref: *const core::ffi::c_void, count: Option<*mut u32>) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsRelease(r#ref : *const core::ffi::c_void, count : *mut u32) -> JsErrorCode);
    unsafe { JsRelease(r#ref, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JsRunScript<P0, P2>(script: P0, sourcecontext: usize, sourceurl: P2, result: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("chakra.dll" "system" fn JsRunScript(script : windows_core::PCWSTR, sourcecontext : usize, sourceurl : windows_core::PCWSTR, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsRunScript(script.param().abi(), sourcecontext, sourceurl.param().abi(), result as _) }
}
#[inline]
pub unsafe fn JsRunSerializedScript<P0, P3>(script: P0, buffer: *const u8, sourcecontext: usize, sourceurl: P3, result: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("chakra.dll" "system" fn JsRunSerializedScript(script : windows_core::PCWSTR, buffer : *const u8, sourcecontext : usize, sourceurl : windows_core::PCWSTR, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsRunSerializedScript(script.param().abi(), buffer, sourcecontext, sourceurl.param().abi(), result as _) }
}
#[inline]
pub unsafe fn JsSerializeScript<P0>(script: P0, buffer: Option<*mut u8>, buffersize: *mut u32) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("chakra.dll" "system" fn JsSerializeScript(script : windows_core::PCWSTR, buffer : *mut u8, buffersize : *mut u32) -> JsErrorCode);
    unsafe { JsSerializeScript(script.param().abi(), buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn JsSetCurrentContext(context: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetCurrentContext(context : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsSetCurrentContext(context) }
}
#[inline]
pub unsafe fn JsSetException(exception: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetException(exception : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsSetException(exception) }
}
#[inline]
pub unsafe fn JsSetExternalData(object: *const core::ffi::c_void, externaldata: Option<*const core::ffi::c_void>) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetExternalData(object : *const core::ffi::c_void, externaldata : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsSetExternalData(object, externaldata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JsSetIndexedProperty(object: *const core::ffi::c_void, index: *const core::ffi::c_void, value: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetIndexedProperty(object : *const core::ffi::c_void, index : *const core::ffi::c_void, value : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsSetIndexedProperty(object, index, value) }
}
#[inline]
pub unsafe fn JsSetProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, value: *const core::ffi::c_void, usestrictrules: u8) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, value : *const core::ffi::c_void, usestrictrules : u8) -> JsErrorCode);
    unsafe { JsSetProperty(object, propertyid, value, usestrictrules) }
}
#[inline]
pub unsafe fn JsSetPrototype(object: *const core::ffi::c_void, prototypeobject: *const core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetPrototype(object : *const core::ffi::c_void, prototypeobject : *const core::ffi::c_void) -> JsErrorCode);
    unsafe { JsSetPrototype(object, prototypeobject) }
}
#[inline]
pub unsafe fn JsSetRuntimeBeforeCollectCallback(runtime: *const core::ffi::c_void, callbackstate: Option<*const core::ffi::c_void>, beforecollectcallback: JsBeforeCollectCallback) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetRuntimeBeforeCollectCallback(runtime : *const core::ffi::c_void, callbackstate : *const core::ffi::c_void, beforecollectcallback : JsBeforeCollectCallback) -> JsErrorCode);
    unsafe { JsSetRuntimeBeforeCollectCallback(runtime, callbackstate.unwrap_or(core::mem::zeroed()) as _, beforecollectcallback) }
}
#[inline]
pub unsafe fn JsSetRuntimeMemoryAllocationCallback(runtime: *const core::ffi::c_void, callbackstate: Option<*const core::ffi::c_void>, allocationcallback: JsMemoryAllocationCallback) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetRuntimeMemoryAllocationCallback(runtime : *const core::ffi::c_void, callbackstate : *const core::ffi::c_void, allocationcallback : JsMemoryAllocationCallback) -> JsErrorCode);
    unsafe { JsSetRuntimeMemoryAllocationCallback(runtime, callbackstate.unwrap_or(core::mem::zeroed()) as _, allocationcallback) }
}
#[inline]
pub unsafe fn JsSetRuntimeMemoryLimit(runtime: *const core::ffi::c_void, memorylimit: usize) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsSetRuntimeMemoryLimit(runtime : *const core::ffi::c_void, memorylimit : usize) -> JsErrorCode);
    unsafe { JsSetRuntimeMemoryLimit(runtime, memorylimit) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsStartDebugging<P0>(debugapplication: P0) -> JsErrorCode
where
    P0: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IDebugApplication32>,
{
    windows_core::link!("chakra.dll" "system" fn JsStartDebugging(debugapplication : * mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsStartDebugging(debugapplication.param().abi()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsStartDebugging<P0>(debugapplication: P0) -> JsErrorCode
where
    P0: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IDebugApplication64>,
{
    windows_core::link!("chakra.dll" "system" fn JsStartDebugging(debugapplication : * mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsStartDebugging(debugapplication.param().abi()) }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsStartProfiling<P0>(callback: P0, eventmask: super::Diagnostics::Debug::ActiveScript::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode
where
    P0: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IActiveScriptProfilerCallback>,
{
    windows_core::link!("chakra.dll" "system" fn JsStartProfiling(callback : * mut core::ffi::c_void, eventmask : super::Diagnostics::Debug::ActiveScript:: PROFILER_EVENT_MASK, context : u32) -> JsErrorCode);
    unsafe { JsStartProfiling(callback.param().abi(), eventmask, context) }
}
#[inline]
pub unsafe fn JsStopProfiling(reason: windows_core::HRESULT) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsStopProfiling(reason : windows_core::HRESULT) -> JsErrorCode);
    unsafe { JsStopProfiling(reason) }
}
#[inline]
pub unsafe fn JsStrictEquals(object1: *const core::ffi::c_void, object2: *const core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsStrictEquals(object1 : *const core::ffi::c_void, object2 : *const core::ffi::c_void, result : *mut bool) -> JsErrorCode);
    unsafe { JsStrictEquals(object1, object2, result as _) }
}
#[inline]
pub unsafe fn JsStringToPointer(value: *const core::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsStringToPointer(value : *const core::ffi::c_void, stringvalue : *mut *mut u16, stringlength : *mut usize) -> JsErrorCode);
    unsafe { JsStringToPointer(value, stringvalue as _, stringlength as _) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn JsValueToVariant(object: *const core::ffi::c_void, variant: *mut super::Variant::VARIANT) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsValueToVariant(object : *const core::ffi::c_void, variant : *mut super::Variant:: VARIANT) -> JsErrorCode);
    unsafe { JsValueToVariant(object, core::mem::transmute(variant)) }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn JsVariantToValue(variant: *const super::Variant::VARIANT, value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_core::link!("chakra.dll" "system" fn JsVariantToValue(variant : *const super::Variant:: VARIANT, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    unsafe { JsVariantToValue(core::mem::transmute(variant), value as _) }
}
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
pub const JsArray: JsValueType = JsValueType(8i32);
pub type JsBackgroundWorkItemCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void)>;
pub type JsBeforeCollectCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void)>;
pub const JsBoolean: JsValueType = JsValueType(4i32);
pub const JsError: JsValueType = JsValueType(7i32);
pub const JsErrorAlreadyDebuggingContext: JsErrorCode = JsErrorCode(65552u32);
pub const JsErrorAlreadyProfilingContext: JsErrorCode = JsErrorCode(65553u32);
pub const JsErrorArgumentNotObject: JsErrorCode = JsErrorCode(65548u32);
pub const JsErrorBadSerializedScript: JsErrorCode = JsErrorCode(65544u32);
pub const JsErrorCannotDisableExecution: JsErrorCode = JsErrorCode(65546u32);
pub const JsErrorCannotSerializeDebugScript: JsErrorCode = JsErrorCode(65551u32);
pub const JsErrorCategoryEngine: JsErrorCode = JsErrorCode(131072u32);
pub const JsErrorCategoryFatal: JsErrorCode = JsErrorCode(262144u32);
pub const JsErrorCategoryScript: JsErrorCode = JsErrorCode(196608u32);
pub const JsErrorCategoryUsage: JsErrorCode = JsErrorCode(65536u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JsErrorCode(pub u32);
pub const JsErrorFatal: JsErrorCode = JsErrorCode(262145u32);
pub const JsErrorHeapEnumInProgress: JsErrorCode = JsErrorCode(65547u32);
pub const JsErrorIdleNotEnabled: JsErrorCode = JsErrorCode(65554u32);
pub const JsErrorInDisabledState: JsErrorCode = JsErrorCode(65545u32);
pub const JsErrorInExceptionState: JsErrorCode = JsErrorCode(65540u32);
pub const JsErrorInProfileCallback: JsErrorCode = JsErrorCode(65549u32);
pub const JsErrorInThreadServiceCallback: JsErrorCode = JsErrorCode(65550u32);
pub const JsErrorInvalidArgument: JsErrorCode = JsErrorCode(65537u32);
pub const JsErrorNoCurrentContext: JsErrorCode = JsErrorCode(65539u32);
pub const JsErrorNotImplemented: JsErrorCode = JsErrorCode(65541u32);
pub const JsErrorNullArgument: JsErrorCode = JsErrorCode(65538u32);
pub const JsErrorOutOfMemory: JsErrorCode = JsErrorCode(131073u32);
pub const JsErrorRuntimeInUse: JsErrorCode = JsErrorCode(65543u32);
pub const JsErrorScriptCompile: JsErrorCode = JsErrorCode(196610u32);
pub const JsErrorScriptEvalDisabled: JsErrorCode = JsErrorCode(196612u32);
pub const JsErrorScriptException: JsErrorCode = JsErrorCode(196609u32);
pub const JsErrorScriptTerminated: JsErrorCode = JsErrorCode(196611u32);
pub const JsErrorWrongThread: JsErrorCode = JsErrorCode(65542u32);
pub type JsFinalizeCallback = Option<unsafe extern "system" fn(data: *const core::ffi::c_void)>;
pub const JsFunction: JsValueType = JsValueType(6i32);
pub const JsMemoryAllocate: JsMemoryEventType = JsMemoryEventType(0i32);
pub type JsMemoryAllocationCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void, allocationevent: JsMemoryEventType, allocationsize: usize) -> bool>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JsMemoryEventType(pub i32);
pub const JsMemoryFailure: JsMemoryEventType = JsMemoryEventType(2i32);
pub const JsMemoryFree: JsMemoryEventType = JsMemoryEventType(1i32);
pub type JsNativeFunction = Option<unsafe extern "system" fn(callee: *const core::ffi::c_void, isconstructcall: bool, arguments: *const *const core::ffi::c_void, argumentcount: u16, callbackstate: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
pub const JsNoError: JsErrorCode = JsErrorCode(0u32);
pub const JsNull: JsValueType = JsValueType(1i32);
pub const JsNumber: JsValueType = JsValueType(2i32);
pub const JsObject: JsValueType = JsValueType(5i32);
pub const JsRuntimeAttributeAllowScriptInterrupt: JsRuntimeAttributes = JsRuntimeAttributes(2i32);
pub const JsRuntimeAttributeDisableBackgroundWork: JsRuntimeAttributes = JsRuntimeAttributes(1i32);
pub const JsRuntimeAttributeDisableEval: JsRuntimeAttributes = JsRuntimeAttributes(16i32);
pub const JsRuntimeAttributeDisableNativeCodeGeneration: JsRuntimeAttributes = JsRuntimeAttributes(8i32);
pub const JsRuntimeAttributeEnableIdleProcessing: JsRuntimeAttributes = JsRuntimeAttributes(4i32);
pub const JsRuntimeAttributeNone: JsRuntimeAttributes = JsRuntimeAttributes(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JsRuntimeAttributes(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JsRuntimeVersion(pub i32);
pub const JsRuntimeVersion10: JsRuntimeVersion = JsRuntimeVersion(0i32);
pub const JsRuntimeVersion11: JsRuntimeVersion = JsRuntimeVersion(1i32);
pub const JsRuntimeVersionEdge: JsRuntimeVersion = JsRuntimeVersion(-1i32);
pub const JsString: JsValueType = JsValueType(3i32);
pub type JsThreadServiceCallback = Option<unsafe extern "system" fn(callback: JsBackgroundWorkItemCallback, callbackstate: *const core::ffi::c_void) -> bool>;
pub const JsUndefined: JsValueType = JsValueType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JsValueType(pub i32);
