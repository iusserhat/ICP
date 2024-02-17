use candid::{CandidType, Deserialize};
use std::include_bytes;

mod core;

const IMAGE_SIZE_IN_PIXELS: usize = 1024;
const LOGO_TRANSPARENT: &[u8] = include_bytes!("../assets/logo_transparent.png");
const LOGO_WHITE: &[u8] = include_bytes!("../assets/logo_white.png");

#[derive(CandidType, Deserialize)]
struct Options {
    add_logo: bool,
    add_gradient: bool,
    add_transparency: Option<bool>,
}

#[derive(CandidType, Deserialize)]
struct QrError {
    // Hata türünü tanımladık
    code: u16,
    message: String,
}

#[derive(CandidType, Deserialize)]
enum QrResult {
    Image(Vec<u8>),
    Err(QrError),
}

// From trait ile hataları qrerror'a dönüştürdük
impl From<core::QrError> for QrError {
    fn from(error: core::QrError) -> Self {
        match error {
            core::QrError::InvalidInput => QrError {
                code: 100,
                message: "Geçersiz girdi".to_string(),
            },
            core::QrError::EncodingError => QrError {
                code: 101,
                message: "Kodlama hatası".to_string(),
            },
            core::QrError::ImageGenerationError => QrError {
                code: 102,
                message: "Görüntü oluşturma hatası".to_string(),
            },
        }
    }
}

// güncellenmiş impl bloğu, hataları daha etkili bir şekilde ele aldı
fn qrcode_impl(input: String, options: Options) -> QrResult {
    let logo = if options.add_transparency == Some(true) {
        LOGO_TRANSPARENT
    } else {
        LOGO_WHITE
    };

    //  Giriş bilgilerini ve seçenekleri loglama
    ic_cdk::println!(
        "Executing QR code generation. Input length: {}, Add logo: {}, Add gradient: {}, Add transparency: {:?}",
        input.len(),
        options.add_logo,
        options.add_gradient,
        options.add_transparency
    );

    let result = match core::generate(input, options, logo, IMAGE_SIZE_IN_PIXELS) {
        Ok(blob) => {
            // Başarılı durumda performans bilgilerini ve başarılı logunu kaydetme
            ic_cdk::println!(
                "QR code generation successful. Executed instructions: {}",
                ic_cdk::api::performance_counter(0)
            );
            QrResult::Image(blob)
        }
        Err(err) => {
            // Y Hata durumunda performans bilgilerini ve hata logunu kaydettik
            ic_cdk::println!(
                "QR code generation failed. Executed instructions: {}",
                ic_cdk::api::performance_counter(0)
            );
            QrResult::Err(err.into())
        }
    };

    result
}

//  Kodun içeriğini query fonksiyonuna da taşıdık
#[ic_cdk::update]
fn qrcode(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}

#[ic_cdk::query]
fn qrcode_query(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}
