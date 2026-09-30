"""Derleme çeşidine göre gömülü yapılandırmayı hazırlar (CI'da, secret'tan üretilen config.json üzerinde).

musteri   : olduğu gibi (sadece gelen bağlantı)
teknisyen : ayrı uygulama adı + sadece giden bağlantı; gelen bağlantıya ait ayarlar çıkarılır

Kullanım: variant.py <config.json> <musteri|teknisyen> <uygulama-adı>
Değerler ekrana basılmaz.
"""

import json
import sys

INCOMING_ONLY = ("approve-mode", "verification-method", "uy-temp-password-requires-click")


def main() -> None:
    path, variant, app_name = sys.argv[1:4]
    with open(path, encoding="utf-8") as f:
        config = json.load(f)
    if variant == "teknisyen":
        config["app-name"] = app_name
        config["conn-type"] = "outgoing"
        overrides = config.setdefault("override-settings", {})
        for key in INCOMING_ONLY:
            overrides.pop(key, None)
    elif variant != "musteri":
        sys.exit(f"bilinmeyen çeşit: {variant}")
    if config.get("app-name") != app_name:
        sys.exit(f"app-name '{app_name}' olmalı")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(config, f, ensure_ascii=False, indent=2)
    print(f"{variant}: app-name={app_name} conn-type={config.get('conn-type')}")


if __name__ == "__main__":
    main()
