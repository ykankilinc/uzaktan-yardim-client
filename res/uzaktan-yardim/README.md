# Uzaktan Yardım yapılandırması

`config.json` derlemeye gömülür (`src/common.rs` → `BUILTIN_CUSTOM_CLIENT`) ama **repoda tutulmaz**.

- Şablon: `config.example.json`
- CI: `UY_CLIENT_CONFIG_B64` secret'ından (base64 JSON) derleme sırasında üretilir.
- Yerel derleme: şablonu `config.json` olarak kopyalayıp değerleri doldur.
