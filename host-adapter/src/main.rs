#![feature(impl_trait_in_fn_trait_return)]
#![feature(unboxed_closures)]

use std::{sync::{Arc, Mutex}, collections::VecDeque};

use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    ConnectionProperties, Channel, Consumer, Connection,
};
use log::info;
use once_cell::sync::Lazy;

const DEVICE_ID: &str = "DF:89:2B:DA:0B:CB";
const SERVER_UUID: &str = "0000dad0-0000-0000-0000-000000000000";
const BUTTON_UUID: &str = "0000dad0-0001-0000-0000-000000000000";
const ACCELEROMETER_X_UUID: &str = "0000dad0-0002-0000-0000-000000000000";
const ACCELEROMETER_Y_UUID: &str = "0000dad0-0002-0000-0000-000000000001";
const ACCELEROMETER_Z_UUID: &str = "0000dad0-0002-0000-0000-000000000002";
const GYROSCOPE_X_UUID: &str = "0000dad0-0003-0000-0000-000000000000";
const GYROSCOPE_Y_UUID: &str = "0000dad0-0003-0000-0000-000000000001";
const GYROSCOPE_Z_UUID: &str = "0000dad0-0003-0000-0000-000000000002";

const WINDOWS_SIZE: usize = 100;
static BUTTON_STATE: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));
static ACCELEROMETER_X_STATE: Lazy<Arc<Mutex<VecDeque<f32>>>> = Lazy::new(|| Arc::new(Mutex::new((0..WINDOWS_SIZE).map(|_| 0.0 ).collect())));
static ACCELEROMETER_Y_STATE: Lazy<Arc<Mutex<VecDeque<f32>>>> = Lazy::new(|| Arc::new(Mutex::new((0..WINDOWS_SIZE).map(|_| 0.0 ).collect())));
static ACCELEROMETER_Z_STATE: Lazy<Arc<Mutex<VecDeque<f32>>>> = Lazy::new(|| Arc::new(Mutex::new((0..WINDOWS_SIZE).map(|_| 0.0 ).collect())));
static GYROSCOPE_X_STATE: Lazy<Arc<Mutex<VecDeque<f32>>>> = Lazy::new(|| Arc::new(Mutex::new((0..WINDOWS_SIZE).map(|_| 0.0 ).collect())));
static GYROSCOPE_Y_STATE: Lazy<Arc<Mutex<VecDeque<f32>>>> = Lazy::new(|| Arc::new(Mutex::new((0..WINDOWS_SIZE).map(|_| 0.0 ).collect())));
static GYROSCOPE_Z_STATE: Lazy<Arc<Mutex<VecDeque<f32>>>> = Lazy::new(|| Arc::new(Mutex::new((0..WINDOWS_SIZE).map(|_| 0.0 ).collect())));

fn push_value(data_windows: &Arc<Mutex<VecDeque<f32>>>, value: f32) {
    let mut data_windows = data_windows.lock().unwrap();
    if data_windows.len() == WINDOWS_SIZE {
        data_windows.pop_back();
    }
    data_windows.push_front(value);
}

async fn create_consumer(channel: &Channel, characterist_uuid: &str) -> Result<Consumer, lapin::Error> {
    let queue_name = format!("{DEVICE_ID}/{SERVER_UUID}/{characterist_uuid}");
    channel.basic_consume(
        queue_name.as_str(),
        queue_name.as_str(),
        BasicConsumeOptions::default(),
        FieldTable::default(),
    ).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let uri = "amqp://user:password@localhost:5672/%2f";
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(uri, options).await.unwrap();
    let channel = connection.create_channel().await.unwrap();

    create_consumer(&channel, BUTTON_UUID).await.map(|consumer| {
        consumer.set_delegate(move |delivery: DeliveryResult| async {
            let delivery = match delivery {
                Err(_) | Ok(None) => return,
                Ok(Some(delivery)) => delivery,
            };

            let value: bool = delivery.data[0] != 0;
            *BUTTON_STATE.lock().unwrap() = value;
            info!("Received value {:?}", value);

            delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack send_webhook_event message");
        });
    })?;

    create_consumer(&channel, ACCELEROMETER_X_UUID).await.map(|consumer| {
        consumer.set_delegate(move |delivery: DeliveryResult| async {
            let delivery = match delivery {
                Err(_) | Ok(None) => return,
                Ok(Some(delivery)) => delivery,
            };

            let value = f32::from_le_bytes(delivery.data[..].try_into().unwrap());
            push_value(&ACCELEROMETER_X_STATE, value);
            info!("Received value {:?}", value);

            delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack send_webhook_event message");
        });
    })?;

    create_consumer(&channel, ACCELEROMETER_Y_UUID).await.map(|consumer| {
        consumer.set_delegate(move |delivery: DeliveryResult| async {
            let delivery = match delivery {
                Err(_) | Ok(None) => return,
                Ok(Some(delivery)) => delivery,
            };

            let value = f32::from_le_bytes(delivery.data[..].try_into().unwrap());
            push_value(&ACCELEROMETER_Y_STATE, value);
            info!("Received value {:?}", value);

            delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack send_webhook_event message");
        });
    })?;

    create_consumer(&channel, ACCELEROMETER_Z_UUID).await.map(|consumer| {
        consumer.set_delegate(move |delivery: DeliveryResult| async {
            let delivery = match delivery {
                Err(_) | Ok(None) => return,
                Ok(Some(delivery)) => delivery,
            };

            let value = f32::from_le_bytes(delivery.data[..].try_into().unwrap());
            push_value(&ACCELEROMETER_Z_STATE, value);
            info!("Received value {:?}", value);

            delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack send_webhook_event message");
        });
    })?;

    create_consumer(&channel, GYROSCOPE_X_UUID).await.map(|consumer| {
        consumer.set_delegate(move |delivery: DeliveryResult| async {
            let delivery = match delivery {
                Err(_) | Ok(None) => return,
                Ok(Some(delivery)) => delivery,
            };

            let value = f32::from_le_bytes(delivery.data[..].try_into().unwrap());
            push_value(&GYROSCOPE_X_STATE, value);
            info!("Received value {:?}", value);

            delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack send_webhook_event message");
        });
    })?;

    create_consumer(&channel, GYROSCOPE_Y_UUID).await.map(|consumer| {
        consumer.set_delegate(move |delivery: DeliveryResult| async {
            let delivery = match delivery {
                Err(_) | Ok(None) => return,
                Ok(Some(delivery)) => delivery,
            };

            let value = f32::from_le_bytes(delivery.data[..].try_into().unwrap());
            push_value(&GYROSCOPE_Y_STATE, value);
            info!("Received value {:?}", value);

            delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack send_webhook_event message");
        });
    })?;

    create_consumer(&channel, GYROSCOPE_Z_UUID).await.map(|consumer| {
        consumer.set_delegate(move |delivery: DeliveryResult| async {
            let delivery = match delivery {
                Err(_) | Ok(None) => return,
                Ok(Some(delivery)) => delivery,
            };

            let value = f32::from_le_bytes(delivery.data[..].try_into().unwrap());
            push_value(&GYROSCOPE_Z_STATE, value);
            info!("Received value {:?}", value);

            delivery.ack(BasicAckOptions::default()).await.expect("Failed to ack send_webhook_event message");
        });
    })?;




    std::future::pending::<()>().await;

    Ok(())
}
