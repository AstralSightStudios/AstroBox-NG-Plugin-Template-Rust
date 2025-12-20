use wit_bindgen::FutureReader;

use crate::{
    astrobox::psys_host::{
        self,
        dialog::{DialogButton, DialogInfo},
    },
    exports::astrobox::psys_plugin::{
        event::{self, EventType},
        lifecycle,
    },
};

pub mod logger;

wit_bindgen::generate!({
    path: "wit",
    world: "psys-world",
    generate_all,
});

struct MyPlugin;

impl event::Guest for MyPlugin {
    #[allow(async_fn_in_trait)]
    fn on_event(event_type: EventType, event_payload: _rt::String) -> FutureReader<String> {
        let (writer, reader) = wit_future::new::<String>(|| "".to_string());

        match event_type {
            EventType::PluginMessage => {}
            EventType::InterconnectMessage => {}
            EventType::DeviceAction => {}
            EventType::ProviderAction => {}
            EventType::DeeplinkAction => {}
            EventType::TransportPacket => {}
        };

        tracing::info!("event_payload: {}", event_payload);

        wit_bindgen::spawn(async move {
            let _ = writer.write("".to_string()).await;
        });

        reader
    }

    fn on_ui_event(
        _event_id: _rt::String,
        _event: event::Event,
        _event_payload: _rt::String,
    ) -> wit_bindgen::rt::async_support::FutureReader<_rt::String> {
        let (writer, reader) = wit_future::new::<String>(|| "".to_string());

        wit_bindgen::spawn(async move {
            let _ = writer.write("".to_string()).await;
        });

        reader
    }
}

impl lifecycle::Guest for MyPlugin {
    #[allow(async_fn_in_trait)]
    fn on_load() -> () {
        logger::init();

        tracing::info!("Hello AstroBox V2 Plugin!");

        wit_bindgen::block_on(async {
            let _ret = psys_host::dialog::show_dialog(
                psys_host::dialog::DialogType::Alert,
                psys_host::dialog::DialogStyle::System,
                &DialogInfo {
                    title: "Plugin Alert".to_string(),
                    content: "该插件正在AstroBox V2的全新WASI插件系统上运行！".to_string(),
                    buttons: vec![DialogButton {
                        id: "1".to_string(),
                        primary: true,
                        content: "OK".to_string(),
                    }],
                },
            )
            .await;
        });
    }
}

export!(MyPlugin);
