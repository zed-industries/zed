#![allow(unused_imports, dead_code)]

pub mod common;

use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, entity::*, DatabaseConnection};
use std::net::{Ipv4Addr, Ipv6Addr};

#[sea_orm_macros::test]
#[cfg(feature = "sqlx-postgres")]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("host_network_tests").await;
    create_tables(&ctx.db).await?;
    create_and_update_host_network(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}

async fn create_and_update_host_network(db: &DatabaseConnection) -> Result<(), DbErr> {
    let addr = IpNetwork::new(Ipv4Addr::new(192, 168, 0, 20).into(), 24).unwrap();
    let net = IpNetwork::new(addr.network(), addr.prefix()).unwrap();

    let host = host_network::Model {
        id: 1,
        hostname: "example.com".to_owned(),
        ipaddress: addr,
        network: net,
    };
    let res = host.clone().into_active_model().insert(db).await?;

    let model = HostNetwork::find().one(db).await?.unwrap();
    assert_eq!(model, res);
    assert_eq!(model, host.clone());

    let addrv6 = IpNetwork::new(
        Ipv6Addr::new(0xfd89, 0x1926, 0x4cae, 0x8abd, 0, 0, 0, 0x6f52).into(),
        64,
    )
    .unwrap();
    let netv6 = IpNetwork::new(addr.network(), addr.prefix()).unwrap();

    let res = host_network::ActiveModel {
        id: Set(1),
        ipaddress: Set(addrv6),
        network: Set(netv6),
        ..Default::default()
    }
    .update(db)
    .await?;

    assert_eq!(
        res,
        host_network::Model {
            id: 1,
            hostname: "example.com".to_owned(),
            ipaddress: addrv6,
            network: netv6,
        }
    );

    Ok(())
}
