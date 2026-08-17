// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use std::cell::Cell;
use std::collections::HashMap;
use std::env::var;
// (header name, &[header dependencies], &[library dependencies])
const DATA: &'static [(&'static str, &'static [&'static str], &'static [&'static str])] = &[
    // km
    ("d3dkmthk", &["basetsd", "d3dukmdt", "minwindef", "ntdef", "windef"], &[]),
    // mmos
    // shared
    ("basetsd", &[], &[]),
    ("bcrypt", &["minwindef", "winnt"], &["bcrypt"]),
    ("bthdef", &["bthsdpdef", "guiddef", "minwindef", "ntdef"], &[]),
    ("bthioctl", &["bthdef", "bthsdpdef", "minwindef", "ntdef", "winioctl"], &[]),
    ("bthsdpdef", &["guiddef", "minwindef", "ntdef"], &[]),
    ("bugcodes", &["ntdef"], &[]),
    ("cderr", &["minwindef"], &[]),
    ("cfg", &["minwindef"], &[]),
    ("d3d9", &["basetsd", "d3d9caps", "d3d9types", "guiddef", "minwindef", "unknwnbase", "windef", "wingdi", "winnt"], &["d3d9"]),
    ("d3d9caps", &["d3d9types", "guiddef", "minwindef", "winnt"], &[]),
    ("d3d9types", &["basetsd", "guiddef", "minwindef", "windef", "winnt"], &[]),
    ("d3dkmdt", &["basetsd", "minwindef", "ntdef"], &[]),
    ("d3dukmdt", &["basetsd", "guiddef", "minwindef", "ntdef"], &[]),
    ("dcomptypes", &["dxgitype", "minwindef", "winnt"], &[]),
    ("devguid", &[], &[]),
    ("devpkey", &["devpropdef"], &[]),
    ("devpropdef", &["guiddef", "minwindef", "winnt"], &[]),
    ("dinputd", &[], &[]),
    ("dxgi", &["basetsd", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &["dxgi"]),
    ("dxgi1_2", &["basetsd", "dxgi", "dxgiformat", "dxgitype", "guiddef", "minwinbase", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("dxgi1_3", &["dxgi", "dxgi1_2", "dxgiformat", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &["dxgi"]),
    ("dxgi1_4", &["basetsd", "dxgi1_2", "dxgi1_3", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("dxgi1_5", &["basetsd", "dxgi", "dxgi1_2", "dxgi1_3", "dxgi1_4", "dxgiformat", "minwindef", "unknwnbase", "winnt"], &[]),
    ("dxgi1_6", &["basetsd", "dxgi1_2", "dxgi1_4", "dxgi1_5", "dxgitype", "guiddef", "minwindef", "windef", "winnt"], &[]),
    ("dxgiformat", &[], &[]),
    ("dxgitype", &["d3d9types", "dxgiformat", "minwindef"], &[]),
    ("enclaveapi", &["basetsd", "minwinbase", "minwindef", "ntdef", "winnt"], &["kernel32"]),
    ("evntprov", &["basetsd", "guiddef", "minwindef", "winnt"], &["advapi32"]),
    ("evntrace", &["basetsd", "evntcons", "evntprov", "guiddef", "handleapi", "minwindef", "timezoneapi", "vadefs", "winnt", "wmistr"], &["advapi32"]),
    ("guiddef", &[], &[]),
    ("hidclass", &["guiddef", "minwindef", "winioctl", "winnt"], &[]),
    ("hidpi", &["hidusage", "minwindef", "ntdef", "ntstatus", "winnt"], &["hid"]),
    ("hidsdi", &["guiddef", "hidpi", "minwindef", "winnt"], &["hid"]),
    ("hidusage", &["minwindef"], &[]),
    ("ifdef", &["basetsd", "guiddef", "ntdef"], &[]),
    ("ifmib", &["ifdef", "ipifcons", "minwindef", "ntdef"], &[]),
    ("in6addr", &["minwindef"], &[]),
    ("inaddr", &["minwindef"], &[]),
    ("intsafe", &[], &[]),
    ("ipifcons", &["minwindef"], &[]),
    ("ipmib", &["ifdef", "ifmib", "minwindef", "nldef", "ntdef"], &[]),
    ("iprtrmib", &["ipmib", "minwindef", "ntdef"], &[]),
    ("ks", &[], &[]),
    ("ksmedia", &["minwindef"], &[]),
    ("ktmtypes", &["guiddef", "minwindef", "winnt"], &[]),
    ("lmcons", &["minwindef", "winnt"], &[]),
    ("minwindef", &["basetsd", "ntdef"], &[]),
    ("mmreg", &["guiddef", "minwindef"], &[]),
    ("mprapidef", &[], &[]),
    ("mstcpip", &["basetsd", "guiddef", "in6addr", "inaddr", "minwindef", "winnt", "ws2def"], &["ntdll"]),
    ("mswsockdef", &["minwindef", "winnt", "ws2def"], &[]),
    ("netioapi", &["basetsd", "guiddef", "ifdef", "ipifcons", "minwindef", "nldef", "ntddndis", "ntdef", "ws2def", "ws2ipdef"], &["iphlpapi"]),
    ("nldef", &["basetsd", "minwindef", "ntdef"], &[]),
    ("ntddndis", &["ifdef", "minwindef"], &[]),
    ("ntddscsi", &["basetsd", "minwindef", "ntdef", "winioctl", "winnt"], &[]),
    ("ntddser", &["devpropdef"], &[]),
    ("ntdef", &["basetsd", "guiddef"], &[]),
    ("ntstatus", &["ntdef"], &[]),
    ("qos", &["minwindef"], &[]),
    ("rpc", &[], &[]),
    ("rpcdce", &["guiddef", "minwindef", "rpc"], &[]),
    ("rpcndr", &[], &[]),
    ("sddl", &["basetsd", "minwindef", "winnt"], &["advapi32"]),
    ("sspi", &["basetsd", "guiddef", "minwindef", "subauth", "wincred", "winnt"], &["credui", "secur32"]),
    ("stralign", &["vcruntime", "winnt"], &["kernel32"]),
    ("tcpestats", &["basetsd", "ntdef"], &[]),
    ("tcpmib", &["basetsd", "in6addr", "minwindef", "ntdef"], &[]),
    ("transportsettingcommon", &["guiddef"], &[]),
    ("tvout", &["guiddef", "minwindef"], &[]),
    ("udpmib", &["basetsd", "in6addr", "minwindef", "ntdef"], &[]),
    ("usb", &["minwindef", "usbspec", "winnt"], &[]),
    ("usbioctl", &["basetsd", "guiddef", "minwindef", "ntdef", "usb", "usbiodef", "usbspec", "winioctl"], &[]),
    ("usbiodef", &["guiddef", "minwindef", "winioctl", "winnt"], &[]),
    ("usbscan", &["ntdef", "winioctl"], &[]),
    ("usbspec", &["basetsd", "guiddef", "minwindef", "winnt"], &[]),
    ("windef", &["minwindef", "winnt"], &[]),
    ("windot11", &["basetsd", "minwindef", "ntddndis", "winnt", "wlantypes"], &[]),
    ("windowsx", &["minwindef"], &[]),
    ("winerror", &["minwindef", "wtypesbase"], &[]),
    ("winusbio", &["minwindef", "usb"], &[]),
    ("wlantypes", &["basetsd", "minwindef"], &[]),
    ("wmistr", &["basetsd", "guiddef", "minwindef", "winnt"], &[]),
    ("wnnc", &["minwindef"], &[]),
    ("ws2def", &["basetsd", "guiddef", "inaddr", "minwindef", "vcruntime", "winnt"], &[]),
    ("ws2ipdef", &["in6addr", "inaddr", "minwindef", "ws2def"], &[]),
    ("wtypes", &["guiddef", "minwindef", "ntdef", "rpcndr", "wingdi", "wtypesbase"], &[]),
    ("wtypesbase", &["minwindef", "rpcndr", "winnt"], &[]),
    // ucrt
    ("corecrt", &[], &[]),
    // um
    ("accctrl", &["guiddef", "minwindef", "winbase", "winnt"], &[]),
    ("aclapi", &["accctrl", "guiddef", "minwindef", "winnt"], &["advapi32"]),
    ("adhoc", &["guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("appmgmt", &["guiddef", "minwindef", "winnt"], &["advapi32"]),
    ("audioclient", &["audiosessiontypes", "basetsd", "guiddef", "minwindef", "mmreg", "strmif", "unknwnbase", "winerror", "winnt", "wtypesbase"], &[]),
    ("audiosessiontypes", &["minwindef"], &[]),
    ("avrt", &["guiddef", "minwindef", "winnt"], &["avrt"]),
    ("bits", &["basetsd", "guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("bits10_1", &["basetsd", "bits", "bits2_0", "bits3_0", "bits5_0", "minwindef", "winnt"], &[]),
    ("bits1_5", &["basetsd", "bits", "rpcndr", "winnt"], &[]),
    ("bits2_0", &["basetsd", "bits", "bits1_5", "minwindef", "winnt"], &[]),
    ("bits2_5", &["minwindef", "rpcndr", "unknwnbase", "winnt"], &[]),
    ("bits3_0", &["basetsd", "bits", "bits2_0", "guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("bits4_0", &["basetsd", "bits3_0", "minwindef", "unknwnbase", "winnt"], &[]),
    ("bits5_0", &["basetsd", "bits1_5", "bits3_0", "bits4_0", "guiddef", "minwindef", "winnt"], &[]),
    ("bitscfg", &["guiddef", "oaidl", "unknwnbase", "winnt", "wtypes"], &["oleaut32"]),
    ("bitsmsg", &["minwindef"], &[]),
    ("bluetoothapis", &["bthdef", "bthsdpdef", "guiddef", "minwinbase", "minwindef", "windef", "winnt"], &["bthprops"]),
    ("bluetoothleapis", &["bthledef", "minwindef", "winerror", "winnt"], &["bluetoothapis"]),
    ("bthledef", &["basetsd", "guiddef", "minwindef", "winnt"], &[]),
    ("cfgmgr32", &["basetsd", "cfg", "devpropdef", "guiddef", "minwindef", "winnt", "winreg"], &["cfgmgr32"]),
    ("cguid", &[], &[]),
    ("combaseapi", &["basetsd", "guiddef", "minwindef", "objidl", "objidlbase", "propidl", "rpcdce", "unknwnbase", "winnt", "wtypesbase"], &["ole32"]),
    ("coml2api", &["minwindef"], &[]),
    ("commapi", &["minwinbase", "minwindef", "winbase", "winnt"], &["kernel32"]),
    ("commctrl", &["basetsd", "commoncontrols", "guiddef", "minwinbase", "minwindef", "vcruntime", "windef", "winnt", "winuser"], &["comctl32"]),
    ("commdlg", &["basetsd", "minwindef", "prsht", "unknwnbase", "windef", "wingdi", "winnt", "winuser"], &["comdlg32"]),
    ("commoncontrols", &["commctrl", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &["comctl32"]),
    ("consoleapi", &["minwindef", "wincon", "wincontypes", "winnt"], &["kernel32"]),
    ("corsym", &["basetsd", "objidlbase", "unknwnbase", "winnt"], &[]),
    ("d2d1", &["basetsd", "d2dbasetypes", "d3dcommon", "dcommon", "dwrite", "dxgi", "guiddef", "minwindef", "unknwnbase", "wincodec", "windef", "winnt"], &["d2d1"]),
    ("d2d1_1", &["basetsd", "d2d1", "d2d1effectauthor", "d2dbasetypes", "dcommon", "documenttarget", "dwrite", "dxgi", "dxgiformat", "guiddef", "minwindef", "objidlbase", "unknwnbase", "wincodec", "winnt"], &["d2d1"]),
    ("d2d1_2", &["d2d1", "d2d1_1", "dxgi", "minwindef", "winnt"], &["d2d1"]),
    ("d2d1_3", &["basetsd", "d2d1", "d2d1_1", "d2d1_2", "d2d1effects", "d2d1svg", "dcommon", "dwrite", "dxgi", "dxgitype", "minwindef", "ntdef", "objidlbase", "wincodec", "winerror"], &["d2d1"]),
    ("d2d1effectauthor", &["basetsd", "d2d1", "d2d1_1", "d2dbasetypes", "d3dcommon", "dxgiformat", "guiddef", "minwindef", "ntdef", "unknwnbase", "wincodec"], &[]),
    ("d2d1effects", &[], &[]),
    ("d2d1effects_1", &[], &[]),
    ("d2d1effects_2", &[], &[]),
    ("d2d1svg", &["basetsd", "d2d1", "d2d1_1", "guiddef", "minwindef", "ntdef", "objidlbase", "winerror"], &[]),
    ("d2dbasetypes", &["d3d9types", "dcommon"], &[]),
    ("d3d", &[], &[]),
    ("d3d10", &["d3dcommon"], &[]),
    ("d3d10_1", &[], &[]),
    ("d3d10_1shader", &[], &[]),
    ("d3d10effect", &[], &[]),
    ("d3d10misc", &[], &[]),
    ("d3d10sdklayers", &[], &[]),
    ("d3d10shader", &["d3d10", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d11", &["basetsd", "d3dcommon", "dxgi", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &["d3d11"]),
    ("d3d11_1", &["basetsd", "d3d11", "d3dcommon", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d11_2", &["basetsd", "d3d11", "d3d11_1", "dxgiformat", "minwindef", "winnt"], &[]),
    ("d3d11_3", &[], &[]),
    ("d3d11_4", &[], &[]),
    ("d3d11on12", &["d3d11", "d3d12", "d3dcommon", "guiddef", "minwindef", "unknwnbase", "winnt"], &["d3d11"]),
    ("d3d11sdklayers", &["basetsd", "d3d11", "dxgi", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d11shader", &["basetsd", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d11tokenizedprogramformat", &["minwindef"], &[]),
    ("d3d12", &["basetsd", "d3dcommon", "dxgiformat", "dxgitype", "guiddef", "minwinbase", "minwindef", "unknwnbase", "windef", "winnt"], &["d3d12"]),
    ("d3d12sdklayers", &["basetsd", "d3d12", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d12shader", &["basetsd", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3dcommon", &["basetsd", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3dcompiler", &["basetsd", "d3d11shader", "d3dcommon", "guiddef", "minwindef", "winnt"], &["d3dcompiler"]),
    ("d3dcsx", &[], &[]),
    ("d3dx10core", &[], &[]),
    ("d3dx10math", &[], &[]),
    ("d3dx10mesh", &[], &[]),
    ("datetimeapi", &["minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("davclnt", &["minwindef", "winnt"], &["netapi32"]),
    ("dbghelp", &["basetsd", "guiddef", "minwindef", "vcruntime", "winnt"], &["dbghelp"]),
    ("dbt", &["basetsd", "guiddef", "minwindef", "winnt", "winuser"], &[]),
    ("dcommon", &["basetsd", "dxgiformat", "minwindef", "windef"], &[]),
    ("dcomp", &["d2d1", "d2d1_1", "d2d1effects", "d2dbasetypes", "d3d9types", "d3dcommon", "dcompanimation", "dcomptypes", "dxgi", "dxgi1_2", "dxgiformat", "guiddef", "minwinbase", "minwindef", "ntdef", "unknwnbase", "windef"], &["dcomp"]),
    ("dcompanimation", &["ntdef", "unknwnbase"], &[]),
    ("dde", &["basetsd", "minwindef"], &["user32"]),
    ("ddraw", &[], &[]),
    ("ddrawi", &[], &[]),
    ("ddrawint", &[], &[]),
    ("debugapi", &["minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("devicetopology", &["guiddef", "minwindef", "unknwnbase", "windef", "winnt", "wtypes"], &[]),
    ("dinput", &[], &[]),
    ("dispex", &["basetsd", "guiddef", "minwindef", "oaidl", "servprov", "unknwnbase", "winerror", "winnt", "wtypes"], &[]),
    ("dmksctl", &[], &[]),
    ("dmusicc", &[], &[]),
    ("docobj", &["guiddef", "minwindef", "oaidl", "unknwnbase", "winnt"], &[]),
    ("documenttarget", &["basetsd", "guiddef", "ntdef", "unknwnbase"], &[]),
    ("dot1x", &["eaptypes", "guiddef", "l2cmn", "minwindef", "winnt"], &[]),
    ("dpa_dsa", &["basetsd", "minwindef", "winnt"], &["comctl32"]),
    ("dpapi", &["minwindef", "wincrypt", "windef", "winnt"], &["crypt32"]),
    ("dsgetdc", &["guiddef", "minwindef", "ntsecapi", "winnt", "ws2def"], &["netapi32"]),
    ("dsound", &["guiddef", "minwindef", "mmsystem", "unknwnbase", "windef", "winerror", "winnt"], &["dsound"]),
    ("dsrole", &["guiddef", "minwindef", "winnt"], &["netapi32"]),
    ("dvp", &[], &[]),
    ("dwmapi", &["basetsd", "minwindef", "uxtheme", "windef", "winnt"], &["dwmapi"]),
    ("dwrite", &["basetsd", "d2d1", "dcommon", "guiddef", "minwindef", "unknwnbase", "windef", "winerror", "wingdi", "winnt"], &["dwrite"]),
    ("dwrite_1", &["basetsd", "dcommon", "dwrite", "minwindef", "winnt"], &[]),
    ("dwrite_2", &["basetsd", "d3d9types", "dcommon", "dwrite", "dwrite_1", "minwindef", "unknwnbase", "winnt"], &[]),
    ("dwrite_3", &["basetsd", "dcommon", "dwrite", "dwrite_1", "dwrite_2", "minwindef", "unknwnbase", "wingdi", "winnt"], &[]),
    ("dxdiag", &[], &[]),
    ("dxfile", &[], &[]),
    ("dxgidebug", &["basetsd", "guiddef", "minwindef", "unknwnbase", "winnt"], &["dxgi"]),
    ("dxva2api", &["basetsd", "d3d9", "d3d9types", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &["dxva2"]),
    ("dxvahd", &["d3d9", "d3d9types", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &["dxva2"]),
    ("eaptypes", &["guiddef", "minwindef", "winnt"], &[]),
    ("endpointvolume", &["basetsd", "guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("errhandlingapi", &["basetsd", "minwindef", "winnt"], &["kernel32"]),
    ("evntcons", &["basetsd", "evntprov", "evntrace", "guiddef", "minwindef", "winnt"], &["advapi32"]),
    ("exdisp", &["basetsd", "docobj", "oaidl", "ocidl", "winnt", "wtypes"], &[]),
    ("fibersapi", &["minwindef", "winnt"], &["kernel32"]),
    ("fileapi", &["minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("functiondiscoverykeys_devpkey", &["wtypes"], &[]),
    ("gl-gl", &[], &["opengl32"]),
    ("handleapi", &["minwindef", "winnt"], &["kernel32"]),
    ("heapapi", &["basetsd", "minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("highlevelmonitorconfigurationapi", &["minwindef", "physicalmonitorenumerationapi", "winnt"], &["dxva2"]),
    ("http", &["guiddef", "minwinbase", "minwindef", "sspi", "winnt", "ws2def"], &["httpapi"]),
    ("imm", &["minwindef", "windef"], &["imm32"]),
    ("interlockedapi", &["minwindef", "winnt"], &["kernel32"]),
    ("ioapiset", &["basetsd", "minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("ipexport", &["basetsd", "in6addr", "ntdef"], &[]),
    ("iphlpapi", &["basetsd", "ifdef", "ifmib", "ipexport", "ipmib", "iprtrmib", "iptypes", "minwinbase", "minwindef", "ntdef", "tcpestats", "tcpmib", "udpmib", "ws2def", "ws2ipdef"], &["iphlpapi"]),
    ("iptypes", &["basetsd", "corecrt", "guiddef", "ifdef", "ipifcons", "minwindef", "nldef", "ntdef", "ws2def"], &[]),
    ("jobapi", &["minwindef", "winnt"], &["kernel32"]),
    ("jobapi2", &["basetsd", "minwinbase", "minwindef", "ntdef", "winnt"], &["kernel32"]),
    ("knownfolders", &[], &[]),
    ("ktmw32", &["guiddef", "minwinbase", "minwindef", "winnt"], &["ktmw32"]),
    ("l2cmn", &["guiddef", "minwindef", "winnt"], &[]),
    ("libloaderapi", &["basetsd", "minwindef", "winnt"], &["kernel32", "user32"]),
    ("lmaccess", &["basetsd", "lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmalert", &["lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmapibuf", &["lmcons", "minwindef"], &["netapi32"]),
    ("lmat", &["basetsd", "lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmdfs", &["guiddef", "lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmerrlog", &["minwindef", "winnt"], &[]),
    ("lmjoin", &["lmcons", "minwindef", "wincrypt", "winnt"], &["netapi32"]),
    ("lmmsg", &["lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmremutl", &["lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmrepl", &["lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmserver", &["guiddef", "lmcons", "minwindef", "winnt", "winsvc"], &["advapi32", "netapi32"]),
    ("lmshare", &["basetsd", "guiddef", "lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmstats", &["lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmsvc", &["lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmuse", &["lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lmwksta", &["lmcons", "minwindef", "winnt"], &["netapi32"]),
    ("lowlevelmonitorconfigurationapi", &["minwindef", "physicalmonitorenumerationapi", "winnt"], &["dxva2"]),
    ("lsalookup", &["guiddef", "minwindef", "ntdef", "winnt"], &["advapi32"]),
    ("memoryapi", &["basetsd", "minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("minschannel", &["guiddef", "minwindef", "wincrypt", "winnt"], &[]),
    ("minwinbase", &["basetsd", "minwindef", "ntstatus", "winnt"], &[]),
    ("mmdeviceapi", &["guiddef", "minwindef", "propidl", "propsys", "unknwnbase", "winnt", "wtypes"], &["mmdevapi"]),
    ("mmeapi", &["basetsd", "imm", "minwindef", "mmsystem", "winnt"], &["winmm"]),
    ("mmsystem", &["basetsd", "minwindef", "mmreg", "winnt"], &[]),
    ("msaatext", &[], &[]),
    ("mscat", &["guiddef", "minwindef", "mssip", "wincrypt", "winnt"], &[]),
    ("mschapp", &["basetsd", "minwindef", "winnt"], &["advapi32"]),
    ("mssip", &["guiddef", "minwindef", "mscat", "wincrypt", "winnt"], &["crypt32"]),
    ("mswsock", &["minwinbase", "minwindef", "mswsockdef", "winnt", "winsock2", "ws2def"], &["mswsock"]),
    ("namedpipeapi", &["minwinbase", "minwindef", "winnt"], &["advapi32", "kernel32"]),
    ("namespaceapi", &["minwinbase", "minwindef", "ntdef", "winnt"], &["kernel32"]),
    ("nb30", &["minwindef", "winnt"], &["netapi32"]),
    ("ncrypt", &["basetsd", "bcrypt", "minwindef", "winnt"], &["ncrypt"]),
    ("ntlsa", &["basetsd", "guiddef", "lsalookup", "minwindef", "ntdef", "ntsecapi", "subauth", "winnt"], &["advapi32"]),
    ("ntsecapi", &["basetsd", "guiddef", "lsalookup", "minwindef", "ntdef", "sspi", "subauth", "winnt"], &["advapi32"]),
    ("oaidl", &["basetsd", "guiddef", "minwindef", "rpcndr", "unknwnbase", "winnt", "wtypes", "wtypesbase"], &[]),
    ("objbase", &["combaseapi", "minwindef", "winnt"], &["ole32"]),
    ("objidl", &["basetsd", "guiddef", "minwindef", "ntdef", "objidlbase", "unknwnbase", "windef", "winnt", "wtypes", "wtypesbase"], &[]),
    ("objidlbase", &["basetsd", "guiddef", "minwindef", "unknwnbase", "winnt", "wtypesbase"], &[]),
    ("ocidl", &["guiddef", "minwindef", "ntdef", "oaidl", "unknwnbase", "wtypes", "wtypesbase"], &[]),
    ("ole2", &["minwindef", "oleidl", "windef", "winnt"], &["ole32"]),
    ("oleauto", &["basetsd", "minwinbase", "minwindef", "oaidl", "winnt", "wtypes", "wtypesbase"], &["oleaut32"]),
    ("olectl", &["winerror", "winnt"], &[]),
    ("oleidl", &["minwindef", "ntdef", "objidl", "unknwnbase", "windef"], &[]),
    ("opmapi", &["basetsd", "d3d9", "d3d9types", "dxva2api", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &["dxva2"]),
    ("pdh", &["basetsd", "guiddef", "minwindef", "windef", "winnt"], &["pdh"]),
    ("perflib", &["basetsd", "guiddef", "minwinbase", "minwindef", "winnt"], &["advapi32"]),
    ("physicalmonitorenumerationapi", &["d3d9", "minwindef", "windef", "winnt"], &["dxva2"]),
    ("playsoundapi", &["minwindef", "winnt"], &["winmm"]),
    ("portabledevice", &["basetsd", "wtypes"], &[]),
    ("portabledeviceapi", &["guiddef", "minwindef", "objidlbase", "portabledevicetypes", "propkeydef", "unknwnbase", "winnt"], &[]),
    ("portabledevicetypes", &["guiddef", "minwindef", "propidl", "propkeydef", "propsys", "unknwnbase", "winnt", "wtypes"], &[]),
    ("powerbase", &["minwindef", "winnt", "winuser"], &["powrprof"]),
    ("powersetting", &["guiddef", "minwindef", "winnt", "winuser"], &["powrprof"]),
    ("powrprof", &["guiddef", "minwindef", "winnt", "winreg"], &["powrprof"]),
    ("processenv", &["minwindef", "winnt"], &["kernel32"]),
    ("processsnapshot", &["basetsd", "minwindef", "winnt"], &["kernel32"]),
    ("processthreadsapi", &["basetsd", "guiddef", "minwinbase", "minwindef", "winnt"], &["advapi32", "kernel32"]),
    ("processtopologyapi", &["minwindef", "winnt"], &["kernel32"]),
    ("profileapi", &["minwindef", "winnt"], &["kernel32"]),
    ("propidl", &["guiddef", "minwindef", "ntdef", "oaidl", "objidlbase", "unknwnbase", "wtypes", "wtypesbase"], &["ole32"]),
    ("propkey", &["minwindef", "ntdef", "wtypes"], &[]),
    ("propkeydef", &["guiddef", "wtypes"], &[]),
    ("propsys", &["minwindef", "propidl", "propkeydef", "unknwnbase", "winnt", "wtypes"], &[]),
    ("prsht", &["basetsd", "minwindef", "windef", "winnt", "winuser"], &["comctl32"]),
    ("psapi", &["basetsd", "minwindef", "winnt"], &["kernel32", "psapi"]),
    ("realtimeapiset", &["basetsd", "minwindef", "winnt"], &["kernel32"]),
    ("reason", &["minwindef"], &[]),
    ("restartmanager", &["minwindef", "winnt"], &["rstrtmgr"]),
    ("restrictederrorinfo", &["unknwnbase", "winnt", "wtypes"], &[]),
    ("rmxfguid", &[], &[]),
    ("rtinfo", &["basetsd"], &[]),
    ("sapi", &["guiddef", "minwindef", "sapi53", "unknwnbase", "winnt"], &[]),
    ("sapi51", &["guiddef", "minwindef", "mmreg", "oaidl", "objidlbase", "rpcndr", "servprov", "unknwnbase", "windef", "winnt", "wtypes", "wtypesbase"], &[]),
    ("sapi53", &["guiddef", "minwindef", "oaidl", "sapi51", "unknwnbase", "urlmon", "winnt", "wtypes"], &[]),
    ("sapiddk", &["guiddef", "minwindef", "sapi", "sapiddk51", "unknwnbase", "winnt"], &[]),
    ("sapiddk51", &["guiddef", "minwindef", "mmreg", "oaidl", "objidlbase", "sapi", "unknwnbase", "windef", "winnt"], &[]),
    ("schannel", &["guiddef", "minwindef", "wincrypt", "windef", "winnt"], &[]),
    ("securityappcontainer", &["minwindef", "winnt"], &["kernel32"]),
    ("securitybaseapi", &["guiddef", "minwinbase", "minwindef", "winnt"], &["advapi32", "kernel32"]),
    ("servprov", &["guiddef", "unknwnbase", "winnt"], &[]),
    ("setupapi", &["basetsd", "commctrl", "devpropdef", "guiddef", "minwindef", "prsht", "spapidef", "windef", "winnt", "winreg"], &["setupapi"]),
    ("shellapi", &["basetsd", "guiddef", "minwinbase", "minwindef", "processthreadsapi", "windef", "winnt", "winuser"], &["shell32", "shlwapi"]),
    ("shellscalingapi", &["minwindef", "windef", "winnt"], &["shcore"]),
    ("shlobj", &["guiddef", "minwinbase", "minwindef", "shtypes", "windef", "winnt"], &["shell32"]),
    ("shobjidl", &["guiddef", "minwindef", "propsys", "shobjidl_core", "shtypes", "unknwnbase", "windef", "winnt"], &[]),
    ("shobjidl_core", &["commctrl", "guiddef", "minwinbase", "minwindef", "objidl", "propkeydef", "propsys", "shtypes", "unknwnbase", "windef", "winnt"], &[]),
    ("shtypes", &["guiddef", "minwindef", "winnt"], &[]),
    ("softpub", &[], &[]),
    ("spapidef", &["minwindef", "winnt"], &[]),
    ("spellcheck", &["minwindef", "ntdef", "objidlbase", "unknwnbase", "winerror"], &[]),
    ("sporder", &["guiddef", "minwindef"], &["sporder"]),
    ("sql", &["sqltypes"], &["odbc32"]),
    ("sqlext", &["sql", "sqltypes"], &[]),
    ("sqltypes", &["basetsd", "guiddef", "windef"], &[]),
    ("sqlucode", &["sqltypes"], &["odbc32"]),
    ("stringapiset", &["minwindef", "winnls", "winnt"], &["kernel32"]),
    ("strmif", &["winnt"], &[]),
    ("subauth", &["minwindef", "winnt"], &[]),
    ("synchapi", &["basetsd", "minwinbase", "minwindef", "winnt"], &["kernel32", "synchronization"]),
    ("sysinfoapi", &["basetsd", "minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("systemtopologyapi", &["minwindef", "winnt"], &["kernel32"]),
    ("taskschd", &["minwinbase", "minwindef", "oaidl", "unknwnbase", "winnt", "wtypes"], &[]),
    ("textstor", &[], &[]),
    ("threadpoolapiset", &["basetsd", "minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("threadpoollegacyapiset", &["minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("timeapi", &["minwindef", "mmsystem"], &["winmm"]),
    ("timezoneapi", &["minwinbase", "minwindef", "winnt"], &["advapi32", "kernel32"]),
    ("tlhelp32", &["basetsd", "minwindef", "winnt"], &["kernel32"]),
    ("unknwnbase", &["guiddef", "minwindef", "winnt"], &[]),
    ("urlhist", &["docobj", "guiddef", "minwindef", "unknwnbase", "winnt", "wtypesbase"], &[]),
    ("urlmon", &["minwindef", "unknwnbase", "winnt"], &[]),
    ("userenv", &["minwindef", "winnt", "winreg"], &["userenv"]),
    ("usp10", &["minwindef", "ntdef", "windef", "winerror", "wingdi", "winnt"], &["usp10"]),
    ("utilapiset", &["minwindef", "ntdef"], &["kernel32"]),
    ("uxtheme", &["commctrl", "minwindef", "windef", "wingdi", "winnt"], &["uxtheme"]),
    ("vsbackup", &["guiddef", "minwindef", "unknwnbase", "vss", "vswriter", "winnt", "wtypes"], &["vssapi"]),
    ("vss", &["guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("vsserror", &["winnt"], &[]),
    ("vswriter", &["minwindef", "unknwnbase", "vss", "winnt", "wtypes"], &[]),
    ("wbemads", &["oaidl", "wbemdisp", "winerror", "wtypes"], &[]),
    ("wbemcli", &["minwindef", "oaidl", "rpcndr", "unknwnbase", "winerror", "winnt", "wtypes"], &[]),
    ("wbemdisp", &["oaidl", "unknwnbase", "winerror", "wtypes"], &[]),
    ("wbemprov", &["minwindef", "oaidl", "unknwnbase", "wbemcli", "winerror", "winnt", "wtypes"], &[]),
    ("wbemtran", &["guiddef", "minwindef", "unknwnbase", "wbemcli", "winerror", "winnt", "wtypes"], &[]),
    ("wct", &["basetsd", "guiddef", "minwindef", "winnt"], &["advapi32"]),
    ("werapi", &["minwindef", "winnt"], &["kernel32", "wer"]),
    ("winbase", &["basetsd", "cfgmgr32", "fileapi", "guiddef", "libloaderapi", "minwinbase", "minwindef", "processthreadsapi", "vadefs", "windef", "winnt"], &["kernel32"]),
    ("wincodec", &["basetsd", "d2d1", "d2d1_1", "dcommon", "dxgiformat", "dxgitype", "guiddef", "minwindef", "ntdef", "objidlbase", "ocidl", "propidl", "unknwnbase", "windef", "winerror", "winnt"], &["windowscodecs"]),
    ("wincodecsdk", &["guiddef", "minwindef", "oaidl", "objidl", "objidlbase", "ocidl", "propidl", "unknwnbase", "wincodec", "winnt", "wtypes"], &["ole32", "oleaut32", "windowscodecs"]),
    ("wincon", &["minwinbase", "minwindef", "wincontypes", "windef", "wingdi", "winnt"], &["kernel32"]),
    ("wincontypes", &["minwindef", "winnt"], &[]),
    ("wincred", &["minwindef", "sspi", "windef", "winnt"], &["advapi32", "credui"]),
    ("wincrypt", &["basetsd", "bcrypt", "guiddef", "minwinbase", "minwindef", "ncrypt", "vcruntime", "winnt"], &["advapi32", "crypt32", "cryptnet"]),
    ("windowsceip", &["minwindef"], &["kernel32"]),
    ("winefs", &["basetsd", "minwinbase", "minwindef", "wincrypt", "winnt"], &["advapi32"]),
    ("winevt", &["basetsd", "guiddef", "minwinbase", "minwindef", "vcruntime", "winnt"], &["wevtapi"]),
    ("wingdi", &["basetsd", "minwindef", "windef", "winnt"], &["gdi32", "msimg32", "opengl32", "winspool"]),
    ("winhttp", &["basetsd", "minwinbase", "minwindef", "winnt"], &["winhttp"]),
    ("wininet", &["basetsd", "minwinbase", "minwindef", "ntdef", "windef", "winineti", "winnt"], &["wininet"]),
    ("winineti", &["minwindef"], &[]),
    ("winioctl", &["basetsd", "devpropdef", "guiddef", "minwindef", "winnt"], &[]),
    ("winnetwk", &["basetsd", "minwindef", "windef", "winerror", "winnt"], &["mpr"]),
    ("winnls", &["basetsd", "guiddef", "minwinbase", "minwindef", "winnt"], &["kernel32"]),
    ("winnt", &["basetsd", "excpt", "guiddef", "ktmtypes", "minwindef", "ntdef", "vcruntime"], &["kernel32"]),
    ("winreg", &["basetsd", "minwinbase", "minwindef", "reason", "winnt"], &["advapi32"]),
    ("winsafer", &["basetsd", "guiddef", "minwindef", "wincrypt", "windef", "winnt"], &["advapi32"]),
    ("winscard", &["basetsd", "guiddef", "minwindef", "rpcdce", "windef", "winnt", "winsmcrd"], &["winscard"]),
    ("winsmcrd", &["minwindef", "winioctl"], &[]),
    ("winsock2", &["basetsd", "guiddef", "inaddr", "minwinbase", "minwindef", "qos", "winbase", "windef", "winerror", "winnt", "ws2def", "wtypesbase"], &["ws2_32"]),
    ("winspool", &["guiddef", "minwinbase", "minwindef", "vcruntime", "windef", "winerror", "wingdi", "winnt"], &["winspool"]),
    ("winsvc", &["minwindef", "winnt"], &["advapi32"]),
    ("wintrust", &["guiddef", "minwindef", "ntdef", "wincrypt", "windef"], &["wintrust"]),
    ("winusb", &["minwinbase", "minwindef", "usb", "usbspec", "winnt", "winusbio"], &["winusb"]),
    ("winuser", &["basetsd", "guiddef", "limits", "minwinbase", "minwindef", "vadefs", "windef", "wingdi", "winnt"], &["user32"]),
    ("winver", &["minwindef", "winnt"], &["kernel32", "version"]),
    ("wlanapi", &["devpropdef", "eaptypes", "guiddef", "l2cmn", "minwindef", "windef", "windot11", "winnt", "wlantypes"], &["wlanapi"]),
    ("wlanihv", &["basetsd", "dot1x", "eaptypes", "guiddef", "l2cmn", "minwindef", "windot11", "winnt", "winuser", "wlanihvtypes", "wlantypes", "wlclient"], &[]),
    ("wlanihvtypes", &["eaptypes", "guiddef", "minwindef", "winnt", "wlantypes"], &[]),
    ("wlclient", &["guiddef", "minwindef", "windot11", "winnt"], &[]),
    ("wow64apiset", &["minwindef", "winnt"], &["kernel32"]),
    ("wpdmtpextensions", &["wtypes"], &[]),
    ("ws2bth", &["bthdef", "bthsdpdef", "guiddef", "minwindef", "winnt", "ws2def"], &[]),
    ("ws2spi", &["basetsd", "guiddef", "minwindef", "vcruntime", "windef", "winnt", "winsock2", "ws2def", "wtypesbase"], &["ws2_32"]),
    ("ws2tcpip", &["guiddef", "minwinbase", "minwindef", "mstcpip", "vcruntime", "winerror", "winnt", "winsock2", "ws2def", "wtypesbase"], &["fwpuclnt", "ws2_32"]),
    ("wtsapi32", &["minwindef", "ntdef"], &["wtsapi32"]),
    ("xinput", &["guiddef", "minwindef", "winnt"], &["xinput"]),
    // vc
    ("excpt", &[], &[]),
    ("limits", &[], &[]),
    ("vadefs", &[], &[]),
    ("vcruntime", &[], &[]),
    // winrt
    ("activation", &["inspectable", "winnt"], &[]),
    ("hstring", &["winnt"], &[]),
    ("inspectable", &["guiddef", "hstring", "minwindef", "unknwnbase", "winnt"], &[]),
    ("roapi", &["activation", "basetsd", "guiddef", "hstring", "inspectable", "objidl", "winnt"], &["runtimeobject"]),
    ("robuffer", &["objidl", "winnt"], &["runtimeobject"]),
    ("roerrorapi", &["basetsd", "hstring", "minwindef", "restrictederrorinfo", "unknwnbase", "winnt"], &["runtimeobject"]),
    ("winstring", &["basetsd", "hstring", "minwindef", "winnt"], &["runtimeobject"]),
];
struct Header {
    required: bool,
    included: Cell<bool>,
    dependencies: &'static [&'static str],
    libraries: &'static [&'static str],
}
struct Graph(HashMap<&'static str, Header>);
impl Graph {
    fn generate() -> Graph {
        Graph(DATA.iter().map(|&(name, dependencies, libraries)| {
            let header = Header {
                required: false,
                included: Cell::new(false),
                dependencies: dependencies,
                libraries: libraries,
            };
            (name, header)
        }).collect())
    }
    fn identify_required(&mut self) {
        for (name, header) in &mut self.0 {
            if let Ok(_) = var(&format!("CARGO_FEATURE_{}", name.to_uppercase())) {
                header.required = true;
                header.included.set(true);
            }
        }
    }
    fn check_everything(&self) {
        if let Ok(_) = var("CARGO_FEATURE_EVERYTHING") {
            for (_, header) in &self.0 {
                header.included.set(true);
            }
        }
    }
    fn resolve_dependencies(&self) {
        let mut done = false;
        while !done {
            done = true;
            for (_, header) in &self.0 {
                if header.included.get() {
                    for dep in header.dependencies {
                        let dep = &self.0.get(dep).expect(dep);
                        if !dep.included.get() {
                            done = false;
                            dep.included.set(true);
                        }
                    }
                }
            }
        }
    }
    fn emit_features(&self) {
        for (name, header) in &self.0 {
            if header.included.get() && !header.required {
                println!("cargo:rustc-cfg=feature=\"{}\"", name);
            }
        }
    }
    fn emit_libraries(&self) {
        let mut libs = self.0.iter().filter(|&(_, header)| {
            header.included.get()
        }).flat_map(|(_, header)| {
            header.libraries.iter()
        }).collect::<Vec<_>>();
        libs.sort();
        libs.dedup();
        // FIXME Temporary hacks until build script is redesigned.
        libs.retain(|&&lib| match &*var("TARGET").unwrap() {
            "aarch64-pc-windows-msvc" | "aarch64-uwp-windows-msvc" | "thumbv7a-pc-windows-msvc" => {
                if lib == "opengl32" { false }
                else { true }
            },
            _ => true,
        });
        let prefix = library_prefix();
        let kind = library_kind();
        for lib in libs {
            println!("cargo:rustc-link-lib={}={}{}", kind, prefix, lib);
        }
    }
}
fn library_prefix() -> &'static str {
    if var("TARGET").map(|target|
        target == "i686-pc-windows-gnu" || target == "x86_64-pc-windows-gnu"
    ).unwrap_or(false) && var("WINAPI_NO_BUNDLED_LIBRARIES").is_err() {
        "winapi_"
    } else {
        ""
    }
}
fn library_kind() -> &'static str {
    if var("WINAPI_STATIC_NOBUNDLE").is_ok() {
        "static-nobundle"
    } else {
        "dylib"
    }
}
fn try_everything() {
    let mut graph = Graph::generate();
    graph.identify_required();
    graph.check_everything();
    graph.resolve_dependencies();
    graph.emit_features();
    graph.emit_libraries();
}
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=WINAPI_NO_BUNDLED_LIBRARIES");
    println!("cargo:rerun-if-env-changed=WINAPI_STATIC_NOBUNDLE");
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        try_everything();
    }
}
