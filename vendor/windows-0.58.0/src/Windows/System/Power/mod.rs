#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IBackgroundEnergyManagerStatics, IBackgroundEnergyManagerStatics_Vtbl, 0xb3161d95_1180_4376_96e1_4095568147ce);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IBackgroundEnergyManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundEnergyManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub LowUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LowUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearMaxAcceptableUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearMaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub MaxAcceptableUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub ExcessiveUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExcessiveUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearTerminationUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearTerminationUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub TerminationUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TerminationUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsage: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageIncreased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageIncreased: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRecentEnergyUsageIncreased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRecentEnergyUsageIncreased: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageReturnedToLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageReturnedToLow: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRecentEnergyUsageReturnedToLow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRecentEnergyUsageReturnedToLow: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IForegroundEnergyManagerStatics, IForegroundEnergyManagerStatics_Vtbl, 0x9ff86872_e677_4814_9a20_5337ca732b98);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IForegroundEnergyManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IForegroundEnergyManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub LowUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LowUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearMaxAcceptableUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearMaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub MaxAcceptableUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub ExcessiveUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExcessiveUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsage: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageIncreased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageIncreased: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRecentEnergyUsageIncreased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRecentEnergyUsageIncreased: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageReturnedToLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageReturnedToLow: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRecentEnergyUsageReturnedToLow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRecentEnergyUsageReturnedToLow: usize,
}
windows_core::imp::define_interface!(IPowerManagerStatics, IPowerManagerStatics_Vtbl, 0x1394825d_62ce_4364_98d5_aa28c7fbd15b);
impl windows_core::RuntimeType for IPowerManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPowerManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnergySaverStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EnergySaverStatus) -> windows_core::HRESULT,
    pub EnergySaverStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnergySaverStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BatteryStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BatteryStatus) -> windows_core::HRESULT,
    pub BatteryStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBatteryStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PowerSupplyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PowerSupplyStatus) -> windows_core::HRESULT,
    pub PowerSupplyStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePowerSupplyStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemainingChargePercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingChargePercentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemainingDischargeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub RemainingDischargeTimeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemainingDischargeTimeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
pub struct BackgroundEnergyManager;
#[cfg(feature = "deprecated")]
impl BackgroundEnergyManager {
    #[cfg(feature = "deprecated")]
    pub fn LowUsageLevel() -> windows_core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LowUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn NearMaxAcceptableUsageLevel() -> windows_core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NearMaxAcceptableUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn MaxAcceptableUsageLevel() -> windows_core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAcceptableUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ExcessiveUsageLevel() -> windows_core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExcessiveUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn NearTerminationUsageLevel() -> windows_core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NearTerminationUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn TerminationUsageLevel() -> windows_core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TerminationUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsage() -> windows_core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentEnergyUsage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageLevel() -> windows_core::Result<u32> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentEnergyUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageIncreased<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentEnergyUsageIncreased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRecentEnergyUsageIncreased(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRecentEnergyUsageIncreased)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageReturnedToLow<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentEnergyUsageReturnedToLow)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRecentEnergyUsageReturnedToLow(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IBackgroundEnergyManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRecentEnergyUsageReturnedToLow)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundEnergyManagerStatics<R, F: FnOnce(&IBackgroundEnergyManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundEnergyManager, IBackgroundEnergyManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for BackgroundEnergyManager {
    const NAME: &'static str = "Windows.System.Power.BackgroundEnergyManager";
}
#[cfg(feature = "deprecated")]
pub struct ForegroundEnergyManager;
#[cfg(feature = "deprecated")]
impl ForegroundEnergyManager {
    #[cfg(feature = "deprecated")]
    pub fn LowUsageLevel() -> windows_core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LowUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn NearMaxAcceptableUsageLevel() -> windows_core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NearMaxAcceptableUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn MaxAcceptableUsageLevel() -> windows_core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAcceptableUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ExcessiveUsageLevel() -> windows_core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExcessiveUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsage() -> windows_core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentEnergyUsage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageLevel() -> windows_core::Result<u32> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentEnergyUsageLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageIncreased<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentEnergyUsageIncreased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRecentEnergyUsageIncreased(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRecentEnergyUsageIncreased)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecentEnergyUsageReturnedToLow<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IForegroundEnergyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentEnergyUsageReturnedToLow)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRecentEnergyUsageReturnedToLow(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IForegroundEnergyManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRecentEnergyUsageReturnedToLow)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IForegroundEnergyManagerStatics<R, F: FnOnce(&IForegroundEnergyManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ForegroundEnergyManager, IForegroundEnergyManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for ForegroundEnergyManager {
    const NAME: &'static str = "Windows.System.Power.ForegroundEnergyManager";
}
pub struct PowerManager;
impl PowerManager {
    pub fn EnergySaverStatus() -> windows_core::Result<EnergySaverStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnergySaverStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn EnergySaverStatusChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnergySaverStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveEnergySaverStatusChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveEnergySaverStatusChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn BatteryStatus() -> windows_core::Result<BatteryStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatteryStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BatteryStatusChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatteryStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveBatteryStatusChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveBatteryStatusChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn PowerSupplyStatus() -> windows_core::Result<PowerSupplyStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerSupplyStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PowerSupplyStatusChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerSupplyStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemovePowerSupplyStatusChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemovePowerSupplyStatusChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn RemainingChargePercent() -> windows_core::Result<i32> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingChargePercent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RemainingChargePercentChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingChargePercentChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveRemainingChargePercentChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRemainingChargePercentChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn RemainingDischargeTime() -> windows_core::Result<super::super::Foundation::TimeSpan> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingDischargeTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RemainingDischargeTimeChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingDischargeTimeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveRemainingDischargeTimeChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRemainingDischargeTimeChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics<R, F: FnOnce(&IPowerManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PowerManager, IPowerManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PowerManager {
    const NAME: &'static str = "Windows.System.Power.PowerManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Discharging: Self = Self(1i32);
    pub const Idle: Self = Self(2i32);
    pub const Charging: Self = Self(3i32);
}
impl windows_core::TypeKind for BatteryStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BatteryStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BatteryStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BatteryStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Power.BatteryStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Disabled: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl windows_core::TypeKind for EnergySaverStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnergySaverStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnergySaverStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for EnergySaverStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Power.EnergySaverStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Inadequate: Self = Self(1i32);
    pub const Adequate: Self = Self(2i32);
}
impl windows_core::TypeKind for PowerSupplyStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PowerSupplyStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PowerSupplyStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PowerSupplyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Power.PowerSupplyStatus;i4)");
}
