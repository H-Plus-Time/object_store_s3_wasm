use object_store::ObjectStore;
use object_store_s3_wasm::S3;
use std::sync::Arc;
use wasm_bindgen_test::*;

use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn it_works() {
    let s3: Arc<dyn ObjectStore> = Arc::new(
        S3::builder()
            .endpoint("http://localhost:9000")
            .region("us-east-1")
            .bucket("test")
            .access_key_id("XDA5RzlTD1xCPwKZ9DH1")
            .secret_access_key("dTYsD9MMf7EojsGZkapANv1bUh9zKLC0jZ4iKaoU")
            .build()
            .expect("Failed to create s3 client"),
    );

    s3.list(None).await.expect("Failed to list objects");
}
