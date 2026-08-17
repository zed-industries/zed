// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Component;

use windows::Win32::Foundation::{SysAllocString, SysFreeString};
use windows::Win32::Security::PSECURITY_DESCRIPTOR;
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, CoCreateInstance, CoInitializeEx, CoInitializeSecurity,
    CoSetProxyBlanket, EOAC_NONE, RPC_C_AUTHN_LEVEL_CALL, RPC_C_AUTHN_LEVEL_DEFAULT,
    RPC_C_IMP_LEVEL_IMPERSONATE,
};
use windows::Win32::System::Rpc::{RPC_C_AUTHN_WINNT, RPC_C_AUTHZ_NONE};
use windows::Win32::System::Variant::{VARIANT, VariantClear};
use windows::Win32::System::Wmi::{
    IEnumWbemClassObject, IWbemLocator, IWbemServices, WBEM_FLAG_FORWARD_ONLY,
    WBEM_FLAG_NONSYSTEM_ONLY, WBEM_FLAG_RETURN_IMMEDIATELY, WBEM_INFINITE, WbemLocator,
};
use windows::core::w;

use std::cell::OnceCell;
use std::sync::OnceLock;

pub(crate) struct ComponentInner {
    temperature: f32,
    max: f32,
    critical: Option<f32>,
    label: String,
    connection: Option<Connection>,
    pub(crate) updated: bool,
}

impl ComponentInner {
    /// Creates a new `ComponentInner` with the given information.
    fn new() -> Option<Self> {
        let mut c = Connection::new()
            .and_then(|x| x.create_instance())
            .and_then(|x| x.connect_server())
            .and_then(|x| x.set_proxy_blanket())
            .and_then(|x| x.exec_query())?;

        c.temperature(true)
            .map(|(temperature, critical)| ComponentInner {
                temperature,
                label: "Computer".to_owned(),
                max: temperature,
                critical,
                connection: Some(c),
                updated: true,
            })
    }

    pub(crate) fn temperature(&self) -> Option<f32> {
        Some(self.temperature)
    }

    pub(crate) fn max(&self) -> Option<f32> {
        Some(self.max)
    }

    pub(crate) fn critical(&self) -> Option<f32> {
        self.critical
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn id(&self) -> Option<&str> {
        Some(&self.label)
    }

    pub(crate) fn refresh(&mut self) {
        if self.connection.is_none() {
            self.connection = Connection::new()
                .and_then(|x| x.create_instance())
                .and_then(|x| x.connect_server())
                .and_then(|x| x.set_proxy_blanket());
        }
        self.connection = if let Some(x) = self.connection.take() {
            x.exec_query()
        } else {
            None
        };
        if let Some(ref mut connection) = self.connection
            && let Some((temperature, _)) = connection.temperature(false)
        {
            self.temperature = temperature;
            if self.temperature > self.max {
                self.max = self.temperature;
            }
        }
    }
}

pub(crate) struct ComponentsInner {
    pub(crate) components: Vec<Component>,
}

impl ComponentsInner {
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub(crate) fn from_vec(components: Vec<Component>) -> Self {
        Self { components }
    }

    pub(crate) fn into_vec(self) -> Vec<Component> {
        self.components
    }

    pub(crate) fn list(&self) -> &[Component] {
        &self.components
    }

    pub(crate) fn list_mut(&mut self) -> &mut [Component] {
        &mut self.components
    }

    pub(crate) fn refresh(&mut self) {
        if self.components.is_empty() {
            self.components = match ComponentInner::new() {
                Some(c) => vec![Component { inner: c }],
                None => Vec::new(),
            };
        } else {
            // There should always be only one here but just in case...
            for c in self.components.iter_mut() {
                c.refresh();
                c.inner.updated = true;
            }
        }
    }
}

macro_rules! bstr {
    ($x:literal) => {{ SysAllocString(w!($x)) }};
}

struct Connection {
    instance: Option<IWbemLocator>,
    server_connection: Option<IWbemServices>,
    enumerator: Option<IEnumWbemClassObject>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

static SECURITY: OnceLock<Result<(), ()>> = OnceLock::new();
thread_local! {
    pub static CONNECTION: OnceCell<Result<(), ()>> = const { OnceCell::new() };
}

unsafe fn initialize_connection() -> Result<(), ()> {
    if unsafe { CoInitializeEx(None, Default::default()) }.is_err() {
        sysinfo_debug!("Failed to initialize connection");
        Err(())
    } else {
        Ok(())
    }
}

unsafe fn initialize_security() -> Result<(), ()> {
    if unsafe {
        CoInitializeSecurity(
            Some(PSECURITY_DESCRIPTOR::default()),
            -1,
            None,
            None,
            RPC_C_AUTHN_LEVEL_DEFAULT,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            None,
            EOAC_NONE,
            None,
        )
    }
    .is_err()
    {
        sysinfo_debug!("Failed to initialize security");
        Err(())
    } else {
        Ok(())
    }
}

impl Connection {
    #[allow(clippy::unnecessary_wraps)]
    fn new() -> Option<Connection> {
        if CONNECTION
            .with(|x| *x.get_or_init(|| unsafe { initialize_connection() }))
            .is_err()
            || SECURITY
                .get_or_init(|| unsafe { initialize_security() })
                .is_err()
        {
            return None;
        }
        Some(Connection {
            instance: None,
            server_connection: None,
            enumerator: None,
        })
    }

    fn create_instance(mut self) -> Option<Connection> {
        let instance =
            unsafe { CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER) }.ok()?;
        self.instance = Some(instance);
        Some(self)
    }

    fn connect_server(mut self) -> Option<Connection> {
        let instance = self.instance.as_ref()?;
        let svc = unsafe {
            let s = bstr!("root\\WMI");
            let res = instance.ConnectServer(
                &s,
                &Default::default(),
                &Default::default(),
                &Default::default(),
                0,
                &Default::default(),
                None,
            );
            SysFreeString(&s);
            res
        }
        .ok()?;

        self.server_connection = Some(svc);
        Some(self)
    }

    fn set_proxy_blanket(self) -> Option<Connection> {
        unsafe {
            CoSetProxyBlanket(
                self.server_connection.as_ref()?,
                RPC_C_AUTHN_WINNT,
                RPC_C_AUTHZ_NONE,
                None,
                RPC_C_AUTHN_LEVEL_CALL,
                RPC_C_IMP_LEVEL_IMPERSONATE,
                None,
                EOAC_NONE,
            )
        }
        .ok()?;

        Some(self)
    }

    fn exec_query(mut self) -> Option<Connection> {
        let server_connection = self.server_connection.as_ref()?;

        let enumerator = unsafe {
            let s = bstr!("WQL"); // query kind
            let query = bstr!("SELECT * FROM MSAcpi_ThermalZoneTemperature");
            let hres = server_connection.ExecQuery(
                &s,
                &query,
                WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY,
                None,
            );
            SysFreeString(&s);
            SysFreeString(&query);
            hres
        }
        .ok()?;

        self.enumerator = Some(enumerator);
        Some(self)
    }

    fn temperature(&mut self, get_critical: bool) -> Option<(f32, Option<f32>)> {
        let enumerator = self.enumerator.take()?;

        let mut nb_returned = 0;
        let mut obj = [None; 1];

        unsafe {
            let _r = enumerator.Next(
                WBEM_INFINITE, // Time out
                obj.as_mut_slice(),
                &mut nb_returned,
            );

            if nb_returned == 0 {
                return None; // not enough rights I suppose...
            }

            let class_obj = match &mut obj {
                [Some(co)] => co,
                _ => return None,
            };

            let _r = class_obj.BeginEnumeration(WBEM_FLAG_NONSYSTEM_ONLY.0);

            let mut variant = std::mem::MaybeUninit::<VARIANT>::uninit();
            // `Get` only initializes the variant if it succeeds, early returning is not a problem
            //
            // <https://learn.microsoft.com/en-us/windows/win32/api/wbemcli/nf-wbemcli-iwbemclassobject-get>
            class_obj
                .Get(
                    w!("CurrentTemperature"),
                    0,
                    variant.as_mut_ptr(),
                    None,
                    None,
                )
                .ok()?;

            let mut variant = variant.assume_init();

            // temperature is given in tenth of degrees Kelvin
            let temp = (variant.Anonymous.decVal.Anonymous2.Lo64 / 10) as f32 - 273.15;
            let _r = VariantClear(&mut variant);

            let mut critical = None;
            if get_critical {
                class_obj
                    .Get(w!("CriticalTripPoint"), 0, &mut variant, None, None)
                    .ok()?;

                // temperature is given in tenth of degrees Kelvin
                critical = Some((variant.Anonymous.decVal.Anonymous2.Lo64 / 10) as f32 - 273.15);
                let _r = VariantClear(&mut variant);
            }

            Some((temp, critical))
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // Those three calls are here to enforce that they get dropped in the good order.
        self.enumerator.take();
        self.server_connection.take();
        self.instance.take();
    }
}
