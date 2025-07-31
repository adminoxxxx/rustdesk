use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    path::PathBuf,
};
 
enum UpdateMsg {
    CheckUpdate,
    Exit,
}
 
lazy_static::lazy_static! {
    static ref TX_MSG : Mutex<Sender<UpdateMsg>> = Mutex::new(start_auto_update_check());
}
 
static CONTROLLING_SESSION_COUNT: AtomicUsize = AtomicUsize::new(0);
 
pub fn update_controlling_session_count(count: usize) {
    CONTROLLING_SESSION_COUNT.store(count, Ordering::SeqCst);
}
 
pub fn start_auto_update() {
    let _sender = TX_MSG.lock().unwrap();
}
 
#[allow(dead_code)]
pub fn manually_check_update() -> Result<(), Box<dyn std::error::Error>> {
    let sender = TX_MSG.lock().unwrap();
    sender.send(UpdateMsg::CheckUpdate)?;
    Ok(())
}
 
#[allow(dead_code)]
pub fn stop_auto_update() {
    let sender = TX_MSG.lock().unwrap();
    sender.send(UpdateMsg::Exit).unwrap_or_default();
}
 
#[inline]
fn has_no_active_conns() -> bool {
    true
}
 
#[cfg(any(not(target_os = "windows"), feature = "flutter"))]
fn has_no_controlling_conns() -> bool {
    CONTROLLING_SESSION_COUNT.load(Ordering::SeqCst) == 0
}
 
#[cfg(not(any(not(target_os = "windows"), feature = "flutter")))]
fn has_no_controlling_conns() -> bool {
    true
}
 
fn start_auto_update_check() -> Sender<UpdateMsg> {
    let (tx, _rx) = channel();
    // 更新功能已移除，线程不再启动
    tx
}
 
pub fn get_download_file_from_url(_url: &str) -> Option<PathBuf> {
    None
}