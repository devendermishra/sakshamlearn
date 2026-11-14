use dioxus::prelude::*;

pub fn apply(lock_on: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        if lock_on {
            spawn(async move {
                let _ = document::eval(
                    r#"if (document.documentElement.requestFullscreen) {
                        document.documentElement.requestFullscreen();
                    }"#,
                )
                .await;
            });
        }
    }
    #[cfg(target_os = "android")]
    android_set(lock_on);
}

#[cfg(target_os = "android")]
fn android_set(lock: bool) {
    let ctx = ndk_context::android_context();
    let vm_ptr = ctx.vm() as *mut jni::sys::JavaVM;
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(vm_ptr) }) else {
        return;
    };
    let vm = std::mem::ManuallyDrop::new(vm);
    let Ok(mut guard) = vm.attach_current_thread() else {
        return;
    };
    let activity_ptr = ctx.context() as jni::sys::jobject;
    if !activity_ptr.is_null() {
        let activity = unsafe { jni::objects::JObject::from_raw(activity_ptr) };
        let method = if lock {
            "startLockTask"
        } else {
            "stopLockTask"
        };
        let _ = guard.call_method(&activity, method, "()V", &[]);
        if guard.exception_check().unwrap_or(false) {
            let _ = guard.exception_clear();
        }
        std::mem::forget(activity);
    }
    drop(guard);
}
