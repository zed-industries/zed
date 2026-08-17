/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// IMDSv2 client usage example
///
/// The IMDS client is used with `aws-config` to load credentials and regions, however, you can also
/// use the client directly. This example demonstrates loading the instance-id from IMDS. More
/// fetures of IMDS can be found [here](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use aws_config::imds::Client;

    let imds = Client::builder().build();
    let instance_id = imds.get("/latest/meta-data/instance-id").await?;
    println!("current instance id: {}", instance_id.as_ref());
    Ok(())
}
