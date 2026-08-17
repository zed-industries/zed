use std::ffi::CString;

use crate::protocol::Message;
use crate::types::client::InvalidId;

use super::*;

struct ServerData(());

macro_rules! impl_server_objectdata {
    ($server_backend:tt) => {
        impl $server_backend::ObjectData<()> for ServerData {
            fn request(
                self: Arc<Self>,
                _handle: &$server_backend::Handle,
                _: &mut (),
                _: $server_backend::ClientId,
                _msg: Message<$server_backend::ObjectId, OwnedFd>,
            ) -> Option<Arc<dyn $server_backend::ObjectData<()>>> {
                Some(self)
            }

            fn destroyed(
                self: Arc<Self>,
                _: &$server_backend::Handle,
                _: &mut (),
                _: $server_backend::ClientId,
                _: $server_backend::ObjectId,
            ) {
            }
        }

        impl $server_backend::GlobalHandler<()> for ServerData {
            fn bind(
                self: Arc<Self>,
                _: &$server_backend::Handle,
                _: &mut (),
                _: $server_backend::ClientId,
                _: $server_backend::GlobalId,
                _: $server_backend::ObjectId,
            ) -> Arc<dyn $server_backend::ObjectData<()>> {
                self
            }
        }
    };
}

impl_server_objectdata!(server_rs);
impl_server_objectdata!(server_sys);

struct ClientData(());

macro_rules! impl_client_objectdata {
    ($client_backend:tt) => {
        impl $client_backend::ObjectData for ClientData {
            fn event(
                self: Arc<Self>,
                _handle: &$client_backend::Backend,
                _msg: Message<$client_backend::ObjectId, OwnedFd>,
            ) -> Option<Arc<dyn $client_backend::ObjectData>> {
                None
            }
            fn destroyed(&self, _object_id: $client_backend::ObjectId) {}
        }
    };
}

impl_client_objectdata!(client_rs);
impl_client_objectdata!(client_sys);

// create a global and destroy it
expand_test!(destroy_global, {
    let (tx, rx) = std::os::unix::net::UnixStream::pair().unwrap();
    let mut server = server_backend::Backend::new().unwrap();
    let _client_id = server.handle().insert_client(rx, Arc::new(())).unwrap();
    let client = client_backend::Backend::connect(tx).unwrap();

    let server_data = Arc::new(ServerData(()));
    let client_data = Arc::new(ClientData(()));

    // Prepare a global
    server.handle().create_global(&interfaces::TEST_GLOBAL_INTERFACE, 3, server_data.clone());

    // get the registry client-side
    let client_display = client.display_id();
    let registry_id = client
        .send_request(
            message!(client_display, 1, [Argument::NewId(client_backend::ObjectId::null())],),
            Some(Arc::new(DoNothingData)),
            Some((&interfaces::WL_REGISTRY_INTERFACE, 1)),
        )
        .unwrap();
    // create the test global
    let test_global_id = client
        .send_request(
            message!(
                registry_id,
                0,
                [
                    Argument::Uint(1),
                    Argument::Str(Some(Box::new(
                        CString::new(interfaces::TEST_GLOBAL_INTERFACE.name.as_bytes()).unwrap(),
                    ))),
                    Argument::Uint(3),
                    Argument::NewId(client_backend::ObjectId::null()),
                ],
            ),
            Some(client_data.clone()),
            Some((&interfaces::TEST_GLOBAL_INTERFACE, 3)),
        )
        .unwrap();

    client.flush().unwrap();
    server.dispatch_all_clients(&mut ()).unwrap();
    server.flush(None).unwrap();
    client.prepare_read().unwrap().read().unwrap();
    client.destroy_object(&test_global_id).unwrap();
});

expand_test!(destroy_twice, {
    let (tx, rx) = std::os::unix::net::UnixStream::pair().unwrap();
    let mut server = server_backend::Backend::new().unwrap();
    let _client_id = server.handle().insert_client(rx, Arc::new(())).unwrap();
    let client = client_backend::Backend::connect(tx).unwrap();

    let server_data = Arc::new(ServerData(()));
    let client_data = Arc::new(ClientData(()));

    // Prepare a global
    server.handle().create_global(&interfaces::TEST_GLOBAL_INTERFACE, 3, server_data.clone());

    // get the registry client-side
    let client_display = client.display_id();
    let registry_id = client
        .send_request(
            message!(client_display, 1, [Argument::NewId(client_backend::ObjectId::null())],),
            Some(Arc::new(DoNothingData)),
            Some((&interfaces::WL_REGISTRY_INTERFACE, 1)),
        )
        .unwrap();
    // create the test global
    let test_global_id = client
        .send_request(
            message!(
                registry_id,
                0,
                [
                    Argument::Uint(1),
                    Argument::Str(Some(Box::new(
                        CString::new(interfaces::TEST_GLOBAL_INTERFACE.name.as_bytes()).unwrap(),
                    ))),
                    Argument::Uint(3),
                    Argument::NewId(client_backend::ObjectId::null()),
                ],
            ),
            Some(client_data.clone()),
            Some((&interfaces::TEST_GLOBAL_INTERFACE, 3)),
        )
        .unwrap();

    client.flush().unwrap();
    server.dispatch_all_clients(&mut ()).unwrap();
    server.flush(None).unwrap();
    client.prepare_read().unwrap().read().unwrap();
    client.destroy_object(&test_global_id).unwrap();
    assert_eq!(client.destroy_object(&test_global_id), Err(InvalidId));
});

expand_test!(destroy_flush_destroy, {
    let (tx, rx) = std::os::unix::net::UnixStream::pair().unwrap();
    let mut server = server_backend::Backend::new().unwrap();
    let _client_id = server.handle().insert_client(rx, Arc::new(())).unwrap();
    let client = client_backend::Backend::connect(tx).unwrap();

    let server_data = Arc::new(ServerData(()));
    let client_data = Arc::new(ClientData(()));

    // Prepare a global
    server.handle().create_global(&interfaces::TEST_GLOBAL_INTERFACE, 3, server_data.clone());

    // get the registry client-side
    let client_display = client.display_id();
    let registry_id = client
        .send_request(
            message!(client_display, 1, [Argument::NewId(client_backend::ObjectId::null())],),
            Some(Arc::new(DoNothingData)),
            Some((&interfaces::WL_REGISTRY_INTERFACE, 1)),
        )
        .unwrap();
    // create the test global
    let test_global_id = client
        .send_request(
            message!(
                registry_id,
                0,
                [
                    Argument::Uint(1),
                    Argument::Str(Some(Box::new(
                        CString::new(interfaces::TEST_GLOBAL_INTERFACE.name.as_bytes()).unwrap(),
                    ))),
                    Argument::Uint(3),
                    Argument::NewId(client_backend::ObjectId::null()),
                ],
            ),
            Some(client_data.clone()),
            Some((&interfaces::TEST_GLOBAL_INTERFACE, 3)),
        )
        .unwrap();

    client.flush().unwrap();
    server.dispatch_all_clients(&mut ()).unwrap();
    server.flush(None).unwrap();
    client.prepare_read().unwrap().read().unwrap();

    client.destroy_object(&test_global_id).unwrap();

    client.flush().unwrap();
    server.dispatch_all_clients(&mut ()).unwrap();
    server.flush(None).unwrap();
    let _ = client.prepare_read().unwrap().read();

    assert_eq!(client.destroy_object(&test_global_id), Err(InvalidId));
});

expand_test!(destroy_then_message, {
    let (tx, rx) = std::os::unix::net::UnixStream::pair().unwrap();
    let mut server = server_backend::Backend::new().unwrap();
    let _client_id = server.handle().insert_client(rx, Arc::new(())).unwrap();
    let client = client_backend::Backend::connect(tx).unwrap();

    let server_data = Arc::new(ServerData(()));
    let client_data = Arc::new(ClientData(()));

    // Prepare a global
    server.handle().create_global(&interfaces::TEST_GLOBAL_INTERFACE, 3, server_data.clone());

    // get the registry client-side
    let client_display = client.display_id();
    let registry_id = client
        .send_request(
            message!(client_display, 1, [Argument::NewId(client_backend::ObjectId::null())],),
            Some(Arc::new(DoNothingData)),
            Some((&interfaces::WL_REGISTRY_INTERFACE, 1)),
        )
        .unwrap();
    // create the test global
    let test_global_id = client
        .send_request(
            message!(
                registry_id,
                0,
                [
                    Argument::Uint(1),
                    Argument::Str(Some(Box::new(
                        CString::new(interfaces::TEST_GLOBAL_INTERFACE.name.as_bytes()).unwrap(),
                    ))),
                    Argument::Uint(3),
                    Argument::NewId(client_backend::ObjectId::null()),
                ],
            ),
            Some(client_data.clone()),
            Some((&interfaces::TEST_GLOBAL_INTERFACE, 3)),
        )
        .unwrap();

    client.flush().unwrap();
    server.dispatch_all_clients(&mut ()).unwrap();
    server.flush(None).unwrap();
    client.prepare_read().unwrap().read().unwrap();

    client.destroy_object(&test_global_id).unwrap();

    client.flush().unwrap();
    server.dispatch_all_clients(&mut ()).unwrap();
    server.flush(None).unwrap();
    let _ = client.prepare_read().unwrap().read();

    assert_eq!(
        client.send_request(
            message!(
                test_global_id.clone(),
                2,
                [Argument::Object(client_backend::ObjectId::null()), Argument::Uint(1),],
            ),
            None,
            None,
        ),
        Err(InvalidId)
    );
});
