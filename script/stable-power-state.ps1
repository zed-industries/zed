<#
.SYNOPSIS
    Pins a GPU to its stable (base) clock for the duration of the script, via
    ID3D12Device::SetStablePowerState.

.DESCRIPTION
    Creates a D3D12 device on the chosen adapter, enables the stable power
    state, and then blocks until you press Enter. Run your benchmark while it
    is blocked; press Enter to restore normal clock/power behavior and exit.

    The stable power state lives only as long as the D3D12 device that set it,
    so the GPU also reverts if this script is killed or crashes.

    Requires Windows Developer Mode. Without it, SetStablePowerState removes
    the device and fails (harmlessly - only this script's own device dies).

    Note that "stable" means base clocks, which are LOWER than boost clocks.
    Absolute numbers will look worse than normal; the point is that run-to-run
    variance collapses, which is what makes A/B comparisons meaningful.

.PARAMETER AdapterIndex
    DXGI adapter to pin. Defaults to 0. Use -ListAdapters to see the options.

.PARAMETER ListAdapters
    Print the available adapters and exit without changing anything.

.EXAMPLE
    ./script/stable-power-state.ps1 -ListAdapters

.EXAMPLE
    ./script/stable-power-state.ps1
#>
[CmdletBinding()]
param(
    [int]$AdapterIndex = 0,
    [switch]$ListAdapters
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not ('StablePower' -as [type])) {
    Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

public sealed class AdapterInfo
{
    public int Index;
    public string Description;
    public double DedicatedVideoMemoryGB;
    public bool IsSoftware;
}

public static class StablePower
{
    [DllImport("dxgi.dll")]
    private static extern int CreateDXGIFactory1(ref Guid riid, out IntPtr factory);

    [DllImport("d3d12.dll")]
    private static extern int D3D12CreateDevice(
        IntPtr adapter, uint minimumFeatureLevel, ref Guid riid, out IntPtr device);

    private static Guid IID_IDXGIFactory1 = new Guid("770aae78-f26f-4dba-a829-253c83d1b387");
    private static Guid IID_ID3D12Device = new Guid("189819f1-1db6-4b57-be54-1821339b85f7");

    private const uint D3D_FEATURE_LEVEL_11_0 = 0xb000;
    private const uint DXGI_ADAPTER_FLAG_SOFTWARE = 2;

    // Vtable slots are counted from the top of the inheritance chain. Calling
    // through raw slots rather than declaring [ComImport] interfaces avoids
    // having to redeclare the ~40 preceding methods, several of which return
    // structs by value and are easy to get subtly wrong.
    //
    // IDXGIFactory1: IUnknown 0-2, IDXGIObject 3-6, IDXGIFactory 7-11.
    private const int VT_ENUM_ADAPTERS1 = 12;
    // IDXGIAdapter1: IUnknown 0-2, IDXGIObject 3-6, IDXGIAdapter 7-9.
    private const int VT_GET_DESC1 = 10;
    // ID3D12Device: IUnknown 0-2, ID3D12Object 3-6, then GetNodeCount at 7
    // through CreateQueryHeap at 39.
    private const int VT_SET_STABLE_POWER_STATE = 40;

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    private struct DXGI_ADAPTER_DESC1
    {
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 128)]
        public string Description;
        public uint VendorId;
        public uint DeviceId;
        public uint SubSysId;
        public uint Revision;
        public UIntPtr DedicatedVideoMemory;
        public UIntPtr DedicatedSystemMemory;
        public UIntPtr SharedSystemMemory;
        public long AdapterLuid;
        public uint Flags;
    }

    [UnmanagedFunctionPointer(CallingConvention.StdCall)]
    private delegate int EnumAdapters1Fn(IntPtr self, uint index, out IntPtr adapter);

    [UnmanagedFunctionPointer(CallingConvention.StdCall)]
    private delegate int GetDesc1Fn(IntPtr self, out DXGI_ADAPTER_DESC1 desc);

    [UnmanagedFunctionPointer(CallingConvention.StdCall)]
    private delegate int SetStablePowerStateFn(IntPtr self, int enable);

    private static T Slot<T>(IntPtr instance, int slot) where T : class
    {
        IntPtr vtable = Marshal.ReadIntPtr(instance);
        IntPtr function = Marshal.ReadIntPtr(vtable, slot * IntPtr.Size);
        return (T)(object)Marshal.GetDelegateForFunctionPointer(function, typeof(T));
    }

    private static void Check(int hr, string what)
    {
        if (hr < 0)
        {
            throw new InvalidOperationException(
                what + " failed: 0x" + hr.ToString("X8") + " (" + Explain(hr) + ")");
        }
    }

    private static string Explain(int hr)
    {
        switch ((uint)hr)
        {
            case 0x887A0004: return "DXGI_ERROR_UNSUPPORTED";
            case 0x887A0005: return "DXGI_ERROR_DEVICE_REMOVED - Developer Mode is most likely off";
            case 0x887A0002: return "DXGI_ERROR_NOT_FOUND";
            case 0x80070057: return "E_INVALIDARG";
            case 0x80004005: return "E_FAIL - Developer Mode is most likely off";
            default: return "unknown HRESULT";
        }
    }

    public static AdapterInfo[] ListAdapters()
    {
        IntPtr factory;
        Check(CreateDXGIFactory1(ref IID_IDXGIFactory1, out factory), "CreateDXGIFactory1");
        var adapters = new List<AdapterInfo>();
        try
        {
            var enumAdapters = Slot<EnumAdapters1Fn>(factory, VT_ENUM_ADAPTERS1);
            for (uint index = 0; ; index++)
            {
                IntPtr adapter;
                if (enumAdapters(factory, index, out adapter) < 0)
                {
                    break;
                }
                try
                {
                    DXGI_ADAPTER_DESC1 desc;
                    Check(Slot<GetDesc1Fn>(adapter, VT_GET_DESC1)(adapter, out desc), "GetDesc1");
                    adapters.Add(new AdapterInfo
                    {
                        Index = (int)index,
                        Description = desc.Description,
                        DedicatedVideoMemoryGB =
                            desc.DedicatedVideoMemory.ToUInt64() / (1024.0 * 1024.0 * 1024.0),
                        IsSoftware = (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE) != 0
                    });
                }
                finally
                {
                    Marshal.Release(adapter);
                }
            }
        }
        finally
        {
            Marshal.Release(factory);
        }
        return adapters.ToArray();
    }

    public static IntPtr CreateDevice(int adapterIndex)
    {
        IntPtr factory;
        Check(CreateDXGIFactory1(ref IID_IDXGIFactory1, out factory), "CreateDXGIFactory1");
        IntPtr adapter = IntPtr.Zero;
        try
        {
            Check(Slot<EnumAdapters1Fn>(factory, VT_ENUM_ADAPTERS1)(factory, (uint)adapterIndex, out adapter),
                "EnumAdapters1(" + adapterIndex + ")");
            IntPtr device;
            Check(D3D12CreateDevice(adapter, D3D_FEATURE_LEVEL_11_0, ref IID_ID3D12Device, out device),
                "D3D12CreateDevice");
            return device;
        }
        finally
        {
            if (adapter != IntPtr.Zero)
            {
                Marshal.Release(adapter);
            }
            Marshal.Release(factory);
        }
    }

    public static void SetStable(IntPtr device, bool enable)
    {
        var setStable = Slot<SetStablePowerStateFn>(device, VT_SET_STABLE_POWER_STATE);
        Check(setStable(device, enable ? 1 : 0), "SetStablePowerState(" + enable + ")");
    }

    public static void Release(IntPtr instance)
    {
        if (instance != IntPtr.Zero)
        {
            Marshal.Release(instance);
        }
    }
}
'@
}

function Test-DeveloperMode {
    try {
        $value = Get-ItemPropertyValue `
            -Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock' `
            -Name 'AllowDevelopmentWithoutDevLicense' `
            -ErrorAction Stop
        return $value -eq 1
    } catch {
        return $false
    }
}

$adapters = [StablePower]::ListAdapters()
if ($adapters.Count -eq 0) {
    throw 'No DXGI adapters found.'
}

if ($ListAdapters) {
    $adapters | Format-Table `
        Index,
        Description,
        @{ Name = 'VRAM (GB)'; Expression = { '{0:N1}' -f $_.DedicatedVideoMemoryGB } },
        IsSoftware
    return
}

$selected = $adapters | Where-Object { $_.Index -eq $AdapterIndex }
if (-not $selected) {
    throw "No adapter at index $AdapterIndex. Run with -ListAdapters to see the options."
}

if (-not (Test-DeveloperMode)) {
    Write-Warning @'
Windows Developer Mode appears to be OFF. SetStablePowerState requires it and
will fail (Settings > System > For developers > Developer Mode). Attempting
anyway in case the registry probe is wrong.
'@
}

$device = [StablePower]::CreateDevice($AdapterIndex)
try {
    [StablePower]::SetStable($device, $true)

    Write-Host ''
    Write-Host "Stable power state ENABLED on adapter $AdapterIndex : $($selected.Description)" -ForegroundColor Green
    Write-Host 'The GPU is pinned to base clocks: lower peak throughput, much lower variance.'
    Write-Host ''
    Write-Host 'Run your benchmark now, then press Enter to restore normal clocks.' -ForegroundColor Cyan
    [void](Read-Host)
}
finally {
    try {
        [StablePower]::SetStable($device, $false)
        Write-Host 'Stable power state DISABLED.' -ForegroundColor Yellow
    } catch {
        Write-Warning "Could not disable explicitly ($($_.Exception.Message)); releasing the device restores normal clocks anyway."
    }
    [StablePower]::Release($device)
}
