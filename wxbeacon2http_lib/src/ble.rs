use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, PeripheralId};
use futures::stream::StreamExt;
use std::error::Error;

use crate::{models::WxBroadcastFormat, SharedState, WX_COMPANY_ID, WX_LOCAL_NAME};

pub async fn run_ble_scan(state: SharedState) -> Result<(), Box<dyn Error>> {
    let central = get_central().await?;
    let mut events = central.events().await?;
    central.start_scan(ScanFilter::default()).await?;

    let mut latest_seq_no: u8 = 0;

    while let Some(event) = events.next().await {
        if let CentralEvent::ManufacturerDataAdvertisement { id, manufacturer_data } = event {
            if let Some(data) = manufacturer_data.get(&WX_COMPANY_ID) {
                if is_wx(&central, &id).await {
                    process_manufacturer_data(data, &mut latest_seq_no, &state).await?;
                }
            }
        }
    }

    Ok(())
}

async fn get_central() -> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    adapters.into_iter().next().ok_or_else(|| "No Bluetooth adapters found".into())
}

async fn is_wx(central: &Adapter, id: &PeripheralId) -> bool {
    if let Ok(peripherals) = central.peripherals().await {
        for p in peripherals {
            if let Ok(Some(properties)) = p.properties().await {
                if properties.local_name.iter().any(|name| name == WX_LOCAL_NAME) && p.id().eq(id) {
                    return true;
                }
            }
        }
    }
    false
}

async fn process_manufacturer_data(
    data: &[u8],
    latest_seq_no: &mut u8,
    state: &SharedState,
) -> Result<(), Box<dyn Error>> {
    let wx_data: WxBroadcastFormat = unsafe { std::ptr::read(data.as_ptr() as *const _) };
    let seq_no = wx_data.seq_no;

    if seq_no != *latest_seq_no {
        *latest_seq_no = seq_no;
        let new_datum = WxBroadcastFormat::parse_env_datum(&wx_data);
        let mut state = state.write().await;
        *state = Some(new_datum.clone());
        println!("New data: {:?}", new_datum);
    }

    Ok(())
}