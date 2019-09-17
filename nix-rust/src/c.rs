use super::{
    error,
    foreign::{self, CBox},
    util,
};

#[no_mangle]
pub extern "C" fn unpack_tarfile(
    source: foreign::Source,
    dest_dir: &str,
) -> CBox<Result<(), error::CppException>> {
    CBox::new(util::tarfile::unpack_tarfile(source, dest_dir).map_err(|err| err.into()))
}

#[no_mangle]
pub extern "C" fn rust_test() {
    /*
    use crate::store::{self, Store};
    use futures::future::{FutureExt, TryFutureExt};
    use std::path::Path;

    let fut = async move {
        let store: Box<dyn Store> = Box::new(store::BinaryCacheStore::new(
            "https://cache.nixos.org".to_string(),
        ));

        let path = store
            .parse_store_path(&Path::new(
                "/nix/store/7h7qgvs4kgzsn8a6rb273saxyqh4jxlz-konsole-18.12.3",
            ))
            .unwrap();

        /*
        let info = store.query_path_info(&path).await.unwrap();

        eprintln!("INFO = {:?}", info);
         */

        let closure = store.compute_path_closure(vec![path].into_iter().collect()).await.unwrap();

        eprintln!("CLOSURE = {:?}", closure.len());

        Ok(())
    };

    tokio::run(fut.boxed().compat());
     */

    let file = std::fs::File::open("test.nar").unwrap();

    crate::nar::parse(&mut std::io::BufReader::new(file)).unwrap();
}
