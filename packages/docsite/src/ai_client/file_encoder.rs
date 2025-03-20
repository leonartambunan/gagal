// use ai_client::ask_ai;
// use base64::prelude::BASE64_STANDARD_NO_PAD;
// use base64::Engine;

// pub async fn encode_document(doc_type: &str, blob: Blob)  -> String {
//     let file_raw_data: JsValue = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
//         .await
//         .expect("File reading should not fail");

//     let file_raw_data = file_raw_data
//         .dyn_into::<ArrayBuffer>()
//         .expect("Expected an ArrayBuffer");
//     let file_raw_data = Uint8Array::new(&file_raw_data);

//     let mut file_bytes = vec![0; file_raw_data.length() as usize];
//     file_raw_data.copy_to(file_bytes.as_mut_slice());


//     warn!("len: {}",file_bytes.len());


//     let x = BASE64_STANDARD_NO_PAD.encode(file_bytes);

//     warn!("x: {}",x);

//     //ask_ai(doc_type, x).await
//     x
// }