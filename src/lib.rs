mod helpers;
mod runner;

#[cfg(target_arch = "wasm32")]
pub mod my_wasm_module {

    const KERNEL: &[u8] = include_bytes!(env!("kernel.spv"));

    use crate::helpers;
    use crate::runner;

    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub async fn hash(name: &str, ty: bool) -> String {
        let mut device = runner::Device::new().await;

        let words = vec![name.to_string()];
        let count = words.len();

        let (texts, sizes): (Vec<Vec<u32>>, Vec<u32>) = if ty {
            words
                .into_iter()
                .map(|x| helpers::prepare_for_gpu(x))
                .unzip()
        } else {
            words
                .into_iter()
                .map(|x| helpers::prepare_for_gpu_u8(hex::decode(x).expect("Input was not hex")))
                .unzip()
        };

        let texts: Vec<u32> = texts.into_iter().flatten().collect();
        let hash = vec![0u32; count * 8];

        web_sys::console::log_1(&"Sending data to GPU...".into());

        let text_gpu = device.to_device(texts.as_slice());
        let hash_gpu = device.to_device(hash.as_slice());
        let size_gpu = device.to_device(sizes.as_slice());

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::util::make_spirv(KERNEL),
        };

        web_sys::console::log_1(&"Loaded shader and now running...".into());

        let args_gpu = runner::ParamsBuilder::new()
            .param(Some(&text_gpu), true)
            .param(Some(&hash_gpu), false)
            .param(Some(&size_gpu), true)
            .build(Some(0));
        web_sys::console::log_1(&"Loading compute...".into());

        let compute = device.compile("main_cs", shader, &args_gpu.0).unwrap();
        device.call(compute, (count as u32, 1, 1), &args_gpu.1);

        web_sys::console::log_1(&"Running...".into());

        let hash_res = device.get(&hash_gpu).await.unwrap();
        let hash_res = &hash_res[0..hash.len()];

        web_sys::console::log_1(&"Retrieving...".into());

        let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
        let chunks = result
            .as_bytes()
            .chunks(64)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        for c in &chunks {
            web_sys::console::log_1(&format!("{}", c).into());
        }

        let d_info = &device.info.as_ref().unwrap().info;

        let d_name = &d_info.name;
        let d_type = &d_info.device_type;
        let d_driver = &d_info.driver;
        let d_driver_info = &d_info.driver_info;
        let d_backend = &d_info.backend;

        web_sys::console::log_1(
            &format!(
                "{}\n  Type: {:?}\n  Driver: {} ({})\n  Backend: {:?}",
                d_name, d_type, d_driver, d_driver_info, d_backend
            )
            .into(),
        );
        return chunks.first().expect("No input?").to_string();
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
    pub fn run() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
    }
}
