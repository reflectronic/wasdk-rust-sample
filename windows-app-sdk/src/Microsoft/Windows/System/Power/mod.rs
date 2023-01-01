#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPowerManagerStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa3554cc_be1c_534c_bff8_72df78e9f4a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EnergySaverStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut EnergySaverStatus,
    ) -> ::windows::core::HRESULT,
    pub EnergySaverStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveEnergySaverStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub BatteryStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BatteryStatus,
    ) -> ::windows::core::HRESULT,
    pub BatteryStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveBatteryStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PowerSupplyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PowerSupplyStatus,
    ) -> ::windows::core::HRESULT,
    pub PowerSupplyStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePowerSupplyStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemainingChargePercent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub RemainingChargePercentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub RemainingDischargeTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows::core::HRESULT,
    pub RemainingDischargeTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveRemainingDischargeTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub PowerSourceKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PowerSourceKind,
    ) -> ::windows::core::HRESULT,
    pub PowerSourceKindChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePowerSourceKindChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub DisplayStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DisplayStatus,
    ) -> ::windows::core::HRESULT,
    pub DisplayStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDisplayStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub SystemIdleStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveSystemIdleStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub EffectivePowerMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EffectivePowerModeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveEffectivePowerModeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub UserPresenceStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut UserPresenceStatus,
    ) -> ::windows::core::HRESULT,
    pub UserPresenceStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveUserPresenceStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub SystemSuspendStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemSuspendStatus,
    ) -> ::windows::core::HRESULT,
    pub SystemSuspendStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveSystemSuspendStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPowerManagerStatics2 {
    type Vtable = IPowerManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPowerManagerStatics2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x61f3cc25_65b4_5def_9c9b_990cef3b0833);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EffectivePowerMode2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut EffectivePowerMode,
    ) -> ::windows::core::HRESULT,
}
pub struct PowerManager;
impl PowerManager {
    pub fn EnergySaverStatus() -> ::windows::core::Result<EnergySaverStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnergySaverStatus)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<EnergySaverStatus>(result__)
        })
    }
    pub fn EnergySaverStatusChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnergySaverStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveEnergySaverStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveEnergySaverStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn BatteryStatus() -> ::windows::core::Result<BatteryStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BatteryStatus)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<BatteryStatus>(result__)
        })
    }
    pub fn BatteryStatusChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BatteryStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveBatteryStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveBatteryStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn PowerSupplyStatus() -> ::windows::core::Result<PowerSupplyStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerSupplyStatus)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PowerSupplyStatus>(result__)
        })
    }
    pub fn PowerSupplyStatusChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerSupplyStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemovePowerSupplyStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemovePowerSupplyStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn RemainingChargePercent() -> ::windows::core::Result<i32> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingChargePercent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn RemainingChargePercentChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingChargePercentChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRemainingChargePercentChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveRemainingChargePercentChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn RemainingDischargeTime() -> ::windows::core::Result<::windows::Foundation::TimeSpan> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingDischargeTime)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::TimeSpan>(result__)
        })
    }
    pub fn RemainingDischargeTimeChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingDischargeTimeChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveRemainingDischargeTimeChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveRemainingDischargeTimeChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn PowerSourceKind() -> ::windows::core::Result<PowerSourceKind> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerSourceKind)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PowerSourceKind>(result__)
        })
    }
    pub fn PowerSourceKindChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PowerSourceKindChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemovePowerSourceKindChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemovePowerSourceKindChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn DisplayStatus() -> ::windows::core::Result<DisplayStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayStatus)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayStatus>(result__)
        })
    }
    pub fn DisplayStatusChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveDisplayStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDisplayStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn SystemIdleStatusChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemIdleStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveSystemIdleStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveSystemIdleStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn EffectivePowerMode(
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<EffectivePowerMode>> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EffectivePowerMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<EffectivePowerMode>>(result__)
        })
    }
    pub fn EffectivePowerModeChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EffectivePowerModeChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveEffectivePowerModeChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveEffectivePowerModeChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn UserPresenceStatus() -> ::windows::core::Result<UserPresenceStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserPresenceStatus)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<UserPresenceStatus>(result__)
        })
    }
    pub fn UserPresenceStatusChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserPresenceStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveUserPresenceStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveUserPresenceStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn SystemSuspendStatus() -> ::windows::core::Result<SystemSuspendStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemSuspendStatus)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SystemSuspendStatus>(result__)
        })
    }
    pub fn SystemSuspendStatusChanged(
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemSuspendStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveSystemSuspendStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RemoveSystemSuspendStatusChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn EffectivePowerMode2() -> ::windows::core::Result<EffectivePowerMode> {
        Self::IPowerManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EffectivePowerMode2)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<EffectivePowerMode>(result__)
        })
    }
    pub fn IPowerManagerStatics<
        R,
        F: FnOnce(&IPowerManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PowerManager, IPowerManagerStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IPowerManagerStatics2<
        R,
        F: FnOnce(&IPowerManagerStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PowerManager, IPowerManagerStatics2> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PowerManager {
    const NAME: &'static str = "Microsoft.Windows.System.Power.PowerManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Discharging: Self = Self(1i32);
    pub const Idle: Self = Self(2i32);
    pub const Charging: Self = Self(3i32);
}
impl ::core::marker::Copy for BatteryStatus {}
impl ::core::clone::Clone for BatteryStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BatteryStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BatteryStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BatteryStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BatteryStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BatteryStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.BatteryStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayStatus(pub i32);
impl DisplayStatus {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Dimmed: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayStatus {}
impl ::core::clone::Clone for DisplayStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.DisplayStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EffectivePowerMode(pub i32);
impl EffectivePowerMode {
    pub const BatterySaver: Self = Self(0i32);
    pub const BetterBattery: Self = Self(1i32);
    pub const Balanced: Self = Self(2i32);
    pub const HighPerformance: Self = Self(3i32);
    pub const MaxPerformance: Self = Self(4i32);
    pub const GameMode: Self = Self(5i32);
    pub const MixedReality: Self = Self(6i32);
}
impl ::core::marker::Copy for EffectivePowerMode {}
impl ::core::clone::Clone for EffectivePowerMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EffectivePowerMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EffectivePowerMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for EffectivePowerMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EffectivePowerMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EffectivePowerMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.EffectivePowerMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Uninitialized: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const On: Self = Self(3i32);
}
impl ::core::marker::Copy for EnergySaverStatus {}
impl ::core::clone::Clone for EnergySaverStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnergySaverStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EnergySaverStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnergySaverStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnergySaverStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnergySaverStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.EnergySaverStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PowerSourceKind(pub i32);
impl PowerSourceKind {
    pub const AC: Self = Self(0i32);
    pub const DC: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerSourceKind {}
impl ::core::clone::Clone for PowerSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerSourceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PowerSourceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerSourceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSourceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerSourceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.PowerSourceKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Inadequate: Self = Self(1i32);
    pub const Adequate: Self = Self(2i32);
}
impl ::core::marker::Copy for PowerSupplyStatus {}
impl ::core::clone::Clone for PowerSupplyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerSupplyStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PowerSupplyStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerSupplyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSupplyStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerSupplyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.PowerSupplyStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemSuspendStatus(pub i32);
impl SystemSuspendStatus {
    pub const Uninitialized: Self = Self(0i32);
    pub const Entering: Self = Self(1i32);
    pub const AutoResume: Self = Self(2i32);
    pub const ManualResume: Self = Self(3i32);
}
impl ::core::marker::Copy for SystemSuspendStatus {}
impl ::core::clone::Clone for SystemSuspendStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemSuspendStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemSuspendStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemSuspendStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemSuspendStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemSuspendStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.SystemSuspendStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserPresenceStatus(pub i32);
impl UserPresenceStatus {
    pub const Present: Self = Self(0i32);
    pub const Absent: Self = Self(1i32);
}
impl ::core::marker::Copy for UserPresenceStatus {}
impl ::core::clone::Clone for UserPresenceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserPresenceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserPresenceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserPresenceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPresenceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserPresenceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.UserPresenceStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
