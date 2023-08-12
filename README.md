# Error on Route

Aug 2023: Leptos-use websocket seems to not be cleaning up properly

## How to run

Start websocket server:

```
# from server
cargo run
```

Start frontend and open:

```
# from frontend
trunk serve --open
```

## How to see error

Click `Start websocket` and verify it worked by looking for `Connection established` on page

Click `Home` to return to the `/` route

See error in console:
```
Uncaught RuntimeError: unreachable
    at __rust_start_panic
    at rust_panic
    at std::panicking::rust_panic_with_hook
    at std::panicking::begin_panic_handler::{{closure}}
    at std::sys_common::backtrace::__rust_end_short_backtrace
    at rust_begin_unwind
    at core::panicking::panic_fmt
    at core::option::expect_failed
    at core::option::Option<T>::expect
    at leptos_reactive::stored_value::StoredValue<T>::get_value
```
