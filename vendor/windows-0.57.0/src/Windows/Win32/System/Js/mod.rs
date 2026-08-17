#[inline]
pub unsafe fn JsAddRef(r#ref: *const core::ffi::c_void, count: Option<*mut u32>) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsAddRef(r#ref : *const core::ffi::c_void, count : *mut u32) -> JsErrorCode);
    JsAddRef(r#ref, core::mem::transmute(count.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JsBoolToBoolean(value: u8, booleanvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsBoolToBoolean(value : u8, booleanvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsBoolToBoolean(value, booleanvalue)
}
#[inline]
pub unsafe fn JsBooleanToBool(value: *const core::ffi::c_void, boolvalue: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsBooleanToBool(value : *const core::ffi::c_void, boolvalue : *mut bool) -> JsErrorCode);
    JsBooleanToBool(value, boolvalue)
}
#[inline]
pub unsafe fn JsCallFunction(function: *const core::ffi::c_void, arguments: &[*const core::ffi::c_void], result: Option<*mut *mut core::ffi::c_void>) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCallFunction(function : *const core::ffi::c_void, arguments : *const *const core::ffi::c_void, argumentcount : u16, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCallFunction(function, core::mem::transmute(arguments.as_ptr()), arguments.len().try_into().unwrap(), core::mem::transmute(result.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JsCollectGarbage(runtime: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCollectGarbage(runtime : *const core::ffi::c_void) -> JsErrorCode);
    JsCollectGarbage(runtime)
}
#[inline]
pub unsafe fn JsConstructObject(function: *const core::ffi::c_void, arguments: &[*const core::ffi::c_void], result: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsConstructObject(function : *const core::ffi::c_void, arguments : *const *const core::ffi::c_void, argumentcount : u16, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsConstructObject(function, core::mem::transmute(arguments.as_ptr()), arguments.len().try_into().unwrap(), result)
}
#[inline]
pub unsafe fn JsConvertValueToBoolean(value: *const core::ffi::c_void, booleanvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsConvertValueToBoolean(value : *const core::ffi::c_void, booleanvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsConvertValueToBoolean(value, booleanvalue)
}
#[inline]
pub unsafe fn JsConvertValueToNumber(value: *const core::ffi::c_void, numbervalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsConvertValueToNumber(value : *const core::ffi::c_void, numbervalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsConvertValueToNumber(value, numbervalue)
}
#[inline]
pub unsafe fn JsConvertValueToObject(value: *const core::ffi::c_void, object: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsConvertValueToObject(value : *const core::ffi::c_void, object : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsConvertValueToObject(value, object)
}
#[inline]
pub unsafe fn JsConvertValueToString(value: *const core::ffi::c_void, stringvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsConvertValueToString(value : *const core::ffi::c_void, stringvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsConvertValueToString(value, stringvalue)
}
#[inline]
pub unsafe fn JsCreateArray(length: u32, result: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateArray(length : u32, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateArray(length, result)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsCreateContext<P0>(runtime: *const core::ffi::c_void, debugapplication: P0, newcontext: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IDebugApplication64>,
{
    windows_targets::link!("chakra.dll" "system" fn JsCreateContext(runtime : *const core::ffi::c_void, debugapplication : * mut core::ffi::c_void, newcontext : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateContext(runtime, debugapplication.param().abi(), newcontext)
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsCreateContext<P0>(runtime: *const core::ffi::c_void, debugapplication: P0, newcontext: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IDebugApplication32>,
{
    windows_targets::link!("chakra.dll" "system" fn JsCreateContext(runtime : *const core::ffi::c_void, debugapplication : * mut core::ffi::c_void, newcontext : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateContext(runtime, debugapplication.param().abi(), newcontext)
}
#[inline]
pub unsafe fn JsCreateError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateError(message, error)
}
#[inline]
pub unsafe fn JsCreateExternalObject(data: Option<*const core::ffi::c_void>, finalizecallback: JsFinalizeCallback, object: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateExternalObject(data : *const core::ffi::c_void, finalizecallback : JsFinalizeCallback, object : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateExternalObject(core::mem::transmute(data.unwrap_or(std::ptr::null())), finalizecallback, object)
}
#[inline]
pub unsafe fn JsCreateFunction(nativefunction: JsNativeFunction, callbackstate: Option<*const core::ffi::c_void>, function: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateFunction(nativefunction : JsNativeFunction, callbackstate : *const core::ffi::c_void, function : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateFunction(nativefunction, core::mem::transmute(callbackstate.unwrap_or(std::ptr::null())), function)
}
#[inline]
pub unsafe fn JsCreateObject(object: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateObject(object : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateObject(object)
}
#[inline]
pub unsafe fn JsCreateRangeError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateRangeError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateRangeError(message, error)
}
#[inline]
pub unsafe fn JsCreateReferenceError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateReferenceError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateReferenceError(message, error)
}
#[inline]
pub unsafe fn JsCreateRuntime(attributes: JsRuntimeAttributes, runtimeversion: JsRuntimeVersion, threadservice: JsThreadServiceCallback, runtime: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateRuntime(attributes : JsRuntimeAttributes, runtimeversion : JsRuntimeVersion, threadservice : JsThreadServiceCallback, runtime : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateRuntime(attributes, runtimeversion, threadservice, runtime)
}
#[inline]
pub unsafe fn JsCreateSyntaxError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateSyntaxError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateSyntaxError(message, error)
}
#[inline]
pub unsafe fn JsCreateTypeError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateTypeError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateTypeError(message, error)
}
#[inline]
pub unsafe fn JsCreateURIError(message: *const core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsCreateURIError(message : *const core::ffi::c_void, error : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsCreateURIError(message, error)
}
#[inline]
pub unsafe fn JsDefineProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, propertydescriptor: *const core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsDefineProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, propertydescriptor : *const core::ffi::c_void, result : *mut bool) -> JsErrorCode);
    JsDefineProperty(object, propertyid, propertydescriptor, result)
}
#[inline]
pub unsafe fn JsDeleteIndexedProperty(object: *const core::ffi::c_void, index: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsDeleteIndexedProperty(object : *const core::ffi::c_void, index : *const core::ffi::c_void) -> JsErrorCode);
    JsDeleteIndexedProperty(object, index)
}
#[inline]
pub unsafe fn JsDeleteProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, usestrictrules: u8, result: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsDeleteProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, usestrictrules : u8, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsDeleteProperty(object, propertyid, usestrictrules, result)
}
#[inline]
pub unsafe fn JsDisableRuntimeExecution(runtime: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsDisableRuntimeExecution(runtime : *const core::ffi::c_void) -> JsErrorCode);
    JsDisableRuntimeExecution(runtime)
}
#[inline]
pub unsafe fn JsDisposeRuntime(runtime: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsDisposeRuntime(runtime : *const core::ffi::c_void) -> JsErrorCode);
    JsDisposeRuntime(runtime)
}
#[inline]
pub unsafe fn JsDoubleToNumber(doublevalue: f64, value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsDoubleToNumber(doublevalue : f64, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsDoubleToNumber(doublevalue, value)
}
#[inline]
pub unsafe fn JsEnableRuntimeExecution(runtime: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsEnableRuntimeExecution(runtime : *const core::ffi::c_void) -> JsErrorCode);
    JsEnableRuntimeExecution(runtime)
}
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsEnumerateHeap(enumerator: *mut Option<super::Diagnostics::Debug::ActiveScript::IActiveScriptProfilerHeapEnum>) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsEnumerateHeap(enumerator : *mut * mut core::ffi::c_void) -> JsErrorCode);
    JsEnumerateHeap(core::mem::transmute(enumerator))
}
#[inline]
pub unsafe fn JsEquals(object1: *const core::ffi::c_void, object2: *const core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsEquals(object1 : *const core::ffi::c_void, object2 : *const core::ffi::c_void, result : *mut bool) -> JsErrorCode);
    JsEquals(object1, object2, result)
}
#[inline]
pub unsafe fn JsGetAndClearException(exception: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetAndClearException(exception : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetAndClearException(exception)
}
#[inline]
pub unsafe fn JsGetCurrentContext(currentcontext: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetCurrentContext(currentcontext : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetCurrentContext(currentcontext)
}
#[inline]
pub unsafe fn JsGetExtensionAllowed(object: *const core::ffi::c_void, value: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetExtensionAllowed(object : *const core::ffi::c_void, value : *mut bool) -> JsErrorCode);
    JsGetExtensionAllowed(object, value)
}
#[inline]
pub unsafe fn JsGetExternalData(object: *const core::ffi::c_void, externaldata: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetExternalData(object : *const core::ffi::c_void, externaldata : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetExternalData(object, externaldata)
}
#[inline]
pub unsafe fn JsGetFalseValue(falsevalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetFalseValue(falsevalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetFalseValue(falsevalue)
}
#[inline]
pub unsafe fn JsGetGlobalObject(globalobject: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetGlobalObject(globalobject : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetGlobalObject(globalobject)
}
#[inline]
pub unsafe fn JsGetIndexedProperty(object: *const core::ffi::c_void, index: *const core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetIndexedProperty(object : *const core::ffi::c_void, index : *const core::ffi::c_void, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetIndexedProperty(object, index, result)
}
#[inline]
pub unsafe fn JsGetNullValue(nullvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetNullValue(nullvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetNullValue(nullvalue)
}
#[inline]
pub unsafe fn JsGetOwnPropertyDescriptor(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, propertydescriptor: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetOwnPropertyDescriptor(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, propertydescriptor : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetOwnPropertyDescriptor(object, propertyid, propertydescriptor)
}
#[inline]
pub unsafe fn JsGetOwnPropertyNames(object: *const core::ffi::c_void, propertynames: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetOwnPropertyNames(object : *const core::ffi::c_void, propertynames : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetOwnPropertyNames(object, propertynames)
}
#[inline]
pub unsafe fn JsGetProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetProperty(object, propertyid, value)
}
#[inline]
pub unsafe fn JsGetPropertyIdFromName<P0>(name: P0, propertyid: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("chakra.dll" "system" fn JsGetPropertyIdFromName(name : windows_core::PCWSTR, propertyid : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetPropertyIdFromName(name.param().abi(), propertyid)
}
#[inline]
pub unsafe fn JsGetPropertyNameFromId(propertyid: *const core::ffi::c_void, name: *mut *mut u16) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetPropertyNameFromId(propertyid : *const core::ffi::c_void, name : *mut *mut u16) -> JsErrorCode);
    JsGetPropertyNameFromId(propertyid, name)
}
#[inline]
pub unsafe fn JsGetPrototype(object: *const core::ffi::c_void, prototypeobject: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetPrototype(object : *const core::ffi::c_void, prototypeobject : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetPrototype(object, prototypeobject)
}
#[inline]
pub unsafe fn JsGetRuntime(context: *const core::ffi::c_void, runtime: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetRuntime(context : *const core::ffi::c_void, runtime : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetRuntime(context, runtime)
}
#[inline]
pub unsafe fn JsGetRuntimeMemoryLimit(runtime: *const core::ffi::c_void, memorylimit: *mut usize) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetRuntimeMemoryLimit(runtime : *const core::ffi::c_void, memorylimit : *mut usize) -> JsErrorCode);
    JsGetRuntimeMemoryLimit(runtime, memorylimit)
}
#[inline]
pub unsafe fn JsGetRuntimeMemoryUsage(runtime: *const core::ffi::c_void, memoryusage: *mut usize) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetRuntimeMemoryUsage(runtime : *const core::ffi::c_void, memoryusage : *mut usize) -> JsErrorCode);
    JsGetRuntimeMemoryUsage(runtime, memoryusage)
}
#[inline]
pub unsafe fn JsGetStringLength(stringvalue: *const core::ffi::c_void, length: *mut i32) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetStringLength(stringvalue : *const core::ffi::c_void, length : *mut i32) -> JsErrorCode);
    JsGetStringLength(stringvalue, length)
}
#[inline]
pub unsafe fn JsGetTrueValue(truevalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetTrueValue(truevalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetTrueValue(truevalue)
}
#[inline]
pub unsafe fn JsGetUndefinedValue(undefinedvalue: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetUndefinedValue(undefinedvalue : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsGetUndefinedValue(undefinedvalue)
}
#[inline]
pub unsafe fn JsGetValueType(value: *const core::ffi::c_void, r#type: *mut JsValueType) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsGetValueType(value : *const core::ffi::c_void, r#type : *mut JsValueType) -> JsErrorCode);
    JsGetValueType(value, r#type)
}
#[inline]
pub unsafe fn JsHasException(hasexception: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsHasException(hasexception : *mut bool) -> JsErrorCode);
    JsHasException(hasexception)
}
#[inline]
pub unsafe fn JsHasExternalData(object: *const core::ffi::c_void, value: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsHasExternalData(object : *const core::ffi::c_void, value : *mut bool) -> JsErrorCode);
    JsHasExternalData(object, value)
}
#[inline]
pub unsafe fn JsHasIndexedProperty(object: *const core::ffi::c_void, index: *const core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsHasIndexedProperty(object : *const core::ffi::c_void, index : *const core::ffi::c_void, result : *mut bool) -> JsErrorCode);
    JsHasIndexedProperty(object, index, result)
}
#[inline]
pub unsafe fn JsHasProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, hasproperty: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsHasProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, hasproperty : *mut bool) -> JsErrorCode);
    JsHasProperty(object, propertyid, hasproperty)
}
#[inline]
pub unsafe fn JsIdle(nextidletick: Option<*mut u32>) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsIdle(nextidletick : *mut u32) -> JsErrorCode);
    JsIdle(core::mem::transmute(nextidletick.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JsIntToNumber(intvalue: i32, value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsIntToNumber(intvalue : i32, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsIntToNumber(intvalue, value)
}
#[inline]
pub unsafe fn JsIsEnumeratingHeap(isenumeratingheap: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsIsEnumeratingHeap(isenumeratingheap : *mut bool) -> JsErrorCode);
    JsIsEnumeratingHeap(isenumeratingheap)
}
#[inline]
pub unsafe fn JsIsRuntimeExecutionDisabled(runtime: *const core::ffi::c_void, isdisabled: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsIsRuntimeExecutionDisabled(runtime : *const core::ffi::c_void, isdisabled : *mut bool) -> JsErrorCode);
    JsIsRuntimeExecutionDisabled(runtime, isdisabled)
}
#[inline]
pub unsafe fn JsNumberToDouble(value: *const core::ffi::c_void, doublevalue: *mut f64) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsNumberToDouble(value : *const core::ffi::c_void, doublevalue : *mut f64) -> JsErrorCode);
    JsNumberToDouble(value, doublevalue)
}
#[inline]
pub unsafe fn JsParseScript<P0, P1>(script: P0, sourcecontext: usize, sourceurl: P1, result: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("chakra.dll" "system" fn JsParseScript(script : windows_core::PCWSTR, sourcecontext : usize, sourceurl : windows_core::PCWSTR, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsParseScript(script.param().abi(), sourcecontext, sourceurl.param().abi(), result)
}
#[inline]
pub unsafe fn JsParseSerializedScript<P0, P1>(script: P0, buffer: *const u8, sourcecontext: usize, sourceurl: P1, result: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("chakra.dll" "system" fn JsParseSerializedScript(script : windows_core::PCWSTR, buffer : *const u8, sourcecontext : usize, sourceurl : windows_core::PCWSTR, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsParseSerializedScript(script.param().abi(), buffer, sourcecontext, sourceurl.param().abi(), result)
}
#[inline]
pub unsafe fn JsPointerToString(stringvalue: &[u16], value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsPointerToString(stringvalue : windows_core::PCWSTR, stringlength : usize, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsPointerToString(core::mem::transmute(stringvalue.as_ptr()), stringvalue.len().try_into().unwrap(), value)
}
#[inline]
pub unsafe fn JsPreventExtension(object: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsPreventExtension(object : *const core::ffi::c_void) -> JsErrorCode);
    JsPreventExtension(object)
}
#[inline]
pub unsafe fn JsRelease(r#ref: *const core::ffi::c_void, count: Option<*mut u32>) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsRelease(r#ref : *const core::ffi::c_void, count : *mut u32) -> JsErrorCode);
    JsRelease(r#ref, core::mem::transmute(count.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JsRunScript<P0, P1>(script: P0, sourcecontext: usize, sourceurl: P1, result: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("chakra.dll" "system" fn JsRunScript(script : windows_core::PCWSTR, sourcecontext : usize, sourceurl : windows_core::PCWSTR, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsRunScript(script.param().abi(), sourcecontext, sourceurl.param().abi(), result)
}
#[inline]
pub unsafe fn JsRunSerializedScript<P0, P1>(script: P0, buffer: *const u8, sourcecontext: usize, sourceurl: P1, result: *mut *mut core::ffi::c_void) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("chakra.dll" "system" fn JsRunSerializedScript(script : windows_core::PCWSTR, buffer : *const u8, sourcecontext : usize, sourceurl : windows_core::PCWSTR, result : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsRunSerializedScript(script.param().abi(), buffer, sourcecontext, sourceurl.param().abi(), result)
}
#[inline]
pub unsafe fn JsSerializeScript<P0>(script: P0, buffer: Option<*mut u8>, buffersize: *mut u32) -> JsErrorCode
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("chakra.dll" "system" fn JsSerializeScript(script : windows_core::PCWSTR, buffer : *mut u8, buffersize : *mut u32) -> JsErrorCode);
    JsSerializeScript(script.param().abi(), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[inline]
pub unsafe fn JsSetCurrentContext(context: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetCurrentContext(context : *const core::ffi::c_void) -> JsErrorCode);
    JsSetCurrentContext(context)
}
#[inline]
pub unsafe fn JsSetException(exception: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetException(exception : *const core::ffi::c_void) -> JsErrorCode);
    JsSetException(exception)
}
#[inline]
pub unsafe fn JsSetExternalData(object: *const core::ffi::c_void, externaldata: Option<*const core::ffi::c_void>) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetExternalData(object : *const core::ffi::c_void, externaldata : *const core::ffi::c_void) -> JsErrorCode);
    JsSetExternalData(object, core::mem::transmute(externaldata.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn JsSetIndexedProperty(object: *const core::ffi::c_void, index: *const core::ffi::c_void, value: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetIndexedProperty(object : *const core::ffi::c_void, index : *const core::ffi::c_void, value : *const core::ffi::c_void) -> JsErrorCode);
    JsSetIndexedProperty(object, index, value)
}
#[inline]
pub unsafe fn JsSetProperty(object: *const core::ffi::c_void, propertyid: *const core::ffi::c_void, value: *const core::ffi::c_void, usestrictrules: u8) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetProperty(object : *const core::ffi::c_void, propertyid : *const core::ffi::c_void, value : *const core::ffi::c_void, usestrictrules : u8) -> JsErrorCode);
    JsSetProperty(object, propertyid, value, usestrictrules)
}
#[inline]
pub unsafe fn JsSetPrototype(object: *const core::ffi::c_void, prototypeobject: *const core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetPrototype(object : *const core::ffi::c_void, prototypeobject : *const core::ffi::c_void) -> JsErrorCode);
    JsSetPrototype(object, prototypeobject)
}
#[inline]
pub unsafe fn JsSetRuntimeBeforeCollectCallback(runtime: *const core::ffi::c_void, callbackstate: Option<*const core::ffi::c_void>, beforecollectcallback: JsBeforeCollectCallback) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetRuntimeBeforeCollectCallback(runtime : *const core::ffi::c_void, callbackstate : *const core::ffi::c_void, beforecollectcallback : JsBeforeCollectCallback) -> JsErrorCode);
    JsSetRuntimeBeforeCollectCallback(runtime, core::mem::transmute(callbackstate.unwrap_or(std::ptr::null())), beforecollectcallback)
}
#[inline]
pub unsafe fn JsSetRuntimeMemoryAllocationCallback(runtime: *const core::ffi::c_void, callbackstate: Option<*const core::ffi::c_void>, allocationcallback: JsMemoryAllocationCallback) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetRuntimeMemoryAllocationCallback(runtime : *const core::ffi::c_void, callbackstate : *const core::ffi::c_void, allocationcallback : JsMemoryAllocationCallback) -> JsErrorCode);
    JsSetRuntimeMemoryAllocationCallback(runtime, core::mem::transmute(callbackstate.unwrap_or(std::ptr::null())), allocationcallback)
}
#[inline]
pub unsafe fn JsSetRuntimeMemoryLimit(runtime: *const core::ffi::c_void, memorylimit: usize) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsSetRuntimeMemoryLimit(runtime : *const core::ffi::c_void, memorylimit : usize) -> JsErrorCode);
    JsSetRuntimeMemoryLimit(runtime, memorylimit)
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsStartDebugging<P0>(debugapplication: P0) -> JsErrorCode
where
    P0: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IDebugApplication64>,
{
    windows_targets::link!("chakra.dll" "system" fn JsStartDebugging(debugapplication : * mut core::ffi::c_void) -> JsErrorCode);
    JsStartDebugging(debugapplication.param().abi())
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsStartDebugging<P0>(debugapplication: P0) -> JsErrorCode
where
    P0: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IDebugApplication32>,
{
    windows_targets::link!("chakra.dll" "system" fn JsStartDebugging(debugapplication : * mut core::ffi::c_void) -> JsErrorCode);
    JsStartDebugging(debugapplication.param().abi())
}
#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
#[inline]
pub unsafe fn JsStartProfiling<P0>(callback: P0, eventmask: super::Diagnostics::Debug::ActiveScript::PROFILER_EVENT_MASK, context: u32) -> JsErrorCode
where
    P0: windows_core::Param<super::Diagnostics::Debug::ActiveScript::IActiveScriptProfilerCallback>,
{
    windows_targets::link!("chakra.dll" "system" fn JsStartProfiling(callback : * mut core::ffi::c_void, eventmask : super::Diagnostics::Debug::ActiveScript:: PROFILER_EVENT_MASK, context : u32) -> JsErrorCode);
    JsStartProfiling(callback.param().abi(), eventmask, context)
}
#[inline]
pub unsafe fn JsStopProfiling(reason: windows_core::HRESULT) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsStopProfiling(reason : windows_core::HRESULT) -> JsErrorCode);
    JsStopProfiling(reason)
}
#[inline]
pub unsafe fn JsStrictEquals(object1: *const core::ffi::c_void, object2: *const core::ffi::c_void, result: *mut bool) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsStrictEquals(object1 : *const core::ffi::c_void, object2 : *const core::ffi::c_void, result : *mut bool) -> JsErrorCode);
    JsStrictEquals(object1, object2, result)
}
#[inline]
pub unsafe fn JsStringToPointer(value: *const core::ffi::c_void, stringvalue: *mut *mut u16, stringlength: *mut usize) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsStringToPointer(value : *const core::ffi::c_void, stringvalue : *mut *mut u16, stringlength : *mut usize) -> JsErrorCode);
    JsStringToPointer(value, stringvalue, stringlength)
}
#[inline]
pub unsafe fn JsValueToVariant(object: *const core::ffi::c_void, variant: *mut windows_core::VARIANT) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsValueToVariant(object : *const core::ffi::c_void, variant : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> JsErrorCode);
    JsValueToVariant(object, core::mem::transmute(variant))
}
#[inline]
pub unsafe fn JsVariantToValue(variant: *const windows_core::VARIANT, value: *mut *mut core::ffi::c_void) -> JsErrorCode {
    windows_targets::link!("chakra.dll" "system" fn JsVariantToValue(variant : *const core::mem::MaybeUninit < windows_core::VARIANT >, value : *mut *mut core::ffi::c_void) -> JsErrorCode);
    JsVariantToValue(core::mem::transmute(variant), value)
}
pub const JS_SOURCE_CONTEXT_NONE: u64 = 18446744073709551615u64;
pub const JsArray: JsValueType = JsValueType(8i32);
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
pub const JsFunction: JsValueType = JsValueType(6i32);
pub const JsMemoryAllocate: JsMemoryEventType = JsMemoryEventType(0i32);
pub const JsMemoryFailure: JsMemoryEventType = JsMemoryEventType(2i32);
pub const JsMemoryFree: JsMemoryEventType = JsMemoryEventType(1i32);
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
pub const JsRuntimeVersion10: JsRuntimeVersion = JsRuntimeVersion(0i32);
pub const JsRuntimeVersion11: JsRuntimeVersion = JsRuntimeVersion(1i32);
pub const JsRuntimeVersionEdge: JsRuntimeVersion = JsRuntimeVersion(-1i32);
pub const JsString: JsValueType = JsValueType(3i32);
pub const JsUndefined: JsValueType = JsValueType(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JsErrorCode(pub u32);
impl windows_core::TypeKind for JsErrorCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JsErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JsErrorCode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JsMemoryEventType(pub i32);
impl windows_core::TypeKind for JsMemoryEventType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JsMemoryEventType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JsMemoryEventType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JsRuntimeAttributes(pub i32);
impl windows_core::TypeKind for JsRuntimeAttributes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JsRuntimeAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JsRuntimeAttributes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JsRuntimeVersion(pub i32);
impl windows_core::TypeKind for JsRuntimeVersion {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JsRuntimeVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JsRuntimeVersion").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JsValueType(pub i32);
impl windows_core::TypeKind for JsValueType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JsValueType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JsValueType").field(&self.0).finish()
    }
}
pub type JsBackgroundWorkItemCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void)>;
pub type JsBeforeCollectCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void)>;
pub type JsFinalizeCallback = Option<unsafe extern "system" fn(data: *const core::ffi::c_void)>;
pub type JsMemoryAllocationCallback = Option<unsafe extern "system" fn(callbackstate: *const core::ffi::c_void, allocationevent: JsMemoryEventType, allocationsize: usize) -> bool>;
pub type JsNativeFunction = Option<unsafe extern "system" fn(callee: *const core::ffi::c_void, isconstructcall: bool, arguments: *const *const core::ffi::c_void, argumentcount: u16, callbackstate: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type JsThreadServiceCallback = Option<unsafe extern "system" fn(callback: JsBackgroundWorkItemCallback, callbackstate: *const core::ffi::c_void) -> bool>;
