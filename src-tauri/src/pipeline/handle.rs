#[derive(Debug, Default)]
pub struct Handles {
    handles: Vec<tauri::async_runtime::JoinHandle<miette::Result<()>>>,
}

impl Handles {
    pub fn push(&mut self, handle: tauri::async_runtime::JoinHandle<miette::Result<()>>) {
        self.handles.push(handle);
    }
}

impl Drop for Handles {
    fn drop(&mut self) {
        for handle in self.handles.drain(..) {
            handle.abort();
        }
    }
}
