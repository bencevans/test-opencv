fn main() {
    let cuda_count = opencv::core::get_cuda_enabled_device_count().unwrap();
    println!("CUDA enabled device count: {}", cuda_count);
}
