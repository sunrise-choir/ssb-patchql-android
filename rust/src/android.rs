use crate::common::*;
use ssb_patchql_core::Patchql;
use std::sync::mpsc;
use std::thread;

// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping this context
// and getting used after being GC'd.
use jni::objects::{JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function.
// We can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jlong;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_com_sunrisechoir_rnpatchql_Patchql_initLibrary(_env: JNIEnv, _: JObject) {
    hide_exceptions();
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_sunrisechoir_rnpatchql_Patchql_patchqlNew(
    env: JNIEnv,
    _class: JClass,
    offset_log_path: JString,
    database_path: JString,
    pub_key: JString,
    secret_key: JString,
) -> jlong {
    let res = handle_exception(|| {
        let offset_log_path_string: String = env
            .get_string(offset_log_path)
            .expect("Couldn't get java string!")
            .into();
        let database_path_string: String = env
            .get_string(database_path)
            .expect("Couldn't get java string!")
            .into();
        let pub_key_string: String = env
            .get_string(pub_key)
            .expect("Couldn't get java string!")
            .into();
        let secret_key_string: String = env
            .get_string(secret_key)
            .expect("Couldn't get java string!")
            .into();
        let patchql = Patchql::new(
            offset_log_path_string,
            database_path_string,
            pub_key_string,
            secret_key_string,
        );

        Box::into_raw(Box::new(patchql)) as jlong
    });

    match res {
        Ok(res) => res,
        Err(err) => {
            env.throw(err).expect("unable to throw error");
            0
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_sunrisechoir_rnpatchql_Patchql_patchqlDestroy(
    _env: JNIEnv,
    _class: JClass,
    context_ptr: jlong,
) {
    let _boxed_counter = Box::from_raw(context_ptr as *mut Patchql);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_sunrisechoir_rnpatchql_Patchql_patchqlQueryAsync(
    env: JNIEnv,
    _class: JClass,
    context_ptr: jlong,
    query: JString,
    callback: JObject,
) {
    let query_string: String = env
        .get_string(query)
        .expect("Couldn't get java string!")
        .into();

    // We need to cast the raw pointer as in instance of patchql.
    let patchql = (&mut *(context_ptr as *mut Patchql)).clone();

    // `JNIEnv` cannot be sent across thread boundaries. To be able to use JNI
    // functions in other threads, we must first obtain the `JavaVM` interface
    // which, unlike `JNIEnv` is `Send`.
    let jvm = env.get_java_vm().expect("unable to get the javaVM object");

    // We need to obtain global reference to the `callback` object before sending
    // it to the thread, to prevent it from being collected by the GC.
    let callback_global = env
        .new_global_ref(callback)
        .expect("unable to make a global ref to callback");

    // Use channel to prevent the Java program to finish before the thread
    // has chance to start.
    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        // Signal that the thread has started.
        tx.send(()).unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let callback = callback_global.as_obj();

        // Use the `JavaVM` interface to attach a `JNIEnv` to the current thread.
        let response_string = patchql
            .query(&query_string)
            .expect("unable to query patchql");

        let response_java_string = JString::from(
            env.new_string(response_string)
                .expect("unable to create java string from rust string"),
        );

        let result_or_err: (JObject, JObject) = (JObject::null().into(), response_java_string.into());
        // Fiiinnnally, callback.
        env.call_method(
            callback,
            "invoke",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[result_or_err.0.into(), result_or_err.1.into()],
        )
        .unwrap();
    });

    // Wait until the thread has started.
    rx.recv().unwrap();
}
