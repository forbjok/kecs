pub mod cli;
pub mod fancy;

pub trait UiHandler {
    fn begin_generate(&mut self, file_count: u32, total_size: u64);
    fn end_generate(&mut self);

    fn begin_verify(&mut self, file_count: u32, total_size: u64);
    fn end_verify(&mut self);

    fn begin_file(&mut self, filename: &str, size: u64);
    fn file_progress(&mut self, bytes: u64);
    fn end_file(&mut self);
}